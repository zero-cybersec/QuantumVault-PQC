# FUSE Write Test – Reproduction Guide

## Objective
Verify that files written through the QuantumVault FUSE mount are
stored as encrypted ciphertext rather than plaintext.

## Test Procedure

1. Build the project using `cargo build`.
2. Create a mount directory using `mkdir -p /tmp/qv-mnt`.
3. Start the filesystem using the project's FUSE mount command.
4. Write a test message through the mounted filesystem.
5. Read the file through the mount and verify the original plaintext.
6. Locate the session backing directory under `/tmp/quantumvault-pqc-*`.
7. Inspect the corresponding `.qv` backing file.
8. Verify that the backing file does not contain the original plaintext.

## Expected Results

- The FUSE mount accepts file write operations.
- Reading through the mounted filesystem returns the original content.
- The backing file contains encrypted binary data.
- The original plaintext is not directly visible in the backing file.

## Important Limitation

This verification demonstrates ciphertext-at-rest for the tested
session. It does not establish persistent encryption-key management
or decryption after restarting the filesystem.

## Result

The live FUSE write and ciphertext-at-rest checks were previously
verified. Refer to FUSE_WRITE_CIPHERTEXT_AT_REST_TEST.md for the
recorded test results.
