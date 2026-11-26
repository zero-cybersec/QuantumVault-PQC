# ML-DSA-65 Reusable C API

## Overview
Implemented reusable ML-DSA-65 digital signature functions
using liboqs 0.16.0.

## API Functions
- qv_mldsa65_keypair()
- qv_mldsa65_sign()
- qv_mldsa65_verify()

## Parameter Sizes
- Public key: 1952 bytes
- Secret key: 4032 bytes
- Signature: 3309 bytes

## Functional Testing
- Keypair generation
- Signature generation
- Valid signature verification
- Modified message rejection

## Invalid Input Testing
- NULL pointer validation
- Insufficient buffer capacity
- Invalid secret key length
- Invalid signature length
- Invalid public key length

## Integration
ML-DSA-65 successfully integrated with:
- AES-256-GCM
- ML-KEM-768

Shared library:
`/tmp/libquantumvault_crypto.so`

All 8 expected API symbols verified using nm.
