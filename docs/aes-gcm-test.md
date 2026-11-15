# AES-256-GCM Encryption Test

## Environment
- OS: Kali Linux
- Language: C
- Crypto library: OpenSSL
- PQC library: liboqs 0.16.0

## Test Workflow
1. Generate ML-KEM-768 keypair.
2. Perform encapsulation and decapsulation.
3. Verify shared-secret equality.
4. Derive AES-256 key using HKDF-SHA256.
5. Generate a random 96-bit nonce.
6. Encrypt a test message using AES-256-GCM.
7. Decrypt and verify the original plaintext.
8. Modify ciphertext and verify authentication failure.

## Results
- ML-KEM shared secret: PASS
- HKDF-SHA256 derivation: PASS
- AES-256-GCM encryption: PASS
- AES-256-GCM decryption: PASS
- Plaintext round-trip: PASS
- Ciphertext tampering detection: PASS

## Security Notes
- Secret material is not printed.
- Secret buffers are cleansed after use.
- A fresh random nonce is generated for the test.
- Modified ciphertext is rejected during authentication.

## Scope
This is an in-memory cryptographic test.
It does not yet implement filesystem encryption.
