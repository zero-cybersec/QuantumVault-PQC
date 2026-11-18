# ML-KEM-768 C API

## Overview
QuantumVault exposes ML-KEM-768 operations through a C API
for integration with Rust FFI.

## API Functions

### qv_mlkem768_keypair
Generates an ML-KEM-768 public and secret key pair.

### qv_mlkem768_encaps
Uses the recipient public key to generate a ciphertext
and a 32-byte shared secret.

### qv_mlkem768_decaps
Uses the recipient secret key and ciphertext to recover
the corresponding shared secret.

## Expected Buffer Sizes

- Public key: 1184 bytes
- Secret key: 2400 bytes
- Ciphertext: 1088 bytes
- Shared secret: 32 bytes

## Return Values

- 0: Success
- -1: Invalid arguments or insufficient buffer capacity
- -2: ML-KEM algorithm unavailable
- -3: Algorithm parameter mismatch
- -4: Cryptographic operation failure

## Verification

- ML-KEM keypair generation passed.
- Encapsulation and decapsulation passed.
- Shared-secret comparison passed.
- Four invalid-input tests passed.
- Combined AES and ML-KEM shared library built.
- All five API symbols exported.
- External C runtime linking test passed.

## Shared Library
Temporary test library: `/tmp/libquantumvault_crypto.so`
