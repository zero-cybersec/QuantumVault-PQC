
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct SignatureKeyPair {
    pub key_id: String,
    pub public_key: Vec<u8>,
    private_key: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum KeyError {
    EmptyKeyId,
    DuplicateKey,
    KeyNotFound,
    InvalidKey,
}

#[derive(Default)]
pub struct SignatureKeyStore {
    keys: HashMap<String, SignatureKeyPair>,
}

impl SignatureKeyStore {
    pub fn new() -> Self {
        Self {
            keys: HashMap::new(),
        }
    }

    pub fn store_keypair(
        &mut self,
        key_id: String,
        public_key: Vec<u8>,
        private_key: Vec<u8>,
    ) -> Result<(), KeyError> {
        if key_id.trim().is_empty() {
            return Err(KeyError::EmptyKeyId);
        }

        if public_key.is_empty() || private_key.is_empty() {
            return Err(KeyError::InvalidKey);
        }

        if self.keys.contains_key(&key_id) {
            return Err(KeyError::DuplicateKey);
        }

        let pair = SignatureKeyPair {
            key_id: key_id.clone(),
            public_key,
            private_key,
        };

        self.keys.insert(key_id, pair);
        Ok(())
    }

    pub fn get_public_key(
        &self,
        key_id: &str,
    ) -> Result<&[u8], KeyError> {
        self.keys
            .get(key_id)
            .map(|pair| pair.public_key.as_slice())
            .ok_or(KeyError::KeyNotFound)
    }

    pub fn get_private_key(
        &self,
        key_id: &str,
    ) -> Result<&[u8], KeyError> {
        self.keys
            .get(key_id)
            .map(|pair| pair.private_key.as_slice())
            .ok_or(KeyError::KeyNotFound)
    }

    pub fn delete_keypair(
        &mut self,
        key_id: &str,
    ) -> Result<(), KeyError> {
        self.keys
            .remove(key_id)
            .map(|_| ())
            .ok_or(KeyError::KeyNotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve_keys() {
        let mut store = SignatureKeyStore::new();

        assert_eq!(
            store.store_keypair(
                "key1".into(),
                vec![1, 2, 3],
                vec![4, 5, 6]
            ),
            Ok(())
        );

        assert_eq!(store.get_public_key("key1"), Ok(&[1, 2, 3][..]));
        assert_eq!(store.get_private_key("key1"), Ok(&[4, 5, 6][..]));
    }

    #[test]
    fn test_missing_key() {
        let store = SignatureKeyStore::new();
        assert_eq!(store.get_public_key("unknown"), Err(KeyError::KeyNotFound));
    }

    #[test]
    fn test_duplicate_key() {
        let mut store = SignatureKeyStore::new();

        store.store_keypair("key1".into(), vec![1], vec![2]).unwrap();

        assert_eq!(
            store.store_keypair("key1".into(), vec![3], vec![4]),
            Err(KeyError::DuplicateKey)
        );
    }

    #[test]
    fn test_empty_key_id() {
        let mut store = SignatureKeyStore::new();

        assert_eq!(
            store.store_keypair("".into(), vec![1], vec![2]),
            Err(KeyError::EmptyKeyId)
        );
    }

    #[test]
    fn test_delete_key() {
        let mut store = SignatureKeyStore::new();

        store.store_keypair("key1".into(), vec![1], vec![2]).unwrap();
        assert_eq!(store.delete_keypair("key1"), Ok(()));
        assert_eq!(store.get_public_key("key1"), Err(KeyError::KeyNotFound));
    }
}

