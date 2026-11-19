use std::time::{SystemTime, UNIX_EPOCH};
use zeroize::Zeroizing;

use crate::oqs::generate_mlkem768_keypair;
use crate::storage::KeyStorage;
use crate::{KeyManagementError, KeyMetadata};

pub struct KeyGenerationRequest {
    pub algorithm: String,
}

pub struct KeyGenerationResponse {
    pub key_id: String,
    pub algorithm: String,
    pub public_key: Vec<u8>,
    pub secret_key: Zeroizing<Vec<u8>>,
}

pub struct KeyGenerationService {
    storage: KeyStorage,
}

impl KeyGenerationService {
    pub fn new(storage: KeyStorage) -> Self {
        Self { storage }
    }

    pub fn generate(
        &self,
        request: KeyGenerationRequest,
    ) -> Result<KeyGenerationResponse, KeyManagementError> {
        if request.algorithm != "ML-KEM-768" {
            return Err(KeyManagementError::InvalidMetadata);
        }

        let keypair =
            generate_mlkem768_keypair().map_err(|_| KeyManagementError::CryptoEngineUnavailable)?;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| KeyManagementError::StorageError)?
            .as_nanos();

        let key_id = format!("mlkem768-{timestamp}");

        let metadata = KeyMetadata {
            key_id: key_id.clone(),
            algorithm: "ML-KEM-768".to_string(),
            key_type: crate::KeyType::KeyPair,
            created_at: timestamp.to_string(),
            storage_path: "secure/keys/".to_string(),
            status: "active".to_string(),
        };

        // Store metadata only, never secret key bytes.
        self.storage.store_key(metadata)?;

        Ok(KeyGenerationResponse {
            key_id,
            algorithm: "ML-KEM-768".to_string(),
            public_key: keypair.public_key,
            secret_key: keypair.secret_key,
        })
    }

    pub fn retrieve_metadata(&self, key_id: &str) -> Result<KeyMetadata, KeyManagementError> {
        self.storage.retrieve_key(key_id)
    }

    pub fn delete_metadata(&self, key_id: &str) -> Result<(), KeyManagementError> {
        self.storage.delete_key(key_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn test_storage() -> KeyStorage {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let path = std::env::temp_dir().join(format!("qv-service-{id}.json"));
        KeyStorage::open(path).unwrap()
    }

    #[test]
    fn test_service_generates_mlkem768_keypair() {
        let service = KeyGenerationService::new(test_storage());

        let response = service
            .generate(KeyGenerationRequest {
                algorithm: "ML-KEM-768".to_string(),
            })
            .unwrap();

        assert_eq!(response.algorithm, "ML-KEM-768");
        assert_eq!(response.public_key.len(), 1184);
        assert_eq!(response.secret_key.len(), 2400);
        assert!(service.retrieve_metadata(&response.key_id).is_ok());
    }

    #[test]
    fn test_service_rejects_unsupported_algorithm() {
        let service = KeyGenerationService::new(test_storage());

        let result = service.generate(KeyGenerationRequest {
            algorithm: "UNKNOWN".to_string(),
        });

        assert!(matches!(result, Err(KeyManagementError::InvalidMetadata)));
    }
}
