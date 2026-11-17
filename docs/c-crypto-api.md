# QuantumVault C Crypto API

## Overview
This module exposes AES-256-GCM encryption and decryption
functions through a C interface for future Rust FFI integration.

## Header
`crypto-engine/include/quantumvault_crypto.h`

## API Functions

### qv_aes256gcm_encrypt
Encrypts plaintext using AES-256-GCM.

Inputs:
- key: 32-byte AES-256 key
- nonce: 12-byte GCM nonce
- plaintext: Input data
- plaintext_len: Input length
- ciphertext: Output ciphertext buffer
- tag: 16-byte authentication tag output

### qv_aes256gcm_decrypt
Decrypts and authenticates AES-256-GCM ciphertext.

Inputs:
- key: 32-byte AES-256 key
- nonce: 12-byte GCM nonce
- ciphertext: Encrypted data
- ciphertext_len: Ciphertext length
- tag: 16-byte authentication tag
- plaintext: Output plaintext buffer

## Verification
- Shared library compiled successfully.
- Both API symbols exported successfully.
- External C test program linked successfully.
- Both function addresses resolved at runtime.

## Shared Library
Temporary test library: `/tmp/libquantumvault_crypto.so`

## Security Notes
- Nonces must never be reused with the same AES-GCM key.
- Authentication tag verification is required before accepting plaintext.
- Secret keys must be securely managed and cleared after use.
