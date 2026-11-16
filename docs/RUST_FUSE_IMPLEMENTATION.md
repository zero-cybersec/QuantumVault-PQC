# QuantumVault-PQC — Rust FUSE Filesystem

## 1. Objective
Implement a read-write virtual filesystem using Rust and FUSE.

## 2. Implemented Features
- Virtual file creation using the FUSE create operation.
- Dynamic directory listing using readdir.
- File lookup and metadata retrieval.
- File reading and writing.
- File truncation and metadata updates using setattr.
- In-memory file storage protected by a Mutex.
- Duplicate filename validation.
- File permissions and timestamp handling.

## 3. Unit Testing
Seven unit tests were executed:

- Virtual file creation
- Duplicate file creation
- File write and read
- File listing and lookup
- File metadata
- Invalid filename
- Write at offset

Result: 7 passed, 0 failed.

## 4. Build Verification
The following commands completed successfully:

- cargo fmt
- cargo check
- cargo test
- cargo build

## 5. FUSE Integration Testing
Mounted the filesystem at /tmp/quantumvault-mnt.

Verified operations:
- touch: virtual file creation
- echo: writing file contents
- cat: reading file contents
- ls -la: directory listing
- stat: file metadata

Test file: test.txt
Expected and actual content: QuantumVault Rust FUSE Test
File size: 28 bytes
Permissions: 0644
Inode: 2

All listed integration checks passed.

The filesystem was successfully unmounted using fusermount3.

## 6. Storage Limitation
Files are stored in memory. They are not persistent and are lost when the filesystem process terminates or is unmounted.

## 7. Conclusion
Virtual file creation, listing, reading, writing, metadata handling, unit testing, and live FUSE integration testing were completed successfully.
