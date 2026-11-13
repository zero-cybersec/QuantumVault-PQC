# ML-KEM-768 Key Generation Test

## Environment
- OS: Kali Linux
- Library: liboqs 0.16.0
- Language: C
- Algorithm: ML-KEM-768

## Implementation
- Initialize ML-KEM-768 using liboqs.
- Allocate public and secret key buffers.
- Generate a keypair using OQS_KEM_keypair.
- Check allocation and API errors.
- Cleanse secret key memory after the test.

## Test Results
- Key generation: Successful
- Public key size: 1184 bytes
- Secret key size: 2400 bytes
- Exit status: 0

## Security Note
Secret key material is not printed or persisted by this
smoke test. It is cleansed from memory after execution.

## Scope
This test covers keypair generation only.
Encapsulation and decapsulation are separate test tasks.
