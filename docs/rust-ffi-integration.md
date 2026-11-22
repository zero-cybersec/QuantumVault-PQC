# QuantumVault-PQC: Rust FFI Integration

## 1. Objective
Verify that the Rust application can call the C cryptographic
API through the Foreign Function Interface (FFI).

## 2. Components
- Rust and Cargo
- C cryptographic API
- OpenSSL AES-256-GCM
- liboqs
- Shared library: libquantumvault_crypto.so

## 3. Exported C API
- qv_aes256gcm_encrypt
- qv_aes256gcm_decrypt
- qv_mlkem768_keypair
- qv_mlkem768_encaps
- qv_mlkem768_decaps

## 4. Rust FFI Coverage
The Rust test application declares and calls:
- qv_aes256gcm_encrypt
- qv_aes256gcm_decrypt

ML-KEM functions are exported by the C library but are not
yet called by this Rust test application.

## 5. Build Instructions

From the repository root:

gcc -Wall -Wextra -Werror -fPIC -shared \
-Icrypto-engine/include \
$(pkg-config --cflags liboqs openssl) \
crypto-engine/quantumvault_crypto.c \
crypto-engine/quantumvault_kem.c \
$(pkg-config --libs liboqs openssl) \
-o /tmp/libquantumvault_crypto.so

Then:

cd rust-ffi
cargo clean
cargo build

## 6. Run Instructions

LD_LIBRARY_PATH=/tmp:/usr/local/lib:$LD_LIBRARY_PATH cargo run

## 7. Test Results
- Rust called C AES-256-GCM encryption: PASS
- Rust called C AES-256-GCM decryption: PASS
- Decrypted plaintext matched original: PASS
- Modified authentication tag rejected: PASS
- Cargo build: PASS
- Program return code: 0

## 8. Important Notes
- The shared library in /tmp is temporary and must be rebuilt
  if it is removed.
- The current Rust FFI test uses fixed key and nonce values
  only for deterministic testing.
- Production use requires secure key handling and unique
  nonces for each encryption under a given key.
- ML-KEM Rust bindings are not covered by this test.

## 9. Conclusion
The Rust application successfully called the C AES-256-GCM
encryption and decryption functions through FFI.
All implemented Rust FFI tests passed.
