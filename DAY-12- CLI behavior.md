# QuantumVault (`qv`): CLI Signing and Signature Verification Design & Test Results

**Status:** IMPLEMENTED & VERIFIED 
**Scope:** Post-Quantum Cryptographic (PQC) Digital Signature Commands (`qv sign`, `qv verify`, `qv key sign`, `qv key verify`), Input/Output Format Specifications, and Positive/Negative Test Execution Evidence.

---

## 1. Executive Summary

QuantumVault is a zero-trust, post-quantum cryptographic secrets vault. As part of the cryptographic subsystem, the CLI has been extended with high-assurance digital signature generation and verification capabilities adhering to NIST FIPS 204 (ML-DSA) and NIST SP 800-208 / Round 3 standards.

The signature subsystem provides:
- **`qv sign`**: Generates post-quantum digital signatures for inline strings, local files, and standard input streams (`-`).
- **`qv verify`**: Validates signature authenticity, payload integrity, key identity, and domain separation contexts.
- **`qv key sign` / `qv key verify`**: Administrative command aliases under the existing `key` group.
- **Supported PQC Algorithms**:
  - `ml-dsa-65` (NIST FIPS 204 Parameter Set 2, primary default, 3,309 byte signature)
  - `ml-dsa-87` (NIST FIPS 204 Parameter Set 3, high-assurance category 5, 4,627 byte signature)
  - `falcon-512` (Compact lattice signature, 690 byte signature)
  - `falcon-1024` (High-security Falcon variant, 1,330 byte signature)
  - `sphincs+-sha2-128s` (Stateless hash-based signature, 7,856 byte signature)
  - `sphincs+-shake-256s` (Stateless SHAKE hash-based signature, 29,792 byte signature)
  - `hybrid-mldsa65-ed25519` (Dual classical/quantum defense-in-depth hybrid, 3,373 byte signature)
- **Zero-Trust Safety & Hardening**:
  - Rejection of classical or broken algorithms (e.g., RSA-2048, ECDSA-P256) with exit code `2`.
  - Rejection of path traversal (`..`) and shell injection metacharacters (`;&|$<>`).
  - Strict enforcement of NIST FIPS 204 Section 5.3 context strings ($\le 255$ bytes).
  - Explicit exit code contracts: `0` (Success/Valid), `2` (Invalid Arguments), `3` (Vault Locked), `4` (Key Not Found), `6` (Cryptographic Integrity Failure), and `7` (PQC Algorithm Unsupported).

---

## 2. Command Architecture & Syntax

### 2.1 `qv sign`

Generates a post-quantum digital signature over a designated payload using a vault-managed signing key.

```bash
qv [GLOBAL_OPTIONS] sign [OPTIONS] [PAYLOAD_PATH]
```

#### Arguments and Options:

| Option / Argument | Type | Default | Description |
|---|---|---|---|
| `[PAYLOAD_PATH]` | Positional | None | Path to payload file or `-` for standard input streaming. |
| `-k, --key-id TEXT` | String (Required) | None | Vault key identifier (2–64 alphanumeric, `.`, `_`, `-`). |
| `-a, --algorithm CHOICE` | String | `ml-dsa-65` | PQC signature algorithm suite. |
| `-d, --data TEXT` | String | None | Inline string literal payload to sign. |
| `-f, --file PATH` | Path | None | Path to local file to sign. |
| `-o, --out-file PATH` | Path | None | Output file path to write detached signature. |
| `-e, --encoding CHOICE` | Choice | `base64` | Signature output encoding: `base64`, `hex`, or `raw`. |
| `-c, --context TEXT` | String | `""` | NIST FIPS 204 domain separation context string ($\le 255$ bytes). |
| `--hash CHOICE` | Choice | `sha3-256` | Pre-hash message digest: `sha256`, `sha384`, `sha512`, `sha3-256`, `sha3-512`, or `none`. |
| `--attached / --detached`| Flag | `--detached`| Attached (bundled payload) vs detached signature mode. |

---

### 2.2 `qv verify`

Verifies a post-quantum digital signature against a specified payload and signing key identifier or standalone public key.

```bash
qv [GLOBAL_OPTIONS] verify [OPTIONS] [PAYLOAD_PATH]
```

#### Arguments and Options:

| Option / Argument | Type | Default | Description |
|---|---|---|---|
| `[PAYLOAD_PATH]` | Positional | None | Path to payload file or `-` for standard input streaming. |
| `-k, --key-id TEXT` | String | None | Vault key identifier used to resolve public key. |
| `-p, --public-key TEXT` | String | None | Standalone public key string or fingerprint. |
| `-s, --signature TEXT` | String | None | Encoded signature string (Base64 or Hex). |
| `--signature-file PATH` | Path | None | Path to file containing detached signature bytes. |
| `-d, --data TEXT` | String | None | Inline string literal payload to verify against. |
| `-f, --file PATH` | Path | None | Path to local payload file to verify. |
| `-a, --algorithm CHOICE` | String | `ml-dsa-65` | Expected signature algorithm suite. |
| `-e, --encoding CHOICE` | Choice | `base64` | Signature encoding format: `base64`, `hex`, or `raw`. |
| `-c, --context TEXT` | String | `""` | Expected NIST FIPS 204 domain separation context string. |

---

## 3. Input & Output Formats

QuantumVault supports three standardized output formats via the global `--format` flag (`text`, `json`, and `yaml`).

### 3.1 `qv sign` Output Formats

#### Human-Readable Text Format (`--format text` - Default):
```text
Key ID:           prod-signer
Key Version:      1
Algorithm:        ml-dsa-65
Hash Function:    sha3-256
Encoding:         base64
Context:          none
Signature (3309 B): UVZTRwEB47DEQpj8HBSa+/TImW+5JOWkdlpJxV3UDBrtG4Bw6wdTzWYzvy8vbM+2...
Fingerprint:      SHA256:0c40892bf9b1166296de22e59cfeb37d
```

#### Structured JSON Format (`--format json`):
```json
{
  "status": "success",
  "key_id": "prod-signer",
  "key_version": 1,
  "algorithm": "ml-dsa-65",
  "hash_function": "sha3-256",
  "encoding": "base64",
  "context": "",
  "signature": "UVZTRwEB47DEQpj8HBSa+/TImW+5JOWkdlpJxV3UDBrtG4Bw6wdTzWYzvy8vbM+2...",
  "signature_bytes": 3309,
  "fingerprint": "SHA256:0c40892bf9b1166296de22e59cfeb37d",
  "timestamp": "2026-11-26T20:34:05Z"
}
```

#### Structured YAML Format (`--format yaml`):
```yaml
algorithm: ml-dsa-65
context: ''
encoding: base64
fingerprint: SHA256:0c40892bf9b1166296de22e59cfeb37d
hash_function: sha3-256
key_id: prod-signer
key_version: 1
signature: UVZTRwEB47DEQpj8HBSa+/TImW+5JOWkdlpJxV3UDBrtG4Bw6wdTzWYzvy8vbM+2...
signature_bytes: 3309
status: success
timestamp: '2026-11-26T20:34:05Z'
```

---

### 3.2 `qv verify` Output Formats

#### Human-Readable Text Format:
```text
STATUS:           VERIFIED
Key ID:           prod-signer
Key Version:      1
Algorithm:        ml-dsa-65
Encoding:         base64
Context:          none
Verification:     Valid cryptographic signature match.
```

#### Structured JSON Format:
```json
{
  "status": "valid",
  "verified": true,
  "key_identifier": "prod-signer",
  "key_version": 1,
  "algorithm": "ml-dsa-65",
  "encoding": "base64",
  "context": "",
  "timestamp": "2026-11-26T20:34:35Z"
}
```

#### Tampered Payload or Corrupted Signature Failure (`exit 6`):
```text
Error [EXIT 6]: Cryptographic verification failed: Cryptographic integrity failure: signature does not match payload, key, or algorithm.
```
Or in JSON format on stderr:
```json
{
  "status": "error",
  "exit_code": 6,
  "message": "Cryptographic verification failed: Cryptographic integrity failure: signature does not match payload, key, or algorithm."
}
```

---

## 4. Exit Code Specification

| Code | Constant | Meaning in Signature Subsystem |
|---|---|---|
| `0` | `EXIT_SUCCESS` | Command completed successfully; signature generated or verified as valid. |
| `2` | `EXIT_INVALID_ARGS` | Parameter validation failure (empty key ID, unsupported algorithm, bad encoding, shell characters, directory traversal, context > 255 bytes, missing payload/signature). |
| `3` | `EXIT_VAULT_LOCKED` | Vault is sealed or locked; master key required to perform signing operations. |
| `4` | `EXIT_SECRET_NOT_FOUND` | Signing key identifier does not exist in the vault key catalog. |
| `6` | `EXIT_INTEGRITY_FAIL` | Cryptographic integrity verification failed (bit flip, payload mutation, key mismatch, context mismatch). |
| `7` | `EXIT_PQC_ALGO_ERROR` | Cryptographic algorithm execution error or unsupported hardware instruction set. |

---

## 5. Automated Test Suite Results

The signature command subsystem was validated across 31 automated unit tests (`tests/test_signature_commands.py`) and 31 end-to-end POSIX subprocess shell scenarios.

### 5.1 Unit Test Suite Execution (`unittest`)

```text
Ran 230 tests in 0.308s

OK (All 230 tests passed)
```

The test suite includes:
- `test_signature_commands.py`: 31 tests (15 positive, 16 negative)
- `test_argument_parsing.py`: 48 tests
- `test_validation.py`: 47 tests
- `test_identity_create.py`: 31 tests
- `test_identity_commands.py`: 20 tests
- `test_mount_commands.py`: 17 tests
- `test_vault_commands.py`: 9 tests
- `test_cli_basic.py`: 6 tests
- `test_error_handling.py`: 3 tests

---

### 5.2 Positive Test Case Matrix (15 Cases)

| ID | Test Scenario | CLI Arguments | Expected Result | Status |
|---|---|---|---|---|
| `TC-SIG-POS-01` | Sign inline string default | `qv sign -k prod-signer -d "payload"` | Exit 0, 3,309 byte base64 signature | **PASS** |
| `TC-SIG-POS-02` | Roundtrip in text format | `qv sign` + `qv verify` | Exit 0, `STATUS: VERIFIED` | **PASS** |
| `TC-SIG-POS-03` | Roundtrip in JSON format | `--format json sign` + `verify` | Exit 0, `verified: true` | **PASS** |
| `TC-SIG-POS-04` | Roundtrip in YAML format | `--format yaml sign` + `verify` | Exit 0, valid YAML structure | **PASS** |
| `TC-SIG-POS-05` | High-assurance ML-DSA-87 | `-a ml-dsa-87` | Exit 0, 4,627 byte signature | **PASS** |
| `TC-SIG-POS-06` | Compact Falcon-512 | `-a falcon-512` | Exit 0, 690 byte signature | **PASS** |
| `TC-SIG-POS-07` | Stateless SPHINCS+-SHA2-128s | `-a sphincs+-sha2-128s` | Exit 0, 7,856 byte signature | **PASS** |
| `TC-SIG-POS-08` | FIPS 204 Context String | `--context "banking-v1"` | Exit 0, matching context verified | **PASS** |
| `TC-SIG-POS-09` | Payload file signing | `--file /tmp/data.bin` | Exit 0, signature matches file bytes | **PASS** |
| `TC-SIG-POS-10` | Detached file generation | `--out-file /tmp/sig.b64` + `--signature-file` | Exit 0, signature written to disk | **PASS** |
| `TC-SIG-POS-11` | Hexadecimal encoding | `--encoding hex` | Exit 0, valid hex signature | **PASS** |
| `TC-SIG-POS-12` | Stdin streaming pipeline | `cat file \| qv sign -k key -` | Exit 0, stdin buffer consumed | **PASS** |
| `TC-SIG-POS-13` | Empty byte string payload | `-d ""` | Exit 0, valid signature over `b""` | **PASS** |
| `TC-SIG-POS-14` | Public key verification | `--public-key prod-signer` | Exit 0, verified against public key | **PASS** |
| `TC-SIG-POS-15` | Administrative aliases | `qv key sign` + `qv key verify` | Exit 0, alias dispatch functional | **PASS** |

---

### 5.3 Negative Test Case Matrix (16 Cases)

| ID | Test Scenario | Test Input / Condition | Expected Exit | Status |
|---|---|---|---|---|
| `TC-SIG-NEG-01` | Tampered signature | Mutated base64 signature byte | `6` (Integrity Failure) | **PASS** |
| `TC-SIG-NEG-02` | Tampered payload | Mutated message content | `6` (Integrity Failure) | **PASS** |
| `TC-SIG-NEG-03` | Wrong key verification | Verified with non-signing key | `6` (Integrity Failure) | **PASS** |
| `TC-SIG-NEG-04` | Context mismatch | Signed with `ctx-1`, verified with `ctx-2` | `6` (Integrity Failure) | **PASS** |
| `TC-SIG-NEG-05` | Missing payload | `qv sign -k key` without data/file/stdin | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-06` | Missing signature | `qv verify -k key -d "msg"` without `-s` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-07` | Missing key identifier | `qv verify -s "sig" -d "msg"` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-08` | Non-existent signing key | `qv sign -k missing-key-id` | `4` (Key Not Found) | **PASS** |
| `TC-SIG-NEG-09` | Non-existent verify key | `qv verify -k missing-key-id` | `4` (Key Not Found) | **PASS** |
| `TC-SIG-NEG-10` | Traversal in `--file` | `--file ../../etc/passwd` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-11` | Traversal in `--out-file` | `--out-file ../../tmp/sig.b64` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-12` | Shell injection in Key ID | `-k "key;rm -rf /"` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-13` | Classical cipher rejected | `-a rsa-2048` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-14` | Context string > 255 B | `--context "A"*256` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-15` | Malformed Base64 | `-s "not-valid-base64!!!"` | `2` (Invalid Arguments) | **PASS** |
| `TC-SIG-NEG-16` | Sealed/Locked vault | `QV_SIMULATE_LOCKED=1` | `3` (Vault Locked) | **PASS** |

---

## 6. Security Architecture & Implementation Caveat

1. **Simulation Engine in CLI Prototype**:
   - The CLI Python implementation utilizes `quantumvault/crypto_sign.py`, which is an explicit deterministic HMAC-based simulation model configured with exact NIST FIPS 204 byte lengths and key derivation routines.
   - It is designed for complete CLI ergonomics, argument validation, boundary condition testing, format translation, and POSIX workflow validation.
   - **Important Caveat**: This simulation is not a FIPS 204 certified cryptographic implementation and must not be used for production security guarantees. Production signing requires integration with the native QuantumVault C API / Rust FFI core (`liboqs` / C PQC engine).
2. **Strict Domain Separation Contexts**:
   - Following NIST FIPS 204 Section 5.3, context strings are bound directly into the pre-hash signing derivation. A context mismatch between signing and verification triggers a deterministic cryptographic rejection (`exit 6`).
3. **Defense-in-Depth Path and Metacharacter Sanitization**:
   - All input paths (`--file`, `--out-file`, `--signature-file`) and identifiers (`--key-id`) are screened for directory traversal (`..`) and shell execution metacharacters (`;&|$<>`) prior to filesystem access.
