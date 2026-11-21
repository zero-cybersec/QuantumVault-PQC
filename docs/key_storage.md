# Key Storage and Security

## Overview
The Key Management module provides metadata storage and retrieval for cryptographic keys.

## Key Types
- PublicKey: Represents public key metadata.
- SecretKey: Represents secret key metadata.
- KeyPair: Represents key-pair metadata.

## Storage Operations
- Store: Saves key metadata in persistent JSON storage.
- Retrieve: Fetches metadata using a key ID.
- Delete: Removes metadata using a key ID.

## Error Handling
- KeyNotFound: Requested key does not exist.
- KeyAlreadyExists: Duplicate key ID.
- StorageError: Storage operation failed.

## Security Requirements
- Never print secret key material in logs.
- Restrict access to secret keys.
- Protect secret key material at rest.
- Securely erase sensitive memory after use.

## Implementation Note
The current JSON storage persists key metadata only. Actual public and secret key bytes require secure storage and integration with the PQC Crypto Engine.
## Key Access Error Handling

- Empty or blank key IDs return an InvalidMetadata error.
- Requests for keys that do not exist return a KeyNotFound error.
- Unavailable crypto engine operations return a CryptoEngineUnavailable error.
- Storage failures return a StorageError.
- Error messages do not expose secret key material.
