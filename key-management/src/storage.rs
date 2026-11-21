use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::{KeyManagementError, KeyMetadata};

pub struct KeyStorage {
    metadata: Mutex<HashMap<String, KeyMetadata>>,
    file_path: PathBuf,
}

impl KeyStorage {
    // Open persistent storage
    pub fn new() -> Self {
        Self::open("secure/key_metadata.json").unwrap_or_else(|_| Self {
            metadata: Mutex::new(HashMap::new()),
            file_path: PathBuf::from("secure/key_metadata.json"),
        })
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, KeyManagementError> {
        let file_path = path.as_ref().to_path_buf();

        let metadata = if file_path.exists() {
            let content =
                fs::read_to_string(&file_path).map_err(|_| KeyManagementError::StorageError)?;

            serde_json::from_str(&content).map_err(|_| KeyManagementError::StorageError)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            metadata: Mutex::new(metadata),
            file_path,
        })
    }

    // Save metadata permanently to JSON
    fn save(&self, data: &HashMap<String, KeyMetadata>) -> Result<(), KeyManagementError> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(|_| KeyManagementError::StorageError)?;
        }

        let json =
            serde_json::to_string_pretty(data).map_err(|_| KeyManagementError::StorageError)?;

        let temp_path = self.file_path.with_extension("tmp");

        fs::write(&temp_path, json).map_err(|_| KeyManagementError::StorageError)?;

        fs::rename(&temp_path, &self.file_path).map_err(|_| KeyManagementError::StorageError)?;

        Ok(())
    }

    pub fn store_key(&self, metadata: KeyMetadata) -> Result<(), KeyManagementError> {
        let mut keys = self
            .metadata
            .lock()
            .map_err(|_| KeyManagementError::StorageError)?;

        if keys.contains_key(&metadata.key_id) {
            return Err(KeyManagementError::KeyAlreadyExists);
        }

        let id = metadata.key_id.clone();
        keys.insert(id.clone(), metadata);

        if let Err(error) = self.save(&keys) {
            keys.remove(&id);
            return Err(error);
        }

        Ok(())
    }

    pub fn retrieve_key(&self, key_id: &str) -> Result<KeyMetadata, KeyManagementError> {
        if key_id.trim().is_empty() {
            return Err(KeyManagementError::InvalidMetadata);
        }
        let keys = self
            .metadata
            .lock()
            .map_err(|_| KeyManagementError::StorageError)?;

        keys.get(key_id)
            .cloned()
            .ok_or(KeyManagementError::KeyNotFound)
    }

    pub fn delete_key(&self, key_id: &str) -> Result<(), KeyManagementError> {
        let mut keys = self
            .metadata
            .lock()
            .map_err(|_| KeyManagementError::StorageError)?;

        let removed = keys.remove(key_id).ok_or(KeyManagementError::KeyNotFound)?;

        if let Err(error) = self.save(&keys) {
            keys.insert(key_id.to_string(), removed);
            return Err(error);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn test_storage() -> (KeyStorage, PathBuf) {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "quantumvault_test_{}_{}.json",
            std::process::id(),
            id
        ));

        let storage = KeyStorage::open(&path).unwrap();
        (storage, path)
    }

    fn sample_metadata() -> KeyMetadata {
        KeyMetadata {
            key_id: "test-key-001".to_string(),
            algorithm: "ML-KEM-768".to_string(),
            key_type: crate::KeyType::KeyPair,
            created_at: "2026-11-15".to_string(),
            storage_path: "secure/keys/".to_string(),
            status: "active".to_string(),
        }
    }

    #[test]
    fn test_store_key() {
        let (storage, path) = test_storage();
        assert!(storage.store_key(sample_metadata()).is_ok());
        assert!(path.exists());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_retrieve_key() {
        let (storage, path) = test_storage();
        storage.store_key(sample_metadata()).unwrap();
        assert!(storage.retrieve_key("test-key-001").is_ok());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_delete_key() {
        let (storage, path) = test_storage();
        storage.store_key(sample_metadata()).unwrap();
        assert!(storage.delete_key("test-key-001").is_ok());
        assert!(storage.retrieve_key("test-key-001").is_err());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_duplicate_key() {
        let (storage, path) = test_storage();
        storage.store_key(sample_metadata()).unwrap();

        assert!(matches!(
            storage.store_key(sample_metadata()),
            Err(KeyManagementError::KeyAlreadyExists)
        ));

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_persistent_storage() {
        let (storage, path) = test_storage();
        storage.store_key(sample_metadata()).unwrap();

        let reopened = KeyStorage::open(&path).unwrap();
        assert!(reopened.retrieve_key("test-key-001").is_ok());

        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_public_key_storage() {
        let (storage, path) = test_storage();
        let mut metadata = sample_metadata();
        metadata.key_id = "public-key-001".to_string();
        metadata.key_type = crate::KeyType::PublicKey;

        storage.store_key(metadata).unwrap();
        assert_eq!(
            storage.retrieve_key("public-key-001").unwrap().key_type,
            crate::KeyType::PublicKey
        );
        let _ = fs::remove_file(path);
    }
    #[test]
    fn test_empty_key_id_returns_error() {
        let storage = KeyStorage::open("test_empty_key_id.json").unwrap();
        let result = storage.retrieve_key("");
        assert!(matches!(result, Err(KeyManagementError::InvalidMetadata)));
    }

    #[test]
    fn test_secret_key_storage() {
        let (storage, path) = test_storage();
        let mut metadata = sample_metadata();
        metadata.key_id = "secret-key-001".to_string();
        metadata.key_type = crate::KeyType::SecretKey;

        storage.store_key(metadata).unwrap();
        assert_eq!(
            storage.retrieve_key("secret-key-001").unwrap().key_type,
            crate::KeyType::SecretKey
        );
        let _ = fs::remove_file(path);
    }
}
