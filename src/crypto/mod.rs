use crate::ffi::QvCryptoContext;
use std::ptr::NonNull;

const CRYPTO_OVERHEAD: usize = 28;
const FINAL_BUFFER_EXTRA: usize = 16;

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

pub struct FfiCryptoProvider {
    context: NonNull<QvCryptoContext>,
}

// Access is serialized by the filesystem state mutex.
unsafe impl Send for FfiCryptoProvider {}

impl FfiCryptoProvider {
    pub fn new() -> Result<Self, CryptoError> {
        let context = unsafe { crate::ffi::qv_crypto_init() };

        let context = NonNull::new(context).ok_or(CryptoError::BackendUnavailable)?;

        Ok(Self { context })
    }
}

impl Drop for FfiCryptoProvider {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::qv_crypto_free(self.context.as_ptr());
        }
    }
}

impl CryptoProvider for FfiCryptoProvider {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        let capacity = plaintext
            .len()
            .checked_add(CRYPTO_OVERHEAD + FINAL_BUFFER_EXTRA)
            .ok_or(CryptoError::EncryptionFailed)?;

        let mut output = vec![0u8; capacity];

        let status = unsafe {
            crate::ffi::qv_crypto_encrypt(
                self.context.as_ptr(),
                plaintext.as_ptr(),
                plaintext.len(),
                output.as_mut_ptr(),
                output.len(),
            )
        };

        if status < CRYPTO_OVERHEAD as i32 {
            return Err(CryptoError::EncryptionFailed);
        }

        let written = status as usize;

        if written > output.len() {
            return Err(CryptoError::EncryptionFailed);
        }

        output.truncate(written);
        Ok(output)
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        if ciphertext.len() < CRYPTO_OVERHEAD {
            return Err(CryptoError::DecryptionFailed);
        }

        let plaintext_len = ciphertext.len() - CRYPTO_OVERHEAD;

        let capacity = plaintext_len
            .checked_add(FINAL_BUFFER_EXTRA)
            .ok_or(CryptoError::DecryptionFailed)?;

        let mut output = vec![0u8; capacity];

        let status = unsafe {
            crate::ffi::qv_crypto_decrypt(
                self.context.as_ptr(),
                ciphertext.as_ptr(),
                ciphertext.len(),
                output.as_mut_ptr(),
                output.len(),
            )
        };

        if status < 0 || status as usize != plaintext_len {
            return Err(CryptoError::DecryptionFailed);
        }

        output.truncate(status as usize);
        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_round_trip() {
        let provider = FfiCryptoProvider::new().unwrap();
        let plaintext = b"QuantumVault AES-256-GCM";

        let encrypted = provider.encrypt(plaintext).unwrap();

        assert_ne!(encrypted, plaintext);
        assert_eq!(encrypted.len(), plaintext.len() + 28);

        let decrypted = provider.decrypt(&encrypted).unwrap();

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_empty_input() {
        let provider = FfiCryptoProvider::new().unwrap();

        let encrypted = provider.encrypt(b"").unwrap();

        assert_eq!(encrypted.len(), 28);
        assert_eq!(provider.decrypt(&encrypted).unwrap(), b"");
    }

    #[test]
    fn test_tampered_ciphertext_rejected() {
        let provider = FfiCryptoProvider::new().unwrap();

        let mut encrypted = provider.encrypt(b"Authentication test").unwrap();

        encrypted[12] ^= 0x01;

        assert_eq!(
            provider.decrypt(&encrypted),
            Err(CryptoError::DecryptionFailed)
        );
    }

    #[test]
    fn test_different_context_cannot_decrypt() {
        let first = FfiCryptoProvider::new().unwrap();
        let second = FfiCryptoProvider::new().unwrap();

        let encrypted = first.encrypt(b"Secret data").unwrap();

        assert_eq!(
            second.decrypt(&encrypted),
            Err(CryptoError::DecryptionFailed)
        );
    }
}
