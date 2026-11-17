# Key Generation Service

## Overview
The Key Generation Service manages cryptographic key generation requests and responses for QuantumVault-PQC.

## Supported Algorithm
- ML-KEM-768

## Key Generation Request
The request contains the algorithm name required for key generation.

## Key Generation Response
The response contains:
- Unique key ID
- Algorithm name
- Public key
- Secret key held in zeroizing memory

## Key Lifecycle
- Generate: Creates an ML-KEM-768 keypair.
- Retrieve: Retrieves key metadata using the key ID.
- Delete: Deletes key metadata using the key ID.

## Security
- Secret key material is not stored in the JSON metadata file.
- Secret key memory uses the Rust Zeroizing wrapper.
- Private key persistence requires secure encrypted storage.
- Sensitive key material must not be exposed in logs.

## Testing
- ML-KEM-768 key generation test.
- Unsupported algorithm validation test.
- Key metadata retrieval test.
- Existing key management and storage tests.

## Result
The key generation service interface is implemented and tested as part of the Rust Key Management module.
