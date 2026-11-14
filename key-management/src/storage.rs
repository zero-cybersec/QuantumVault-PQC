use std::collections::HashMap;
use std::sync::Mutex;

use crate::{KeyManagementError, KeyMetadata};

pub struct KeyStorage {
    metadata: Mutex<HashMap<String, KeyMetadata>>,
}

impl KeyStorage {
    pub fn new() -> Self {
        Self {
            metadata: Mutex::new(HashMap::new()),
        }
    }

    pub fn store_key(
        &self,
        metadata: KeyMetadata,
    ) -> Result<(), KeyManagementError> {
        let mut keys = self.metadata
            .lock()
            .map_err(|_| KeyManagementError::StorageError)?;

        if keys.contains_key(&metadata.key_id) {
            return Err(KeyManagementError::KeyAlreadyExists);
        }

        keys.insert(metadata.key_id.clone(), metadata);
        Ok(())
    }

    pub fn retrieve_key(
        &self,
        key_id: &str,
    ) -> Result<KeyMetadata, KeyManagementError> {
        let keys = self.metadata
            .lock()
            .map_err(|_| KeyManagementError::StorageError)?;

        keys.get(key_id)
            .cloned()
            .ok_or(KeyManagementError::KeyNotFound)
    }

    pub fn delete_key(
        &self,
        key_id: &str,
    ) -> Result<(), KeyManagementError> {
        let mut keys = self.metadata
            .lock()
            .map_err(|_| KeyManagementError::StorageError)?;

        keys.remove(key_id)
            .map(|_| ())
            .ok_or(KeyManagementError::KeyNotFound)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_metadata() -> KeyMetadata {
        KeyMetadata {
            key_id: "test-key-001".to_string(),
            algorithm: "ML-KEM-768".to_string(),
            key_type: "keypair".to_string(),
            created_at: "2026-11-14".to_string(),
            storage_path: "secure/keys/".to_string(),
            status: "active".to_string(),
        }
    }

    #[test]
    fn test_store_key() {
        let storage = KeyStorage::new();
        assert!(storage.store_key(sample_metadata()).is_ok());
    }

    #[test]
    fn test_retrieve_key() {
        let storage = KeyStorage::new();
        storage.store_key(sample_metadata()).unwrap();

        assert!(storage.retrieve_key("test-key-001").is_ok());
    }

    #[test]
    fn test_delete_key() {
        let storage = KeyStorage::new();
        storage.store_key(sample_metadata()).unwrap();

        assert!(storage.delete_key("test-key-001").is_ok());
        assert!(storage.retrieve_key("test-key-001").is_err());
    }

    #[test]
    fn test_duplicate_key() {
        let storage = KeyStorage::new();
        storage.store_key(sample_metadata()).unwrap();

        assert!(matches!(
            storage.store_key(sample_metadata()),
            Err(KeyManagementError::KeyAlreadyExists)
        ));
    }
}
