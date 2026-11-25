# QuantumVault (`qv`) — CLI Specification & Test Plan: Post-Quantum Signature Generation and Verification

**Project:** QuantumVault — Zero-Trust, Post-Quantum Cryptographic Secrets Management Vault  
**Subsystem:** Cryptographic Operations Layer — Digital Signatures (`qv sign`, `qv verify`)  
**Milestone:** Week 3 Cryptographic CLI Design & Test Specification     
**Status:** Approved Specification & Test Harness Architecture  

---

## 1. Executive Summary & Architectural Overview

The QuantumVault digital signature subsystem provides hardware-grade, zero-trust cryptographic signature generation and verification. To ensure long-term resilience against cryptanalytically relevant quantum computers (CRQCs), the subsystem deprecates all legacy public-key signature algorithms (RSA, ECDSA, Ed25519) and standardizes on **NIST FIPS 204 Module-Lattice-Based Digital Signature Algorithm (ML-DSA)**, supplemented by high-assurance stateful/hash-based algorithms (**Falcon** and **SPHINCS+**).

This document establishes:
1. **CLI Command Architecture**: Ergonomic top-level commands (`qv sign`, `qv verify`) and their administrative equivalents (`qv key sign`, `qv key verify`).
2. **Command Parameter & Flag Specifications**: Precise syntax, validation constraints, default behaviors, environment variable bindings, and structured output schemas (Text, JSON, YAML).
3. **Defense-in-Depth Sanitization**: Input boundary guards against path traversal, shell injection, algorithm downgrade attacks, and context-string buffer overruns.
4. **Comprehensive Test Suite**: A 32-case verification matrix comprising 16 positive scenarios and 16 negative/adversarial scenarios mapped to standardized POSIX exit codes (`0`, `2`, `3`, `4`, `5`, `6`, `7`).
5. **Operational Test Playbooks**: End-to-end bash scenarios for CI/CD artifact signing, Unix pipe streaming, and offline public-key verification.

---

## 2. CLI Command Hierarchy & Design Principles

### 2.1 Command Tree

The signature workflow is integrated into the QuantumVault command hierarchy at two complementary levels:

```
quantumvault (qv)
├── sign                       # High-frequency developer/DevOps signing command
│   ├── [PAYLOAD_PATH]         # Positional payload path (or '-' for stdin)
│   ├── --key-id (-k)          # Vault signing key identifier (required)
│   ├── --data (-d)            # Raw string payload alternative
│   ├── --file (-f)            # Explicit input file path
│   ├── --algorithm (-a)       # Post-quantum signature algorithm (default: ml-dsa-65)
│   ├── --hash (-H)            # Pre-hashing function (sha3-256, sha512, none)
│   ├── --context (-c)         # NIST FIPS 204 domain-separation context string
│   ├── --encoding (-e)        # Signature serialization format (base64, hex, raw)
│   ├── --out-file (-o)        # Path to write detached signature
│   └── --detached / --attached# Detached vs. bundled container signature
│
├── verify                     # Signature verification command
│   ├── [PAYLOAD_PATH]         # Positional payload path (or '-' for stdin)
│   ├── --signature (-s)       # Inline signature string (base64/hex) or '@file'
│   ├── --signature-file       # Path to detached signature file
│   ├── --key-id (-k)          # Internal vault key identifier
│   ├── --public-key (-p)      # Standalone public key file or string
│   ├── --data (-d)            # Raw string payload alternative
│   ├── --file (-f)            # Explicit input file path
│   ├── --algorithm (-a)       # Expected PQC signature algorithm
│   ├── --context (-c)         # Expected domain-separation context string
│   └── --encoding (-e)        # Expected signature encoding (base64, hex)
│
└── key                        # Administrative key lifecycle management
    ├── sign                   # Alias pointing to top-level 'qv sign'
    └── verify                 # Alias pointing to top-level 'qv verify'
```

### 2.2 Core Design Principles

1. **Ergonomic Developer Experience**: Mirror standard Unix tools (`openssl dgst`, `cosign`, `minisign`) while abstracting complex post-quantum lattice mathematics.
2. **Stream & Pipe First**: Support Unix pipeline integration (`cat artifact.tar.gz | qv sign -k release-key -`) without requiring temporary disk writes.
3. **Cryptographic Agility without Classical Fallback**: Support multiple NIST post-quantum parameter sets, while strictly prohibiting classical ciphers (e.g., `rsa2048`, `secp256r1`) at the validation layer.
4. **Domain Separation by Default**: Full support for NIST FIPS 204 context strings to prevent cross-protocol signature substitution attacks.
5. **Zero-Trust Memory Hygiene**: Sensitive private key handles and deterministic signing nonces are held in locked memory (`mlock`) and zeroized immediately post-execution.

---

## 3. Cryptographic Signature Algorithms & Parameter Sets

QuantumVault enforces an allowlist of approved post-quantum signature algorithms. Classical algorithms are rejected at the argument validation boundary before reaching the cryptographic core.

| Algorithm Identifier | Mathematical Family | NIST Security Category | Public Key Size (Bytes) | Signature Size (Bytes) | Primary Use Case |
|---|---|:---:|:---:|:---:|---|
| `ml-dsa-65` *(Default)* | Module-Lattice (FIPS 204 / Dilithium3) | Category 3 (AES-192 equivalent) | 1,952 | 3,309 | General-purpose signatures, TLS, software artifacts |
| `ml-dsa-87` | Module-Lattice (FIPS 204 / Dilithium5) | Category 5 (AES-256 equivalent) | 2,592 | 4,627 | High-assurance government, root PKI, financial ledgers |
| `falcon-512` | NTRU Lattice (Fast Fourier) | Category 1 (AES-128 equivalent) | 897 | 690 | Bandwidth-constrained environments, small payload signatures |
| `falcon-1024` | NTRU Lattice (Fast Fourier) | Category 5 (AES-256 equivalent) | 1,793 | 1,330 | High-security bandwidth-constrained signatures |
| `sphincs+-sha2-128s` | Stateless Hash-Based (SPHINCS+) | Category 1 (Conservative) | 32 | 7,856 | Ultra-conservative long-term cold storage, root CA |
| `sphincs+-shake-256s` | Stateless Hash-Based (SPHINCS+) | Category 5 (Conservative) | 64 | 29,792 | Ultra-high security root-level digital notarization |
| `hybrid-mldsa65-ed25519`| Dual Hybrid (PQC + Classical) | Hybrid Category 3 + Ed25519 | 1,984 | 3,373 | Transitional enterprise environments requiring dual compliance |

---

## 4. Command Arguments & Expected Outputs

### 4.1 Command: `qv sign`

#### 4.1.1 Parameter & Flag Specification

| Option / Flag | Short | Type | Default | Required | Environment Variable | Description & Validation Rules |
|---|:---:|---|---|:---:|---|---|
| `[PAYLOAD_PATH]` | — | Path / `-` | `None` | No | — | Positional path to target file. `-` indicates STDIN stream. |
| `--key-id` | `-k` | String | `None` | **Yes** | `QV_KEY_ID` | Identifier of the vault signing key. Matches `^[a-zA-Z0-9][a-zA-Z0-9._-]{1,63}$`. |
| `--data` | `-d` | String | `None` | No | — | Inline payload string. Mutually exclusive with `--file` / STDIN. |
| `--file` | `-f` | Path | `None` | No | — | Path to payload file. Must exist; traversal (`..`) forbidden. |
| `--algorithm` | `-a` | Choice | `ml-dsa-65` | No | `QV_SIGNING_ALGO` | Signature suite. Must be in `VALID_PQC_ALGORITHMS`. |
| `--hash` | `-H` | Choice | `sha3-256` | No | — | Pre-hash function: `sha256`, `sha384`, `sha512`, `sha3-256`, `sha3-512`, `none`. |
| `--context` | `-c` | String | `""` | No | — | NIST FIPS 204 domain-separation string. Max 255 bytes UTF-8. |
| `--encoding` | `-e` | Choice | `base64` | No | — | Signature output serialization: `base64`, `hex`, `raw`. |
| `--out-file` | `-o` | Path | `None` | No | — | Target path to write detached signature. Traversal forbidden. |
| `--detached` / `--attached` | — | Flag | `--detached` | No | — | Generate detached signature vs. signed wrapper container. |

#### 4.1.2 Output Formats

##### A. Standard Human-Readable Text Output (`--format text` / Default)
```text
QuantumVault Post-Quantum Signature Generated
============================================================
  Key ID:          root-signing-key
  Key Version:     1
  Algorithm:       ML-DSA-65 (NIST FIPS 204)
  Hash Function:   SHA3-256
  Encoding:        base64
  Context:         release-v2026.1
  Timestamp:       2026-11-25T19:30:00Z
  Signature Size:  3,309 bytes (4,412 Base64 characters)
  PQC Fingerprint: SHA256:7f9b8c2d1e0a...
------------------------------------------------------------
Signature:
MIIC3jCCAc6gAwIBAgIIb/0q1t5X8h0wDQYJKoZIhvcNAQELBQAw...[truncated]...
============================================================
```

##### B. Structured JSON Output (`--format json`)
```json
{
  "status": "success",
  "key_id": "root-signing-key",
  "key_version": 1,
  "algorithm": "ml-dsa-65",
  "hash_function": "sha3-256",
  "encoding": "base64",
  "context": "release-v2026.1",
  "signature": "MIIC3jCCAc6gAwIBAgIIb/0q1t5X8h0wDQYJKoZIhvcNAQELBQAw...",
  "signature_bytes": 3309,
  "fingerprint": "SHA256:7f9b8c2d1e0a3f5c7b9a1d3e5f7a9b1c3d5e7f9a1b3c5d7e9f1a3b5c7d9e1f3a",
  "timestamp": "2026-11-25T19:30:00Z"
}
```

##### C. Structured YAML Output (`--format yaml`)
```yaml
status: success
key_id: root-signing-key
key_version: 1
algorithm: ml-dsa-65
hash_function: sha3-256
encoding: base64
context: release-v2026.1
signature: MIIC3jCCAc6gAwIBAgIIb/0q1t5X8h0wDQYJKoZIhvcNAQELBQAw...
signature_bytes: 3309
fingerprint: SHA256:7f9b8c2d1e0a3f5c7b9a1d3e5f7a9b1c3d5e7f9a1b3c5d7e9f1a3b5c7d9e1f3a
timestamp: '2026-11-25T19:30:00Z'
```

---

### 4.2 Command: `qv verify`

#### 4.2.1 Parameter & Flag Specification

| Option / Flag | Short | Type | Default | Required | Description & Validation Rules |
|---|:---:|---|---|:---:|---|
| `[PAYLOAD_PATH]` | — | Path / `-` | `None` | No | Positional path to target file. `-` indicates STDIN. |
| `--signature` | `-s` | String | `None` | Conditional | Raw signature string or `@<file>` reference. Required if `--signature-file` omitted. |
| `--signature-file` | — | Path | `None` | Conditional | Path to detached signature file. Traversal forbidden. |
| `--key-id` | `-k` | String | `None` | Conditional | Vault signing key ID for key lookup. Required unless `--public-key` given. |
| `--public-key` | `-p` | Path / String | `None` | Conditional | Standalone public key file or string for offline verification. |
| `--data` | `-d` | String | `None` | No | Inline payload string. Mutually exclusive with `--file`. |
| `--file` | `-f` | Path | `None` | No | Explicit payload file path. |
| `--algorithm` | `-a` | Choice | Auto | No | Signature algorithm. Auto-detected from key metadata if omitted. |
| `--context` | `-c` | String | `""` | No | Expected domain separation context string. Must match signing context. |
| `--encoding` | `-e` | Choice | `base64` | No | Encoding of the supplied signature: `base64`, `hex`. |

#### 4.2.2 Output Formats & Verification Results

##### A. Verification Success (`EXIT_SUCCESS` / Code 0)

**Text Format:**
```text
Signature Verification Result
============================================================
  Status:          VERIFIED (Valid Cryptographic Signature)
  Key Identifier:  root-signing-key (Version 1)
  Algorithm:       ML-DSA-65 (NIST FIPS 204)
  Context:         release-v2026.1
  Verification At: 2026-11-25T19:32:15Z
============================================================
```

**JSON Format (`--format json`):**
```json
{
  "status": "valid",
  "verified": true,
  "key_id": "root-signing-key",
  "key_version": 1,
  "algorithm": "ml-dsa-65",
  "context": "release-v2026.1",
  "timestamp": "2026-11-25T19:32:15Z"
}
```

##### B. Verification Failure (`EXIT_INTEGRITY_FAIL` / Code 6)

**Text Format (Standard Error):**
```text
Error [QV_EXIT_INTEGRITY_FAIL]: Signature verification FAILED.
The cryptographic signature does not match the provided payload, key, or context.
Details: Lattice verification equation failed: w1 != HighBits(Az - c*t1*2^d, 2*gamma2)
```

**JSON Format (`--format json`):**
```json
{
  "status": "invalid",
  "verified": false,
  "error_code": 6,
  "error_name": "QV_EXIT_INTEGRITY_FAIL",
  "message": "Signature verification failed: cryptographic integrity mismatch.",
  "key_id": "root-signing-key",
  "algorithm": "ml-dsa-65"
}
```

---

## 5. POSIX Exit Code Mappings

All signature commands adhere strictly to QuantumVault's standardized exit code specification:

| Code | Constant Name | CLI Error Headline | Trigger Condition |
|:---:|---|---|---|
| **0** | `EXIT_SUCCESS` | `OK` | Signature generated successfully or signature successfully verified. |
| **1** | `EXIT_GENERAL_ERROR` | `QV_EXIT_GENERAL_ERROR` | Unhandled OS exceptions, I/O read errors, out of memory. |
| **2** | `EXIT_INVALID_ARGS` | `QV_EXIT_INVALID_ARGS` | Missing required flags, path traversal (`..`), injection characters, context >255B, malformed Base64. |
| **3** | `EXIT_VAULT_LOCKED` | `QV_EXIT_VAULT_LOCKED` | Signing requested while vault is in sealed/locked state. |
| **4** | `EXIT_SECRET_NOT_FOUND` | `QV_EXIT_SECRET_NOT_FOUND` | Specified `--key-id` does not exist in the active vault catalog. |
| **5** | `EXIT_ACCESS_DENIED` | `QV_EXIT_ACCESS_DENIED` | Authenticated identity lacks RBAC `sign` capability on target key. |
| **6** | `EXIT_INTEGRITY_FAIL` | `QV_EXIT_INTEGRITY_FAIL` | Signature verification failed: payload modified, signature corrupted, or context mismatched. |
| **7** | `EXIT_PQC_ALGO_ERROR` | `QV_EXIT_PQC_ALGO_ERROR` | Classical algorithm requested (e.g. `rsa2048`) or unsupported PQC parameter set. |

---

## 6. Comprehensive Test Suite Matrix

### 6.1 Positive Test Cases (`TC-SIG-POS-01` to `TC-SIG-POS-16`)

| Test ID | Command / Focus | Target Subsystem & Parameters | Execution Command Syntax | Expected Result & Output Assertions | Exit Code |
|---|---|---|---|---|:---:|
| `TC-SIG-POS-01` | String Payload Signing | `qv sign` with `-d` string and default `ml-dsa-65` | `qv sign -k test-key -d "hello world"` | Generates 3,309-byte ML-DSA-65 signature in Base64; prints metadata header. | **0** |
| `TC-SIG-POS-02` | Valid Signature Verification | `qv verify` inline signature string | `qv verify -k test-key -d "hello world" -s "<SIG_B64>"` | Output reports `VERIFIED (Valid Cryptographic Signature)`; `verified: true`. | **0** |
| `TC-SIG-POS-03` | File Payload Signing | `qv sign` with `--file` positional argument | `qv sign -k test-key /tmp/payload.bin` | Reads file bytes without modification; outputs Base64 signature. | **0** |
| `TC-SIG-POS-04` | Detached File Verification | `qv verify` with `--signature-file` | `qv verify -k test-key /tmp/payload.bin --signature-file /tmp/payload.sig` | Validates detached signature against file bytes on disk. | **0** |
| `TC-SIG-POS-05` | STDIN Streaming Pipe Signing | `qv sign` from Unix pipeline (`-`) | `echo "pipeline-data" \| qv sign -k test-key -` | Reads entire stream via STDIN; outputs valid signature. | **0** |
| `TC-SIG-POS-06` | High-Security ML-DSA-87 | `qv sign` with `-a ml-dsa-87` | `qv sign -k root-key -d "critical" -a ml-dsa-87` | Generates 4,627-byte ML-DSA-87 signature; verifies algorithm tag in metadata. | **0** |
| `TC-SIG-POS-07` | Fast-Verification Falcon-512 | `qv sign` with `-a falcon-512` | `qv sign -k iot-key -d "telemetry" -a falcon-512` | Generates 690-byte compact Falcon-512 signature. | **0** |
| `TC-SIG-POS-08` | Stateless SPHINCS+-SHA2-128s | `qv sign` with `-a sphincs+-sha2-128s` | `qv sign -k cold-key -d "root-cert" -a sphincs+-sha2-128s` | Generates 7,856-byte SPHINCS+ hash-based signature. | **0** |
| `TC-SIG-POS-09` | Domain Separation Context | `qv sign` with `--context "git-commit"` | `qv sign -k test-key -d "commit 9a1" -c "git-commit"` | Context string embedded in FIPS 204 signing computation. | **0** |
| `TC-SIG-POS-10` | Context Match Verification | `qv verify` with matching `--context` | `qv verify -k test-key -d "commit 9a1" -s "<SIG>" -c "git-commit"` | Verification succeeds when context matches exactly. | **0** |
| `TC-SIG-POS-11` | Output to File (`--out-file`) | `qv sign` writing to destination file | `qv sign -k test-key -d "data" -o /tmp/sig.out` | Writes signature directly to file; stdout reports file creation. | **0** |
| `TC-SIG-POS-12` | Hexadecimal Encoding | `qv sign` with `--encoding hex` | `qv sign -k test-key -d "data" -e hex` | Outputs lowercase hexadecimal signature string; verify parses hex cleanly. | **0** |
| `TC-SIG-POS-13` | Structured JSON Output | `qv sign --format json` | `qv sign -k test-key -d "data" --format json` | Validates against JSON Schema; contains `status`, `signature`, `key_id`. | **0** |
| `TC-SIG-POS-14` | Structured YAML Output | `qv sign --format yaml` | `qv sign -k test-key -d "data" --format yaml` | Outputs valid YAML parseable by PyYAML. | **0** |
| `TC-SIG-POS-15` | Offline Public Key Verify | `qv verify` with standalone `--public-key` | `qv verify -p /tmp/test-pub.pem -d "data" -s "<SIG>"` | Verifies signature without needing active vault connection. | **0** |
| `TC-SIG-POS-16` | Empty Payload Signing | `qv sign` with empty byte sequence `b""` | `qv sign -k test-key -d ""` | Handles 0-byte payload safely; produces valid verifiable signature. | **0** |

---

### 6.2 Negative & Security Failure Test Cases (`TC-SIG-NEG-01` to `TC-SIG-NEG-16`)

| Test ID | Failure Injection Scope | Specific Attack Vector / Misconfiguration | Execution Command Syntax | Expected Error & Security Behavior | Exit Code |
|---|---|---|---|---|:---:|
| `TC-SIG-NEG-01` | Tampered Signature | Single bit-flip mutation in Base64 signature string | `qv verify -k test-key -d "data" -s "<CORRUPTED_SIG>"` | Rejects with `QV_EXIT_INTEGRITY_FAIL`; w1 verification fails. | **6** |
| `TC-SIG-NEG-02` | Tampered Payload | 1 byte modified in payload with original signature | `qv verify -k test-key -d "dataX" -s "<VALID_SIG>"` | Rejects with `QV_EXIT_INTEGRITY_FAIL`; payload digest mismatch. | **6** |
| `TC-SIG-NEG-03` | Key Substitution Attack | Valid signature verified against different key ID | `qv verify -k wrong-key -d "data" -s "<VALID_SIG>"` | Rejects with `QV_EXIT_INTEGRITY_FAIL`; public key mismatch. | **6** |
| `TC-SIG-NEG-04` | Context String Mismatch | Sign with `--context A`, verify with `--context B` | `qv verify -k test-key -d "data" -s "<SIG>" -c "ctxB"` | Rejects with `QV_EXIT_INTEGRITY_FAIL`; context binding failure. | **6** |
| `TC-SIG-NEG-05` | Missing Payload Input | Neither `--data`, `--file`, nor STDIN supplied | `qv sign -k test-key` | Rejects with `QV_EXIT_INVALID_ARGS`; prompts for payload source. | **2** |
| `TC-SIG-NEG-06` | Missing Signature Input | `qv verify` called without `--signature` / file | `qv verify -k test-key -d "data"` | Rejects with `QV_EXIT_INVALID_ARGS`; prompts for signature. | **2** |
| `TC-SIG-NEG-07` | Missing Verification Key | `qv verify` with neither `--key-id` nor `--public-key`| `qv verify -d "data" -s "<SIG>"` | Rejects with `QV_EXIT_INVALID_ARGS`; requires verification key. | **2** |
| `TC-SIG-NEG-08` | Non-Existent Key ID | Target `--key-id` not present in vault catalog | `qv sign -k non-existent-key-99 -d "data"` | Rejects with `QV_EXIT_SECRET_NOT_FOUND`; key not located. | **4** |
| `TC-SIG-NEG-09` | Path Traversal in `--file` | Traversal path `../../etc/shadow` in input | `qv sign -k test-key -f "../../etc/shadow"` | Rejects with `QV_EXIT_INVALID_ARGS`; directory traversal blocked. | **2** |
| `TC-SIG-NEG-10` | Path Traversal in `--out-file` | Traversal path `../../../root/.bashrc` | `qv sign -k test-key -d "x" -o "../../../root/.sig"` | Rejects with `QV_EXIT_INVALID_ARGS`; out-file traversal blocked. | **2** |
| `TC-SIG-NEG-11` | Shell Injection in Key ID | Metacharacters `; rm -rf /` in `--key-id` | `qv sign -k "key;rm -rf /" -d "data"` | Rejects with `QV_EXIT_INVALID_ARGS`; forbidden characters regex. | **2** |
| `TC-SIG-NEG-12` | Classical Algorithm Downgrade | Deprecated cipher requested (`--algorithm rsa2048`) | `qv sign -k test-key -d "data" -a rsa2048` | Rejects with `QV_EXIT_INVALID_ARGS` / `QV_EXIT_PQC_ALGO_ERROR`. | **2** / **7** |
| `TC-SIG-NEG-13` | Context Length Limit Exceeded | Context string > 255 bytes (FIPS 204 limit) | `qv sign -k test-key -d "data" -c "$(python3 -c 'print("A"*256)')"` | Rejects with `QV_EXIT_INVALID_ARGS`; context length exceeds 255B. | **2** |
| `TC-SIG-NEG-14` | Malformed Base64 Encoding | Corrupted characters (`!@#$%`) in signature | `qv verify -k test-key -d "data" -s "invalid!@#base64"` | Rejects with `QV_EXIT_INVALID_ARGS`; invalid Base64 decoding. | **2** |
| `TC-SIG-NEG-15` | Vault Sealed / Locked | Sign requested while vault session is locked | `qv sign -k test-key -d "data"` *(locked state)* | Rejects with `QV_EXIT_VAULT_LOCKED`; prompts to unseal vault. | **3** |
| `TC-SIG-NEG-16` | Truncated Signature Buffer | Signature length shorter than algorithm minimum | `qv verify -k test-key -d "data" -s "dGVzdA=="` | Rejects with `QV_EXIT_INVALID_ARGS` or `QV_EXIT_INTEGRITY_FAIL`. | **2** / **6** |

---

## 7. Documented CLI Test Scenarios & Playbooks

### Playbook 1: Software Artifact Signing in CI/CD Pipelines

```bash
#!/usr/bin/env bash
set -euo pipefail

# 1. Build release artifact
echo "QuantumVault v0.2.0 binary artifact" > /tmp/qv-release.tar.gz

# 2. Generate detached post-quantum signature with ML-DSA-65 and context binding
echo "==> Signing release artifact..."
qv sign \
  --key-id release-signer-2026 \
  --algorithm ml-dsa-65 \
  --context "qv-release-v0.2.0" \
  --file /tmp/qv-release.tar.gz \
  --out-file /tmp/qv-release.tar.gz.sig

# 3. Verify artifact integrity before distribution
echo "==> Verifying signature..."
qv verify \
  --key-id release-signer-2026 \
  --context "qv-release-v0.2.0" \
  --file /tmp/qv-release.tar.gz \
  --signature-file /tmp/qv-release.tar.gz.sig

echo "==> CI/CD Artifact Signing & Verification Successful!"
```

### Playbook 2: Unix Pipeline Streaming & Standard Input Signing

```bash
#!/usr/bin/env bash
set -euo pipefail

# Stream dynamic system payload through stdin to produce JSON output
SIG_JSON=$(uname -a | qv sign -k telemetry-key --format json -)

# Extract signature from JSON
SIGNATURE=$(echo "$SIG_JSON" | python3 -c "import sys, json; print(json.load(sys.stdin)['signature'])")

# Verify piped payload against extracted signature
uname -a | qv verify -k telemetry-key --signature "$SIGNATURE" -
echo "Pipeline streaming verification confirmed."
```

### Playbook 3: Negative Tamper Detection Verification

```bash
#!/usr/bin/env bash
set +e

echo "Original Payload" > /tmp/payload.txt
qv sign -k test-key -f /tmp/payload.txt -o /tmp/payload.sig

# Simulate malicious tampering in payload
echo "Tampered Payload" > /tmp/payload.txt

# Verification must fail with Exit Code 6
qv verify -k test-key -f /tmp/payload.txt --signature-file /tmp/payload.sig
EXIT_CODE=$?

if [ $EXIT_CODE -eq 6 ]; then
  echo "SUCCESS: Tampering detected cleanly with QV_EXIT_INTEGRITY_FAIL (6)."
else
  echo "FAILURE: Unexpected exit code $EXIT_CODE (expected 6)."
  exit 1
fi
```

---

## 8. Automated Test Harness Implementation Architecture

For Week 3 integration into `tests/test_signature_commands.py`, the following ClickRunner test harness will be deployed:

```python
"""Automated Unit & Integration Tests for 'qv sign' and 'qv verify'."""
import unittest
from click.testing import CliRunner
from quantumvault.cli import cli
from quantumvault.constants import (
    EXIT_SUCCESS,
    EXIT_INVALID_ARGS,
    EXIT_VAULT_LOCKED,
    EXIT_SECRET_NOT_FOUND,
    EXIT_INTEGRITY_FAIL,
)

class TestSignatureCommands(unittest.TestCase):
    def setUp(self):
        self.runner = CliRunner()

    def test_sign_valid_string_default_algorithm(self):
        """TC-SIG-POS-01: Sign valid string with ML-DSA-65."""
        res = self.runner.invoke(cli, ["sign", "-k", "test-key", "-d", "hello world"])
        self.assertEqual(res.exit_code, EXIT_SUCCESS)
        self.assertIn("ML-DSA-65", res.output)
        self.assertIn("Signature:", res.output)

    def test_verify_valid_signature(self):
        """TC-SIG-POS-02: Verify valid signature."""
        sign_res = self.runner.invoke(cli, ["--format", "json", "sign", "-k", "test-key", "-d", "hello world"])
        self.assertEqual(sign_res.exit_code, EXIT_SUCCESS)
        import json
        sig = json.loads(sign_res.output)["signature"]

        verify_res = self.runner.invoke(cli, ["verify", "-k", "test-key", "-d", "hello world", "-s", sig])
        self.assertEqual(verify_res.exit_code, EXIT_SUCCESS)
        self.assertIn("VERIFIED", verify_res.output)

    def test_verify_tampered_payload_fails(self):
        """TC-SIG-NEG-02: Tampered payload rejected with EXIT_INTEGRITY_FAIL."""
        sign_res = self.runner.invoke(cli, ["--format", "json", "sign", "-k", "test-key", "-d", "hello world"])
        import json
        sig = json.loads(sign_res.output)["signature"]

        verify_res = self.runner.invoke(cli, ["verify", "-k", "test-key", "-d", "tampered text", "-s", sig])
        self.assertEqual(verify_res.exit_code, EXIT_INTEGRITY_FAIL)
        self.assertIn("QV_EXIT_INTEGRITY_FAIL", verify_res.output)

    def test_sign_missing_payload_fails(self):
        """TC-SIG-NEG-05: Missing payload rejected with EXIT_INVALID_ARGS."""
        res = self.runner.invoke(cli, ["sign", "-k", "test-key"])
        self.assertEqual(res.exit_code, EXIT_INVALID_ARGS)
        self.assertIn("Error", res.output)

    def test_sign_classical_algorithm_rejected(self):
        """TC-SIG-NEG-12: Classical RSA-2048 rejected at validation."""
        res = self.runner.invoke(cli, ["sign", "-k", "test-key", "-d", "test", "-a", "rsa2048"])
        self.assertEqual(res.exit_code, EXIT_INVALID_ARGS)
        self.assertIn("Invalid PQC algorithm", res.output)
```

---

## 9. Integration Checklist & Next Steps for Implementation

| Milestone Step | Target File(s) | Description | Priority |
|---|---|---|:---:|
| **Step 1: Validation Types** | `quantumvault/validation.py` | Add `SignatureParamType`, `ContextStringParamType`, and signature encoding validators. | P0 |
| **Step 2: Sign Command** | `quantumvault/commands/sign_cmd.py` | Implement `qv sign` with options, formatting, and mock/catalog PQC engine. | P0 |
| **Step 3: Verify Command** | `quantumvault/commands/verify_cmd.py` | Implement `qv verify` with integrity validation, tampering detection, and exit code 6. | P0 |
| **Step 4: CLI Registration** | `quantumvault/cli.py` | Register `sign` and `verify` top-level commands and aliases under `key_group`. | P0 |
| **Step 5: Automated Test Suite** | `tests/test_signature_commands.py` | Implement full 32-case unit and subprocess test suite. Target 230+ total tests. | P0 |
| **Step 6: C/Rust Integration** | `src/crypto/pqc_sign.c` / Rust crate | Bind to `liboqs` / NIST FIPS 204 C reference implementation. | P1 |

---
