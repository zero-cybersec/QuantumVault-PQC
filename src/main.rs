mod crypto;
mod ffi;

use crate::crypto::{CryptoProvider, FfiCryptoProvider};
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use fuser::{
    Errno, FileAttr, FileHandle, FileType, Filesystem, FopenFlags, INodeNo, LockOwner, MountOption,
    OpenFlags, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEntry, ReplyOpen,
    ReplyWrite, Request, TimeOrNow, WriteFlags,
};
const TTL: Duration = Duration::from_secs(1);
const ROOT_INODE: u64 = 1;

static STORAGE_COUNTER: AtomicU64 = AtomicU64::new(0);

struct VirtualFile {
    name: String,
    // Encrypted bytes are stored here, never plaintext.
    data: Vec<u8>,
    plaintext_size: usize,
    mode: u16,
    uid: u32,
    gid: u32,
    created: SystemTime,
    modified: SystemTime,
    accessed: SystemTime,
}

struct FsState {
    files: BTreeMap<u64, VirtualFile>,
    next_inode: u64,
    crypto: FfiCryptoProvider,
    storage_dir: PathBuf,
}

impl FsState {
    fn new() -> Self {
        let id = STORAGE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let storage_dir =
            std::env::temp_dir().join(format!("quantumvault-pqc-{}-{}", std::process::id(), id));

        fs::create_dir_all(&storage_dir).expect("Failed to create QuantumVault backing storage");

        println!("QuantumVault backing storage: {}", storage_dir.display());

        Self {
            files: BTreeMap::new(),
            next_inode: 2,
            crypto: FfiCryptoProvider::new()
                .expect("Failed to initialize QuantumVault crypto backend"),
            storage_dir,
        }
    }

    fn persist_ciphertext(&self, ino: u64, ciphertext: &[u8]) -> Result<(), Errno> {
        let destination = self.storage_dir.join(format!("{ino}.qv"));
        let temporary = self.storage_dir.join(format!(
            ".{ino}.{}.tmp",
            STORAGE_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));

        let result = (|| {
            let mut file = File::create(&temporary).map_err(|_| Errno::EIO)?;
            file.write_all(ciphertext).map_err(|_| Errno::EIO)?;
            file.sync_all().map_err(|_| Errno::EIO)?;
            fs::rename(&temporary, &destination).map_err(|_| Errno::EIO)?;
            Ok(())
        })();

        if result.is_err() {
            let _ = fs::remove_file(&temporary);
        }

        result
    }

    fn create_file(&mut self, name: &str, mode: u16, uid: u32, gid: u32) -> Result<u64, Errno> {
        if name.is_empty() || name == "." || name == ".." || name.contains('/') {
            return Err(Errno::EINVAL);
        }

        if self.files.values().any(|file| file.name == name) {
            return Err(Errno::EEXIST);
        }

        let encrypted_empty = self.crypto.encrypt(&[]).map_err(|_| Errno::EIO)?;

        let ino = self.next_inode;
        self.next_inode = self.next_inode.checked_add(1).ok_or(Errno::EIO)?;

        self.persist_ciphertext(ino, &encrypted_empty)?;

        let now = SystemTime::now();

        self.files.insert(
            ino,
            VirtualFile {
                name: name.to_string(),
                data: encrypted_empty,
                plaintext_size: 0,
                mode,
                uid,
                gid,
                created: now,
                modified: now,
                accessed: now,
            },
        );

        Ok(ino)
    }

    fn lookup_name(&self, name: &str) -> Option<u64> {
        self.files
            .iter()
            .find(|(_, file)| file.name == name)
            .map(|(ino, _)| *ino)
    }

    fn write_file(&mut self, ino: u64, offset: u64, data: &[u8]) -> Result<usize, Errno> {
        let start = usize::try_from(offset).map_err(|_| Errno::EINVAL)?;
        let end = start.checked_add(data.len()).ok_or(Errno::EINVAL)?;

        let (mut plaintext, expected_size) = {
            let file = self.files.get(&ino).ok_or(Errno::ENOENT)?;
            if data.is_empty() {
                return Ok(0);
            }

            let plaintext = self.crypto.decrypt(&file.data).map_err(|_| Errno::EIO)?;
            (plaintext, file.plaintext_size)
        };

        if plaintext.len() != expected_size {
            return Err(Errno::EIO);
        }

        plaintext.resize(plaintext.len().max(end), 0);
        plaintext[start..end].copy_from_slice(data);

        let encrypted = self.crypto.encrypt(&plaintext).map_err(|_| Errno::EIO)?;

        self.persist_ciphertext(ino, &encrypted)?;

        let file = self.files.get_mut(&ino).ok_or(Errno::ENOENT)?;
        file.data = encrypted;
        file.plaintext_size = plaintext.len();
        file.modified = SystemTime::now();

        Ok(data.len())
    }

    fn resize_file(&mut self, ino: u64, new_size: usize) -> Result<(), Errno> {
        let (mut plaintext, expected_size) = {
            let file = self.files.get(&ino).ok_or(Errno::ENOENT)?;
            let plaintext = self.crypto.decrypt(&file.data).map_err(|_| Errno::EIO)?;
            (plaintext, file.plaintext_size)
        };

        if plaintext.len() != expected_size {
            return Err(Errno::EIO);
        }

        plaintext.resize(new_size, 0);
        let encrypted = self.crypto.encrypt(&plaintext).map_err(|_| Errno::EIO)?;

        self.persist_ciphertext(ino, &encrypted)?;

        let file = self.files.get_mut(&ino).ok_or(Errno::ENOENT)?;
        file.data = encrypted;
        file.plaintext_size = new_size;
        file.modified = SystemTime::now();

        Ok(())
    }

    fn read_file(&self, ino: u64, offset: u64, size: u32) -> Result<Vec<u8>, Errno> {
        let (ciphertext, expected_size) = {
            let file = self.files.get(&ino).ok_or(Errno::ENOENT)?;
            (file.data.clone(), file.plaintext_size)
        };

        let plaintext = self.crypto.decrypt(&ciphertext).map_err(|_| Errno::EIO)?;

        if plaintext.len() != expected_size {
            return Err(Errno::EIO);
        }

        let start = usize::try_from(offset).map_err(|_| Errno::EINVAL)?;

        if start >= plaintext.len() {
            return Ok(Vec::new());
        }

        let end = start.saturating_add(size as usize).min(plaintext.len());
        Ok(plaintext[start..end].to_vec())
    }

    fn file_attr(&self, ino: u64) -> Option<FileAttr> {
        let file = self.files.get(&ino)?;
        let size = file.plaintext_size as u64;

        Some(FileAttr {
            ino: INodeNo(ino),
            size,
            blocks: size.div_ceil(512),
            atime: file.accessed,
            mtime: file.modified,
            ctime: file.modified,
            crtime: file.created,
            kind: FileType::RegularFile,
            perm: file.mode,
            nlink: 1,
            uid: file.uid,
            gid: file.gid,
            rdev: 0,
            blksize: 512,
            flags: 0,
        })
    }
}

struct VirtualFS {
    state: Mutex<FsState>,
}

impl VirtualFS {
    fn new() -> Self {
        Self {
            state: Mutex::new(FsState::new()),
        }
    }
}

fn root_attr() -> FileAttr {
    FileAttr {
        ino: INodeNo(ROOT_INODE),
        size: 0,
        blocks: 0,
        atime: UNIX_EPOCH,
        mtime: UNIX_EPOCH,
        ctime: UNIX_EPOCH,
        crtime: UNIX_EPOCH,
        kind: FileType::Directory,
        perm: 0o755,
        nlink: 2,
        uid: unsafe { libc::getuid() },
        gid: unsafe { libc::getgid() },
        rdev: 0,
        blksize: 512,
        flags: 0,
    }
}

impl Filesystem for VirtualFS {
    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        if parent.0 != ROOT_INODE {
            reply.error(Errno::ENOENT);
            return;
        }

        let Some(name) = name.to_str() else {
            reply.error(Errno::EINVAL);
            return;
        };

        let state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        if let Some(ino) = state.lookup_name(name) {
            let attr = state.file_attr(ino).unwrap();
            reply.entry(&TTL, &attr, fuser::Generation(0));
        } else {
            reply.error(Errno::ENOENT);
        }
    }

    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        if ino.0 == ROOT_INODE {
            reply.attr(&TTL, &root_attr());
            return;
        }

        let state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        match state.file_attr(ino.0) {
            Some(attr) => reply.attr(&TTL, &attr),
            None => reply.error(Errno::ENOENT),
        }
    }

    fn setattr(
        &self,
        _req: &Request,
        ino: INodeNo,
        mode: Option<u32>,
        uid: Option<u32>,
        gid: Option<u32>,
        size: Option<u64>,
        atime: Option<TimeOrNow>,
        mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<FileHandle>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<fuser::BsdFileFlags>,
        reply: ReplyAttr,
    ) {
        if ino.0 == ROOT_INODE {
            reply.error(Errno::EISDIR);
            return;
        }

        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        if let Some(size) = size {
            let Ok(new_size) = usize::try_from(size) else {
                reply.error(Errno::EINVAL);
                return;
            };

            if let Err(err) = state.resize_file(ino.0, new_size) {
                reply.error(err);
                return;
            }
        }

        {
            let Some(file) = state.files.get_mut(&ino.0) else {
                reply.error(Errno::ENOENT);
                return;
            };

            if let Some(mode) = mode {
                file.mode = (mode & 0o7777) as u16;
            }

            if let Some(uid) = uid {
                file.uid = uid;
            }

            if let Some(gid) = gid {
                file.gid = gid;
            }

            if let Some(atime) = atime {
                file.accessed = match atime {
                    TimeOrNow::SpecificTime(time) => time,
                    TimeOrNow::Now => SystemTime::now(),
                };
            }

            if let Some(mtime) = mtime {
                file.modified = match mtime {
                    TimeOrNow::SpecificTime(time) => time,
                    TimeOrNow::Now => SystemTime::now(),
                };
            }
        }

        match state.file_attr(ino.0) {
            Some(attr) => reply.attr(&TTL, &attr),
            None => reply.error(Errno::ENOENT),
        }
    }

    fn create(
        &self,
        req: &Request,
        parent: INodeNo,
        name: &OsStr,
        mode: u32,
        umask: u32,
        _flags: i32,
        reply: ReplyCreate,
    ) {
        if parent.0 != ROOT_INODE {
            reply.error(Errno::ENOENT);
            return;
        }

        let Some(name) = name.to_str() else {
            reply.error(Errno::EINVAL);
            return;
        };

        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        let permissions = (mode & !umask & 0o7777) as u16;

        match state.create_file(name, permissions, req.uid(), req.gid()) {
            Ok(ino) => {
                let attr = state.file_attr(ino).unwrap();

                reply.created(
                    &TTL,
                    &attr,
                    fuser::Generation(0),
                    FileHandle(0),
                    FopenFlags::empty(),
                );
            }
            Err(err) => reply.error(err),
        }
    }

    fn open(&self, _req: &Request, ino: INodeNo, _flags: OpenFlags, reply: ReplyOpen) {
        if ino.0 == ROOT_INODE {
            reply.error(Errno::EISDIR);
            return;
        }

        let state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        if state.files.contains_key(&ino.0) {
            reply.opened(FileHandle(0), FopenFlags::empty());
        } else {
            reply.error(Errno::ENOENT);
        }
    }

    fn read(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        size: u32,
        _flags: OpenFlags,
        _lock_owner: Option<LockOwner>,
        reply: ReplyData,
    ) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        match state.read_file(ino.0, offset, size) {
            Ok(data) => {
                if let Some(file) = state.files.get_mut(&ino.0) {
                    file.accessed = SystemTime::now();
                }
                reply.data(&data);
            }
            Err(err) => reply.error(err),
        }
    }

    fn write(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        data: &[u8],
        _write_flags: WriteFlags,
        _flags: OpenFlags,
        _lock_owner: Option<LockOwner>,
        reply: ReplyWrite,
    ) {
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        match state.write_file(ino.0, offset, data) {
            Ok(size) => reply.written(size as u32),
            Err(err) => reply.error(err),
        }
    }

    fn readdir(
        &self,
        _req: &Request,
        ino: INodeNo,
        _fh: FileHandle,
        offset: u64,
        mut reply: ReplyDirectory,
    ) {
        if ino.0 != ROOT_INODE {
            reply.error(Errno::ENOTDIR);
            return;
        }

        let state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => {
                reply.error(Errno::EIO);
                return;
            }
        };

        let mut entries = vec![
            (INodeNo(ROOT_INODE), FileType::Directory, ".".to_string()),
            (INodeNo(ROOT_INODE), FileType::Directory, "..".to_string()),
        ];

        for (ino, file) in &state.files {
            entries.push((INodeNo(*ino), FileType::RegularFile, file.name.clone()));
        }

        for (i, entry) in entries.iter().enumerate().skip(offset as usize) {
            if reply.add(entry.0, (i + 1) as u64, entry.1, entry.2.as_str()) {
                break;
            }
        }

        reply.ok();
    }
}

fn main() {
    let mountpoint = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -- <mountpoint>");

    let mut config = fuser::Config::default();

    config.mount_options.extend([
        MountOption::RW,
        MountOption::FSName("quantum-vault".to_string()),
    ]);

    println!("Starting QuantumVault virtual filesystem...");

    fuser::mount(VirtualFS::new(), mountpoint, &config).expect("Failed to mount FUSE filesystem");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_virtual_file_creation() {
        let mut state = FsState::new();
        let ino = state.create_file("test.txt", 0o644, 1000, 1000).unwrap();

        assert_eq!(ino, 2);
        assert!(state.files.contains_key(&ino));
    }

    #[test]
    fn test_duplicate_file_creation() {
        let mut state = FsState::new();

        state.create_file("test.txt", 0o644, 1000, 1000).unwrap();

        assert!(matches!(
            state.create_file("test.txt", 0o644, 1000, 1000),
            Err(Errno::EEXIST)
        ));
    }

    #[test]
    fn test_file_write_and_read() {
        let mut state = FsState::new();
        let ino = state.create_file("data.txt", 0o644, 1000, 1000).unwrap();

        state.write_file(ino, 0, b"QuantumVault").unwrap();

        assert_eq!(state.read_file(ino, 0, 100).unwrap(), b"QuantumVault");
    }

    #[test]
    fn test_file_listing_and_lookup() {
        let mut state = FsState::new();

        state.create_file("one.txt", 0o644, 1000, 1000).unwrap();
        state.create_file("two.txt", 0o644, 1000, 1000).unwrap();

        assert_eq!(state.lookup_name("one.txt"), Some(2));
        assert_eq!(state.lookup_name("two.txt"), Some(3));
        assert_eq!(state.files.len(), 2);
    }

    #[test]
    fn test_file_metadata() {
        let mut state = FsState::new();
        let ino = state
            .create_file("metadata.txt", 0o640, 1000, 1000)
            .unwrap();

        state.write_file(ino, 0, b"Hello").unwrap();

        let attr = state.file_attr(ino).unwrap();

        assert_eq!(attr.size, 5);
        assert_eq!(attr.perm, 0o640);
        assert_eq!(attr.kind, FileType::RegularFile);
        assert_eq!(attr.uid, 1000);
        assert_eq!(attr.gid, 1000);
    }

    #[test]
    fn test_file_storage_contains_ciphertext() {
        let mut state = FsState::new();
        let ino = state
            .create_file("encrypted.txt", 0o600, 1000, 1000)
            .unwrap();
        let plaintext = b"QuantumVault secret data";

        state.write_file(ino, 0, plaintext).unwrap();

        let stored = &state.files.get(&ino).unwrap().data;
        assert_ne!(stored.as_slice(), plaintext);
        assert_eq!(stored.len(), plaintext.len() + 28);

        let disk_path = state.storage_dir.join(format!("{ino}.qv"));
        let disk_contents = fs::read(disk_path).unwrap();

        assert_eq!(disk_contents, *stored);
        assert_ne!(disk_contents.as_slice(), plaintext);
        assert_eq!(state.read_file(ino, 0, 100).unwrap(), plaintext);
    }

    #[test]
    fn test_empty_write_does_not_change_stored_ciphertext() {
        let mut state = FsState::new();
        let ino = state.create_file("empty.txt", 0o600, 1000, 1000).unwrap();

        let before = fs::read(state.storage_dir.join(format!("{ino}.qv"))).unwrap();
        assert_eq!(state.write_file(ino, 0, b"").unwrap(), 0);
        let after = fs::read(state.storage_dir.join(format!("{ino}.qv"))).unwrap();

        assert_eq!(before, after);
        assert_eq!(state.read_file(ino, 0, 10).unwrap(), b"");
    }

    #[test]
    fn test_invalid_write_inputs() {
        let mut state = FsState::new();

        assert!(matches!(
            state.write_file(999, 0, b"data"),
            Err(Errno::ENOENT)
        ));

        let ino = state
            .create_file("invalid-write.txt", 0o600, 1000, 1000)
            .unwrap();

        assert!(matches!(
            state.write_file(ino, u64::MAX, b"data"),
            Err(Errno::EINVAL)
        ));
    }

    #[test]
    fn test_tampered_file_is_rejected() {
        let mut state = FsState::new();
        let ino = state
            .create_file("tampered.txt", 0o600, 1000, 1000)
            .unwrap();

        state.write_file(ino, 0, b"Sensitive data").unwrap();
        state.files.get_mut(&ino).unwrap().data[12] ^= 1;

        assert!(matches!(state.read_file(ino, 0, 100), Err(Errno::EIO)));
    }

    #[test]
    fn test_encrypted_file_resize() {
        let mut state = FsState::new();
        let ino = state.create_file("resize.txt", 0o600, 1000, 1000).unwrap();

        state.write_file(ino, 0, b"HelloWorld").unwrap();
        state.resize_file(ino, 5).unwrap();

        assert_eq!(state.read_file(ino, 0, 100).unwrap(), b"Hello");
        assert_eq!(state.file_attr(ino).unwrap().size, 5);
    }

    #[test]
    fn test_invalid_filename() {
        let mut state = FsState::new();

        assert!(state.create_file("../bad.txt", 0o644, 1000, 1000).is_err());
    }

    #[test]
    fn test_write_at_offset() {
        let mut state = FsState::new();
        let ino = state.create_file("offset.txt", 0o644, 1000, 1000).unwrap();

        state.write_file(ino, 0, b"HelloWorld").unwrap();
        state.write_file(ino, 5, b"Rust").unwrap();

        assert_eq!(state.read_file(ino, 0, 100).unwrap(), b"HelloRustd");
    }
}
