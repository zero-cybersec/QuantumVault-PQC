use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use zeroize::Zeroize;

#[derive(Debug, PartialEq)]
pub enum SecureStorageError {
    InvalidKey,
    EncryptionFailed,
    DecryptionFailed,
}

pub fn encrypt_secret_key(
    secret: &[u8],
    encryption_key: &[u8],
    nonce: &[u8; 24],
) -> Result<Vec<u8>, SecureStorageError> {
    let cipher = XChaCha20Poly1305::new_from_slice(encryption_key)
        .map_err(|_| SecureStorageError::InvalidKey)?;

    cipher.encrypt(XNonce::from_slice(nonce), secret)
        .map_err(|_| SecureStorageError::EncryptionFailed)
}

pub fn decrypt_secret_key(
    encrypted: &[u8],
    encryption_key: &[u8],
    nonce: &[u8; 24],
) -> Result<Vec<u8>, SecureStorageError> {
    let cipher = XChaCha20Poly1305::new_from_slice(encryption_key)
        .map_err(|_| SecureStorageError::InvalidKey)?;

    cipher.decrypt(XNonce::from_slice(nonce), encrypted)
        .map_err(|_| SecureStorageError::DecryptionFailed)
}

pub fn clear_secret_key(secret: &mut Vec<u8>) {
    secret.zeroize();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_and_decryption() {
        let key = [7u8; 32];
        let nonce = [1u8; 24];
        let secret = b"sample-secret-key";

        let encrypted = encrypt_secret_key(secret, &key, &nonce).unwrap();
        assert_ne!(encrypted, secret);

        let decrypted = decrypt_secret_key(&encrypted, &key, &nonce).unwrap();
        assert_eq!(decrypted, secret);
    }

    #[test]
    fn test_wrong_key_rejected() {
        let key = [7u8; 32];
        let wrong_key = [8u8; 32];
        let nonce = [1u8; 24];

        let encrypted = encrypt_secret_key(b"secret", &key, &nonce).unwrap();

        assert_eq!(
            decrypt_secret_key(&encrypted, &wrong_key, &nonce),
            Err(SecureStorageError::DecryptionFailed)
        );
    }

    #[test]
    fn test_tampered_data_rejected() {
        let key = [7u8; 32];
        let nonce = [1u8; 24];
        let mut encrypted = encrypt_secret_key(b"secret", &key, &nonce).unwrap();

        encrypted[0] ^= 1;

        assert_eq!(
            decrypt_secret_key(&encrypted, &key, &nonce),
            Err(SecureStorageError::DecryptionFailed)
        );
    }
}
