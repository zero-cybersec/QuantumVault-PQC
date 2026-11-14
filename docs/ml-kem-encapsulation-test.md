
# ML-KEM-768 Encapsulation and Decapsulation Test

## Environment
- OS: Kali Linux
- Language: C
- Library: liboqs 0.16.0
- Algorithm: ML-KEM-768

## Test Workflow
1. Generate ML-KEM-768 keypair.
2. Encapsulate using the public key.
3. Generate a shared secret and ciphertext.
4. Decapsulate using the secret key.
5. Compare both shared secrets.

## Test Results
- Keypair generation: PASS
- Encapsulation: PASS
- Decapsulation: PASS
- Shared-secret comparison: PASS
- Shared-secret length: 32 bytes

## Security Handling
- Secret key and shared-secret buffers are cleansed.
- Secret material is not printed.
- Test buffers are released after execution.

## Scope
This test verifies ML-KEM key establishment.
It does not implement file encryption.
