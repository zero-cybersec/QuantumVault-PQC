use std::ffi::OsStr;
use std::time::{Duration, UNIX_EPOCH};

use fuser::{
    Errno, FileAttr, FileHandle, FileType, Filesystem, INodeNo, LockOwner, MountOption, OpenFlags,
    ReplyAttr, ReplyData, ReplyDirectory, ReplyEntry, Request,
};

const TTL: Duration = Duration::from_secs(1);
const HELLO: &[u8] = b"Hello World from Rust FUSE!\n";

struct HelloFS;

fn get_attr(ino: INodeNo) -> Option<FileAttr> {
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };

    let attr = match ino.0 {
        1 => FileAttr {
            ino: INodeNo(1),
            size: 0,
            blocks: 0,
            atime: UNIX_EPOCH,
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 2,
            uid,
            gid,
            rdev: 0,
            blksize: 512,
            flags: 0,
        },

        2 => FileAttr {
            ino: INodeNo(2),
            size: HELLO.len() as u64,
            blocks: 1,
            atime: UNIX_EPOCH,
            mtime: UNIX_EPOCH,
            ctime: UNIX_EPOCH,
            crtime: UNIX_EPOCH,
            kind: FileType::RegularFile,
            perm: 0o444,
            nlink: 1,
            uid,
            gid,
            rdev: 0,
            blksize: 512,
            flags: 0,
        },

        _ => return None,
    };

    Some(attr)
}

impl Filesystem for HelloFS {
    fn lookup(&self, _req: &Request, parent: INodeNo, name: &OsStr, reply: ReplyEntry) {
        if parent == INodeNo(1) && name == OsStr::new("hello.txt") {
            reply.entry(&TTL, &get_attr(INodeNo(2)).unwrap(), fuser::Generation(0));
        } else {
            reply.error(Errno::ENOENT);
        }
    }

    fn getattr(&self, _req: &Request, ino: INodeNo, _fh: Option<FileHandle>, reply: ReplyAttr) {
        match get_attr(ino) {
            Some(attr) => reply.attr(&TTL, &attr),
            None => reply.error(Errno::ENOENT),
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
        if ino != INodeNo(1) {
            reply.error(Errno::ENOENT);
            return;
        }

        let entries = [
            (INodeNo(1), FileType::Directory, "."),
            (INodeNo(1), FileType::Directory, ".."),
            (INodeNo(2), FileType::RegularFile, "hello.txt"),
        ];

        for (i, entry) in entries.iter().enumerate().skip(offset as usize) {
            if reply.add(entry.0, (i + 1) as u64, entry.1, entry.2) {
                break;
            }
        }

        reply.ok();
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
        if ino != INodeNo(2) {
            reply.error(Errno::ENOENT);
            return;
        }

        let start = offset as usize;

        if start >= HELLO.len() {
            reply.data(&[]);
            return;
        }

        let end = start.saturating_add(size as usize).min(HELLO.len());
        reply.data(&HELLO[start..end]);
    }
}

fn main() {
    let mountpoint = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -- <mountpoint>");

    let mut config = fuser::Config::default();

    config.mount_options.extend([
        MountOption::RO,
        MountOption::FSName("rust-hello-fs".to_string()),
    ]);

    println!("Starting Rust Hello World FUSE filesystem...");

    fuser::mount(HelloFS, mountpoint, &config).expect("Failed to mount FUSE filesystem");
}
