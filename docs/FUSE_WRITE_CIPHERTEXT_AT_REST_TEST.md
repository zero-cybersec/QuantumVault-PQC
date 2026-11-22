# FUSE Write and Ciphertext-at-Rest Verification

**Contributor:** Md Zishan Hussain
**Project:** QuantumVault-PQC
**Branch:** zeeshan

## 1. Objective

- Test FUSE write operations.
- Verify ciphertext-at-rest behavior.
- Write a test file through the FUSE mount.
- Inspect backing storage.
- Document test results and blockers.

## 2. Test Procedure

1. Start the filesystem:

   cargo run -- /tmp/qv-mnt

2. Write a test file through the mounted filesystem:

   printf 'QuantumVault Live Encryption Test 12345' > /tmp/qv-mnt/live_test.txt

3. Read the file:

   cat /tmp/qv-mnt/live_test.txt

4. Verify the mount:

   findmnt -T /tmp/qv-mnt

5. Inspect backing storage:

   find /tmp -type f -name '*.qv' -ls

6. Inspect encrypted bytes:

   xxd /tmp/quantumvault-pqc-10811-0/3.qv

7. Search for plaintext:

   grep -aFq 'QuantumVault Live Encryption Test 12345' \
   /tmp/quantumvault-pqc-10811-0/3.qv

## 3. Observed Results

| Test | Result |
|---|---|
| FUSE mount detection | PASS |
| File write through FUSE | PASS |
| Read-back verification | PASS |
| Backing .qv file creation | PASS |
| Ciphertext hex inspection | PASS |
| Exact plaintext search | PASS — plaintext not found |

Test message length: 39 bytes.
Observed backing file size: 67 bytes.
AES-256-GCM overhead: 28 bytes.

Expected encrypted size: 39 + 28 = 67 bytes.

## 4. Blockers and Limitations

- Initial mount attempt returned Permission denied.
- A subsequent live mount was detected successfully.
- Session-specific temporary backing storage is used.
- Encryption keys are generated at runtime and are not persisted.
- Restart-safe key recovery remains future work.

## 5. Conclusion

The live FUSE write test successfully demonstrated that the test
message could be read back through the mounted filesystem while
the backing storage contained encrypted binary data.

The exact plaintext was not found in the backing file.
