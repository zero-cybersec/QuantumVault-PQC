cat > docs/ml_dsa_key_management.md <<'EOF'
# ML-DSA-65 Key Management Design

## Overview
The Key Management module will manage public and private keys used for ML-DSA-65 digital signatures.

## Key Handling
- Public keys can be retrieved and shared for signature verification.
- Private keys must be stored securely and accessed only by authorized operations.
- Private key data must never be printed in logs.
- Each key pair will have a unique key ID.
- Key metadata will include the key ID, algorithm, creation time, and key type.

## Key Storage
- Public key metadata will be stored using the existing KeyStorage module.
- Private keys must be protected from unauthorized access.
- Secure persistent storage for private keys will be handled in a future integration.

## Key Management Interface
The module should support:
- Store a generated ML-DSA-65 key pair.
- Retrieve a public key using its key ID.
- Retrieve a private key securely for signing.
- Delete a key pair using its key ID.
- Return an error when a key ID is missing or invalid.

## Key Lifecycle
1. Generate an ML-DSA-65 key pair.
2. Assign a unique key ID.
3. Store the public and private key data securely.
4. Retrieve the private key for signing.
5. Retrieve the public key for verification.
6. Delete the key pair when it is no longer required.

## Error Handling
The module should handle missing keys, duplicate key IDs, invalid key data, and storage errors.

## Conclusion
This design provides the required key management structure for ML-DSA-65 digital signatures in QuantumVault-PQC.


## Secure ML_DSA Secret-Key Storage and Retrieval

### 1. Secure Storage 
- Secret keys must never be stored in plain text.
- Secret keys should be encrypted before being stored permanently.
- Encryption keys must be protected separately from stored secret keys.
- Only authorized components should access secret-key storage.

### 2. Key Retrieval and Access Control
- Only authenticated and authorized users or services can request key access.
- Secret keys should be retrieved only when required for signing.
- Public keys can be accessed separately for signature verificatin.
- Secret keys must never be exposed through logs or error messages.

### 3. Key Lifecycle
1. Generate the ML-DSA-65 key pair.
2. Encrypt and securely store the secret key.
3. Retrieve the secret key onlyfor authorized signing operations.
4. Use the key for signing.
5. Clear sensitive key data from memory after use where possible.
6. Revoke or delete keys when they are no longer required.

### 4. Implementation Status
This section defines the secure storage and access-control design.Encrypted persistent storage and secure memory handling are requirements for future implementation. The current in-memory key store is a basic prototype and does not yet provide these protections.

## Day 14 – Secure Key Handling and Access Restrictions

### Access Restrictions
- Only authorized components can retrieve secret keys.
- Secret keys must be accessed only when required for signing operations.
- Public keys can be shared for signature verification.
- Unauthorized access requests must be rejected.
- Secret keys must never be displayed in logs or error messages.

### Secure Key Handling
- Secret keys must be encrypted before persistent storage.
- XChaCha20-Poly1305 authenticated encryption is used in the secure storage prototype.
- Incorrect encryption keys must result in decryption failure.
- Modified or corrupted ciphertext must be rejected.
- Sensitive key data should be cleared from memory after use.

### Security Testing
- Encryption and decryption test passed.
- Incorrect encryption key test passed.
- Tampered ciphertext test passed.
- Total key-management tests passed: 23.

### Implementation Note
The current secure storage module provides encryption and decryption functionality. Production deployment still requires secure encryption-key management, unique nonce generation, persistent storage integration, and authorization enforcement.
