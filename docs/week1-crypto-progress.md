# QuantumVault-PQC
## Week 1 – Cryptographic Engine Progress Report

### 1. Project Overview

QuantumVault-PQC is a Post-Quantum Cryptography based local filesystem project. The current development work focuses on establishing post-quantum shared secrets and testing authenticated data encryption.

### 2. Development Environment

| Component | Version / Platform |
|---|---|
| Operating System | Kali Linux |
| Programming Language | C |
| PQC Library | liboqs 0.16.0 |
| Cryptographic Library | OpenSSL 3.6.5 |
| Key Encapsulation Algorithm | ML-KEM-768 |
| Key Derivation | HKDF-SHA256 |
| Symmetric Encryption | AES-256-GCM |

### 3. Completed Implementation

The following cryptographic components have been implemented and tested:

1. Compiled and installed the liboqs library.
2. Verified ML-KEM-768 algorithm availability.
3. Implemented ML-KEM-768 keypair generation.
4. Implemented encapsulation and decapsulation.
5. Verified that both parties derive matching shared secrets.
6. Implemented HKDF-SHA256 key derivation.
7. Implemented AES-256-GCM encryption and decryption.
8. Verified successful plaintext recovery after decryption.
9. Tested ciphertext tampering detection through GCM authentication.

### 4. Automated Test Runner

Created the following script:

`scripts/run_crypto_tests.sh`

The script automates the compilation and execution of the existing cryptographic test programs.

**Execution command:**

```bash
./scripts/run_crypto_tests.sh
```

The test runner successfully completed the following operations:

- Checked required dependencies.
- Compiled the ML-KEM test.
- Compiled the AES-GCM test.
- Executed both cryptographic tests.
- Reported successful completion of all tests.

### 5. Test Results

| Test Case | Result |
|---|---|
| ML-KEM-768 keypair generation | PASS |
| ML-KEM encapsulation | PASS |
| ML-KEM decapsulation | PASS |
| Shared-secret equality | PASS |
| HKDF-SHA256 key derivation | PASS |
| AES-256-GCM encryption | PASS |
| AES-256-GCM decryption | PASS |
| Plaintext round-trip verification | PASS |
| Ciphertext tampering detection | PASS |

**Overall Test Status: PASS**

All tests executed through the automated test runner completed successfully.

### 6. Security Considerations

- ML-KEM-768 is used for post-quantum shared-secret establishment.
- HKDF-SHA256 derives an AES-256 key from the established shared secret.
- AES-256-GCM provides authenticated encryption.
- Modified ciphertext is rejected during authentication verification.
- Sensitive cryptographic buffers are cleansed in the relevant test implementations.

### 7. Current Scope and Limitations

The current implementation demonstrates cryptographic key establishment and authenticated encryption through in-memory tests.

The following project milestones remain part of subsequent development:

- Basic Rust FUSE filesystem implementation.
- Rust/C FFI integration.
- Transparent encryption during filesystem write operations.
- Transparent decryption during filesystem read operations.
- ML-DSA digital signature integration.
- Secure key lifecycle and memory protection.
- Final CLI integration and demonstration.

The current test programs are development and verification utilities, not a completed production filesystem encryption system.

### 8. Week 1 Progress Summary

The cryptographic foundation has been implemented and verified through individual tests and an automated test runner.

ML-KEM-768 key establishment, HKDF-SHA256 key derivation, AES-256-GCM authenticated encryption, and ciphertext tampering detection have passed their tests.

The next development phase will focus on filesystem implementation and integration of the cryptographic engine.
