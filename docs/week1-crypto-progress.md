# QuantumVault-PQC — Week 1 Crypto Progress

## Environment
- OS: Kali Linux
- Language: C
- PQC library: liboqs 0.16.0
- OpenSSL: 3.6.5

## Completed Implementation
1. Compiled and installed liboqs.
2. Verified ML-KEM-768 availability.
3. Implemented ML-KEM keypair generation test.
4. Implemented encapsulation and decapsulation test.
5. Verified shared-secret equality.
6. Implemented HKDF-SHA256 key derivation.
7. Implemented AES-256-GCM encryption and decryption test.
8. Verified plaintext round-trip.
9. Verified ciphertext tampering detection.

## Test Results
- ML-KEM-768 keypair: PASS
- Encapsulation: PASS
- Decapsulation: PASS
- Shared-secret comparison: PASS
- HKDF-SHA256: PASS
- AES-256-GCM encryption: PASS
- AES-256-GCM decryption: PASS
- Ciphertext authentication: PASS

## Scope and Limitations
The current implementation demonstrates cryptographic key establishment and authenticated encryption in memory.

Transparent filesystem encryption and Rust/C FFI integration are planned for subsequent development weeks.
