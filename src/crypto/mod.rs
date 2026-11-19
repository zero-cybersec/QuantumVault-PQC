#[derive(Debug, PartialEq)]
pub enum CryptoError {
    BackendUnavailable,
    EncryptionFailed,
    DecryptionFailed,
}

pub trait CryptoProvider {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError>;

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError>;
}

/// Placeholder for future native FFI integration.
pub struct FfiCryptoProvider;

impl CryptoProvider for FfiCryptoProvider {
    fn encrypt(&self, _plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Err(CryptoError::BackendUnavailable)
    }

    fn decrypt(&self, _ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        Err(CryptoError::BackendUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crypto_backend_unavailable() {
        let provider = FfiCryptoProvider;

        assert_eq!(
            provider.encrypt(b"Hello"),
            Err(CryptoError::BackendUnavailable)
        );

        assert_eq!(
            provider.decrypt(b"Hello"),
            Err(CryptoError::BackendUnavailable)
        );
    }
}
