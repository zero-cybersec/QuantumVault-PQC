# QuantumVault-PQC – Key Management and Security

## 1. Overview
The Key Management module manages cryptographic key metadata and provides interfaces for key generation, storage, retrieval, and deletion.

## 2. Key Metadata Structure
Each key record contains:
- key_id: Unique identifier of the key.
- algorithm: Cryptographic algorithm name.
- key_type: Type of key.
- created_at: Key creation timestamp.
- storage_path: Assigned storage location.
- status: Current key status.

## 3. Key Lifecycle

### Key Generation
- Provides an interface for key generation.
- Current implementation uses a mock implementation.
- Actual ML-KEM key generation will be integrated with the Crypto Engine.

### Key Storage
- Stores key metadata in JSON format.
- Uses a HashMap protected by a Mutex during runtime.
- Metadata is persisted in secure/key_metadata.json.
- Duplicate key IDs are rejected.

### Key Retrieval
- Retrieves metadata using the key ID.
- Returns an error if the key ID does not exist.

### Key Deletion
- Deletes metadata using the key ID.
- Deleted metadata cannot be retrieved.

## 4. Secure Key Handling Requirements
- Private keys must never be stored in plaintext.
- Secret key material must use secure storage.
- Access to private keys must be restricted.
- Sensitive key data must never appear in logs.
- Keys must be securely erased when no longer required.

## 5. Memory Security Requirements
Future implementation must include:
- mlock() to prevent sensitive memory pages from being swapped.
- explicit_bzero() or an equivalent secure zeroization method.
- Avoid unnecessary copies of private key material.
- Release sensitive memory immediately after use.

## 6. Implementation Status
- Key metadata structure: Implemented.
- Key generation interface: Implemented with mock implementation.
- Persistent JSON metadata storage: Implemented.
- Metadata retrieval and deletion: Implemented.
- Unit tests: 9 passed.
- Actual ML-KEM key generation integration: Pending.
- Secure private-key storage: Pending.
- Secure memory wiping: Pending.

## 7. Conclusion
The key management module provides metadata handling, persistent metadata storage, and key lifecycle operations. Integration with the actual PQC Crypto Engine and secure handling of private key material remain future implementation tasks.
