// QuantumVault-PQC Key Management Module

#[derive(Debug, Clone, PartialEq)]
pub struct KeyMetadata {
    pub key_id: String,
    pub algorithm: String,
    pub key_type: String,
    pub created_at: String,
    pub storage_path: String,
    pub status: String,
}

// Errors returned by key management operations
#[derive(Debug)]
pub enum KeyManagementError {
    KeyNotFound,
    KeyAlreadyExists,
    InvalidMetadata,
    StorageError,
    CryptoEngineUnavailable,
}

// Interfaces for key management operations
pub trait KeyManagement {

    // Interface for key generation
    fn generate_key(
        &self,
        algorithm: &str,
    ) -> Result<KeyMetadata, KeyManagementError>;

    // Interface for retrieving key metadata
    fn retrieve_key(
        &self,
        key_id: &str,
    ) -> Result<KeyMetadata, KeyManagementError>;

    // Interface for deleting a key
    fn delete_key(
        &self,
        key_id: &str,
    ) -> Result<(), KeyManagementError>;
}
#[cfg(test)]
mod tests {
    use super::*;

    struct MockKeyManager;

    impl KeyManagement for MockKeyManager {
        fn generate_key(
            &self,
            algorithm: &str,
        ) -> Result<KeyMetadata, KeyManagementError> {
            Ok(KeyMetadata {
                key_id: "test-key-001".to_string(),
                algorithm: algorithm.to_string(),
                key_type: "keypair".to_string(),
                created_at: "2026-11-13".to_string(),
                storage_path: "secure/keys/".to_string(),
                status: "active".to_string(),
            })
        }

        fn retrieve_key(
            &self,
            key_id: &str,
        ) -> Result<KeyMetadata, KeyManagementError> {
            if key_id == "test-key-001" {
                self.generate_key("ML-KEM-768")
            } else {
                Err(KeyManagementError::KeyNotFound)
            }
        }

        fn delete_key(
            &self,
            key_id: &str,
        ) -> Result<(), KeyManagementError> {
            if key_id == "test-key-001" {
                Ok(())
            } else {
                Err(KeyManagementError::KeyNotFound)
            }
        }
    }

    #[test]
    fn test_key_generation_interface() {
        let manager = MockKeyManager;
        let result = manager.generate_key("ML-KEM-768");
        assert!(result.is_ok());
    }

    #[test]
    fn test_key_retrieval_interface() {
        let manager = MockKeyManager;
        assert!(manager.retrieve_key("test-key-001").is_ok());
    }

    #[test]
    fn test_key_deletion_interface() {
        let manager = MockKeyManager;
        assert!(manager.delete_key("test-key-001").is_ok());
    }

    #[test]
    fn test_unknown_key_returns_error() {
        let manager = MockKeyManager;
        assert!(manager.retrieve_key("unknown").is_err());
    }
}
