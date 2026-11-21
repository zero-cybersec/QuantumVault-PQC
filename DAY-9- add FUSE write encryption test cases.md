# QuantumVault FUSE Write Encryption Test Cases

**Purpose:** Define test inputs, expected encrypted-storage properties, failure behavior, and a checklist for future FUSE integration testing.

**Status:** Test specification only. No FUSE write-encryption implementation or mounted-filesystem integration test is included in the current QuantumVault CLI. The existing `fs/vfs-sandbox/` entry is sample mount-catalog data. Treat the cases below as requirements for the later integration test suite, not as completed product behavior.

## 1. Test environment and prerequisites

- A disposable Linux test host with FUSE support and a temporary mount directory.
- A test vault and non-production test key material supplied through the approved test harness.
- A mounted QuantumVault FUSE filesystem backed by a temporary storage directory.
- Test observer capable of examining the backing bytes without logging or exposing plaintext/key material in shared CI logs.
- Test each successful write through the filesystem interface, then inspect the backing representation and read the file back through the mounted interface.
- Run all destructive/corruption cases only against disposable fixtures.

## 2. Input data set

| ID | Input | Expected plaintext length | Purpose |
|---|---|---:|---|
| IN-01 | UTF-8 bytes `database_password_super_secret_pqc_2026` | 40 bytes | Ordinary short file write |
| IN-02 | Empty byte string `b""` | 0 bytes | Empty-file semantics |
| IN-03 | `b"A" * 102400` | 102,400 bytes (100 KiB) | Larger content, multi-block/chunk writes |
| IN-04 | Bytes `00 01 7f 80 ff` | 5 bytes | Binary/NUL and non-UTF-8 preservation |
| IN-05 | Payloads of block-size - 1, block-size, block-size + 1 | Depends on configured block size | Chunk-boundary coverage |
| IN-06 | Same test payload written twice | Same as selected input | Randomized encryption and nonce uniqueness |

Input lengths and test values should be generated in the test fixture and asserted from actual byte arrays, rather than copied from this table as hardcoded expected counts if values change.

## 3. Test cases and expected results

| ID | Scenario and steps | Expected result |
|---|---|---|
| TC-FW-01 | Write IN-01; verify write completion; read it back | Write succeeds; exact byte-for-byte round trip; backing representation contains ciphertext/metadata, not the complete plaintext sequence |
| TC-FW-02 | Create a file with IN-02; read it back | File exists and reads as exactly zero bytes; storage still uses the documented authenticated container format |
| TC-FW-03 | Write and read IN-03 | Exact round trip across multiple blocks; no truncation, duplication, or reordering; storage size includes documented encryption overhead |
| TC-FW-04 | Write and read IN-04 | Exact raw byte equality, including NUL and non-UTF-8 bytes; no text encoding conversion |
| TC-FW-05 | Write IN-05 around configured block boundaries | Every payload round-trips exactly; chunk count, lengths, offsets, and final partial block are correct |
| TC-FW-06 | Write identical plaintext twice, read both | Both reads match; ciphertext differs because each encryption uses fresh, unique nonces (and fresh per-file data key/salt if that is the chosen design) |
| TC-FW-07 | Flip one ciphertext or authentication-tag bit in the backing fixture, then read | Read fails with integrity error; no plaintext or partial plaintext is returned; map to `EIO` and QuantumVault integrity exit code 6 where applicable |
| TC-FW-08 | Truncate header, chunk metadata, ciphertext, and tag in separate fixtures | Each malformed container is rejected before returning plaintext; controlled integrity/format error, no crash |
| TC-FW-09 | Attempt a write while the vault/session is locked or expired | Write denied; new path remains absent, or old file remains unchanged; map to `EACCES` / vault-locked code 3 |
| TC-FW-10 | Attempt a write on a read-only mount or without write policy | Write denied; storage unchanged; map to `EACCES` / access-denied code 5 |
| TC-FW-11 | Exhaust quota during a new write and during an overwrite | Return `ENOSPC`; no partial file is visible; failed overwrite preserves the prior committed contents |
| TC-FW-12 | Supply invalid path, traversal path, negative offset, or invalid length/flag | Reject with `EINVAL` / invalid-arguments code 2; do not modify storage |
| TC-FW-13 | Inject crypto-backend or key-service failure before commit | Return controlled crypto/general error; no partial ciphertext is committed; do not log plaintext, keys, or recovery material |
| TC-FW-14 | Kill the writer during write/flush/rename, then remount | Recovery exposes either the previous complete version or the new complete version, never a partial container; exact durability semantics follow the chosen `fsync` contract |
| TC-FW-15 | Race two writers to the same file | Behavior follows documented locking/version policy; no mixed or corrupt ciphertext; conflict is surfaced consistently or writes are serialized |
| TC-FW-16 | Restart/remount and write enough blocks to exercise nonce allocation | Nonces remain unique for a given encryption key across chunks, files, process restarts, and concurrent writes; no counter rollback/reuse |

## 4. Encrypted-storage assertions

For each successful write, integration tests should verify:

1. The backing representation conforms to the versioned storage format. Header fields that affect parsing or security are authenticated.
2. The full plaintext byte sequence is not present in the ciphertext representation. This is a leakage sanity check, not proof of encryption strength.
3. Every nonce is unique for the associated key, including across restarts and concurrent writers; the nonce policy is documented and tested.
4. Reads authenticate the container before releasing plaintext and reproduce the original bytes exactly.
5. Tampering, wrong keys, wrong key identifiers, malformed lengths, and missing/reordered chunks fail closed.
6. Temporary files, journals, crash recovery artifacts, debug output, and error messages do not leak plaintext or key material.
7. Publication is atomic and follows explicitly documented durability semantics for write, flush, and close.
8. Empty files use authenticated storage records rather than an unauthenticated plaintext special case.
9. Encryption algorithm, key wrapping, key rotation, and key storage match approved cryptographic design. Statistical entropy alone is not an acceptance criterion.

## 5. Failure behavior and status mapping

| Failure | Expected filesystem result | CLI mapping (if surfaced by CLI) |
|---|---|---|
| Invalid path/offset | `EINVAL` | 2, invalid arguments |
| Vault locked/session expired | `EACCES` | 3, vault locked |
| File does not exist on read | `ENOENT` | 4, not found |
| Read-only mount/policy denial | `EACCES` | 5, access denied |
| Corrupt or tampered ciphertext | `EIO`; no plaintext | 6, integrity failure |
| Unsupported crypto suite/KEM failure | Controlled crypto error; no commit | 7, PQC algorithm error |
| Backing store full | `ENOSPC`; atomic rollback | Storage/general error mapping must be agreed and documented |
| Concurrent write conflict | `EBUSY` or documented serialized behavior | Storage/general error mapping must be agreed and documented |
| Unexpected backend failure | Controlled error; no partial commit | 1, general error, unless a more specific mapping is defined |

All failure cases must preserve a previously committed file, avoid partial plaintext output, and keep diagnostics free of secret bytes, private keys, DEKs/KEKs, and recovery material.

## 6. Integration checklist

- [ ] Agree on the encrypted container format, versioning, authenticated fields, and key-rotation compatibility.
- [ ] Use the approved production cryptographic implementation; do not substitute a test-only cipher or key-wrapping scheme.
- [ ] Confirm nonce generation/uniqueness strategy across threads, processes, restarts, and crash recovery.
- [ ] Implement atomic temporary-write/commit behavior and document flush/fsync semantics.
- [ ] Add mount-level tests for TC-FW-01 through TC-FW-16, marking unsupported cases with an explicit rationale.
- [ ] Verify backing storage and logs for plaintext/key leakage using test-only secrets.
- [ ] Verify locked, read-only, quota, corruption, and crypto-failure behavior and the resulting errno/status mappings.
- [ ] Run in an isolated FUSE-enabled environment; ensure cleanup even after test failures.
- [ ] Record environment, FUSE/kernel versions, test commands, per-case results, and failure logs with secrets redacted.
- [ ] Run full regression suite after integration.

## 7. Current verification status

These are prepared test cases and expected outcomes for later integration. No FUSE encryption tests have been executed because the accessible codebase contains CLI sample mount metadata but no FUSE write-encryption implementation or mount integration harness. The existing CLI unit suite is separate from this test specification and should not be presented as FUSE validation.

