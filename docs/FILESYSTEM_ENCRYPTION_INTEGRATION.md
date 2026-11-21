# QuantumVault-PQC: Filesystem Encryption Integration

## 1. Objective

Integrate the Rust FUSE virtual filesystem with a native C
cryptographic library through Rust Foreign Function Interface (FFI).

The objective is to encrypt file contents before storage and
decrypt them when file data is requested.

## 2. System Architecture

FUSE Filesystem
       |
       v
File Read / Write Operations
       |
       v
CryptoProvider Interface
       |
       v
Rust FFI Adapter
       |
       v
Native C Crypto Library
       |
       v
OpenSSL AES-256-GCM

## 3. Encryption Flow

1. User writes plaintext data into a virtual filesystem file.
2. FUSE receives the write request.
3. FsState retrieves and decrypts existing file contents.
4. The updated plaintext is passed to CryptoProvider.
5. Rust FFI calls the native qv_crypto_encrypt() function.
6. The C library encrypts data using AES-256-GCM.
7. Ciphertext is stored in VirtualFile.data.
8. Plaintext file size is maintained separately for FUSE metadata.

## 4. Decryption Flow

1. User requests file data through FUSE.
2. The filesystem retrieves stored ciphertext.
3. CryptoProvider calls the native qv_crypto_decrypt() function.
4. AES-GCM authentication is verified.
5. The decrypted plaintext is checked against the expected size.
6. The requested data range is returned through the FUSE read callback.

Authentication failures are reported as filesystem I/O errors.

## 5. Rust FFI Interface

The following native functions are declared:

- qv_crypto_init()
- qv_crypto_encrypt()
- qv_crypto_decrypt()
- qv_crypto_free()

The native context is represented using an opaque pointer.

The Rust FfiCryptoProvider manages the native context and
releases it through its Drop implementation.

## 6. Native C Cryptographic Implementation

The native C implementation uses OpenSSL EVP APIs
with AES-256-GCM authenticated encryption.

Ciphertext layout:

- 12-byte random nonce
- 16-byte authentication tag
- Encrypted payload

Total encryption overhead: 28 bytes.

The encryption function returns the number of output bytes.
The decryption function verifies the authentication tag before
returning plaintext.

The native context generates a random 256-bit key during
initialization and cleanses key memory when released.

## 7. C Library Linking

The Cargo build script supports:

- Compiling the native C crypto source.
- Creating and linking the static QuantumVault crypto library.
- Linking the OpenSSL crypto library.

The Rust application uses the native C library through FFI.

## 8. Filesystem Integration

The following operations support encrypted storage:

- File creation initializes encrypted empty content.
- File writes decrypt existing content, update it, and re-encrypt it.
- File reads decrypt and authenticate stored ciphertext.
- File resizing decrypts, resizes and re-encrypts content.
- File metadata reports plaintext size rather than ciphertext size.

The current implementation uses in-memory filesystem storage.

## 9. Security Considerations and Limitations

- Validate input and output buffer sizes.
- Check native function return codes.
- Authenticate ciphertext before returning plaintext.
- Protect cryptographic keys.
- Release native resources properly.
- Handle encryption and decryption failures safely.
- The encryption key is currently held in memory.
- Persistent key storage and recovery across filesystem restarts
  are not implemented.
- The current implementation re-encrypts the complete file
  after each write operation.

## 10. Testing and Verification

The following tests have passed:

Crypto tests:
- Encryption and decryption round trip.
- Empty input encryption.
- Tampered ciphertext rejection.
- Decryption with a different crypto context rejection.

Filesystem tests:
- Virtual file creation.
- Duplicate file creation.
- Encrypted file write and read.
- File listing and lookup.
- File metadata verification.
- Invalid filename rejection.
- Write at offset.
- Ciphertext storage verification.
- Tampered file rejection.
- Encrypted file resizing.

Test command:

    cargo test -- --nocapture

Result:

    14 passed
    0 failed

Compilation verification:

    cargo check

Result: Successful.

## 11. Implementation Status

Completed:
- Rust FFI integration.
- Native C cryptographic implementation.
- OpenSSL AES-256-GCM encryption.
- Rust CryptoProvider implementation.
- Native C library linking.
- Encrypted FUSE file storage.
- Encrypted write and authenticated read operations.
- Encrypted file resizing.
- Error handling for encryption and decryption failures.
- Unit testing and integration-level filesystem state testing.
- Documentation of encryption and decryption flows.

Remaining:
- Live mounted FUSE filesystem verification.
- Persistent secure key storage and recovery.
- Performance optimization for large files.

## 12. Conclusion

QuantumVault-PQC now integrates the Rust FUSE filesystem
with a native C AES-256-GCM cryptographic backend through FFI.

File contents are encrypted before storage and authenticated
decryption is performed when data is read.

All 14 automated tests passed successfully.
