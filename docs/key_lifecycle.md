# QuantumVault-PQC Key Lifecycle

## 1. Overview

The Key Management module manages the lifecycle of cryptographic key metadata in QuantumVault-PQC.

It provides interfaces for key generation, storage, retrieval, and deletion.

## 2. Key Generation

- The module defines an interface for requesting key generation.
- The cryptographic operation is handled by the PQC Crypto Engine.
- ML-KEM-768 is the intended key encapsulation algorithm.
- Generated key metadata identifies the key and its algorithm.

## 3. Key Storage

- Key metadata is stored using the KeyStorage module.
- Each key is identified by a unique key ID.
- Duplicate key IDs are rejected.
- The current implementation uses in-memory storage.

## 4. Key Retrieval

- Keys can be retrieved using their unique key ID.
- The storage module returns the corresponding metadata.
- Unknown key IDs return a KeyNotFound error.

## 5. Key Deletion

- Keys can be deleted using their unique key ID.
- Successful deletion removes the metadata entry.
- Requests for unknown keys return an error.

## 6. Error Handling

The module supports the following errors:

- KeyNotFound
- KeyAlreadyExists
- InvalidMetadata
- StorageError
- CryptoEngineUnavailable

## 7. Testing

The module includes tests for:

- Key generation interface
- Key retrieval interface
- Key deletion interface
- Unknown key handling
- Metadata storage
- Duplicate key prevention
- Successful retrieval
- Successful deletion

All 8 unit tests passed successfully.

## 8. Current Limitations

- Storage currently uses in-memory metadata.
- Persistent storage is not implemented.
- Actual cryptographic key generation is handled by the separate PQC Crypto Engine.
- Secure encrypted secret-key storage requires future integration.

## 9. Conclusion

The Key Management module provides the foundation for managing cryptographic key metadata and lifecycle operations in QuantumVault-PQC.
