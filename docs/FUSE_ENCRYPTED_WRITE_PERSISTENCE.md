# FUSE Encrypted Write Persistence — Test Report

**Contributor:** Md Zishan Hussain
**Project:** QuantumVault-PQC
**Branch:** zeeshan

## Objective
Verify encrypted data persistence in the FUSE write path and confirm
that plaintext is not stored in the backing file.

## Implementation
- FUSE write callback delegates to FsState::write_file.
- Existing encrypted content is decrypted before modification.
- Updated content is encrypted before persistence.
- Ciphertext is stored in a `.qv` backing file.
- Temporary file and rename are used for persistence.

## Encryption Format
- 12-byte nonce
- 16-byte authentication tag
- Ciphertext payload
- Total overhead: 28 bytes

## Test Procedure
Commands executed:

    cargo fmt
    cargo test
    cargo check

## Observed Results

    16 tests passed
    0 tests failed
    cargo check completed successfully

Verified cases:
- Encrypted file storage
- Disk ciphertext matches encrypted in-memory data
- Plaintext is not stored as the file contents
- Empty write handling
- Invalid inode and offset handling
- Read/write and offset operations
- Tampered ciphertext rejection

## FUSE Write-Path Limitations
- Backing storage currently uses a session-specific temporary directory.
- Unit tests exercise the FsState write persistence path.
- A live mounted FUSE end-to-end test remains outstanding.
- FUSE reads currently decrypt data held in memory rather than reloading
  the backing file.
- Encryption keys are generated at runtime and are not persisted.
- Restart-safe key management is not implemented.

## Conclusion
All 16 unit tests passed and cargo check succeeded.
The unit tests verify encrypted disk persistence and input validation.
Live mounted FUSE verification and persistent key management remain
future work.

## 7. Live Mounted FUSE Verification

A live FUSE mount was successfully verified in the Linux environment.

### Procedure

1. Started the filesystem using:
   `cargo run -- /tmp/qv-mnt`
2. Confirmed the mount using `findmnt`.
3. Wrote the following test message through the mounted filesystem:

   `QuantumVault Live Encryption Test 12345`

4. Read the file back using `cat`.
5. Inspected the backing file:
   `/tmp/quantumvault-pqc-10811-0/3.qv`
6. Examined its bytes using `xxd`.
7. Searched the backing file for the exact plaintext using `grep -aFq`.

### Observed Results

- FUSE mount detected: PASS
- QuantumVault process running: PASS
- File write through mount: PASS
- Original plaintext read back: PASS
- Backing `.qv` file created: PASS
- Backing file size: 67 bytes
- Expected size: 39-byte plaintext + 28-byte encryption overhead
- Exact plaintext search in backing file: PASS — not found
- Hex dump showed binary encrypted data.

### Conclusion

The live mounted FUSE write test confirmed that the test message
could be read back through the mounted filesystem while the corresponding
backing file contained encrypted binary data rather than the exact
plaintext message.

This verifies the tested live write path. Restart-safe key management
and durable storage configuration remain future improvements.
