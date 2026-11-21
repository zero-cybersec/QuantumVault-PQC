# QuantumVault Rust FFI Integration

## Objective
Verify that Rust can call the C cryptographic API through
the Foreign Function Interface (FFI).

## Components
- Rust 1.97.1
- Cargo 1.97.1
- C crypto API
- OpenSSL AES-256-GCM
- liboqs shared library

## Shared Library
Temporary library:
`/tmp/libquantumvault_crypto.so`

Exported functions:
- qv_aes256gcm_encrypt
- qv_aes256gcm_decrypt
- qv_mlkem768_keypair
- qv_mlkem768_encaps
- qv_mlkem768_decaps

## Verification Results
- Rust called C AES-256-GCM encryption: PASS
- Rust called C AES-256-GCM decryption: PASS
- Decrypted plaintext matched original: PASS
- Modified authentication tag rejected: PASS
- Cargo build completed successfully: PASS

## Execution
`cargo build`

`LD_LIBRARY_PATH=/tmp:/usr/local/lib:$LD_LIBRARY_PATH cargo run`

## Conclusion
Rust successfully called the C AES-256-GCM API through FFI.
Encryption, decryption, plaintext verification and
authentication-tag rejection tests passed.
