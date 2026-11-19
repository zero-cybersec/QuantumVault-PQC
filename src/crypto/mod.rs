use crate::ffi::QvCryptoContext;

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

struct CryptoContext(*mut QvCryptoContext);

impl Drop for CryptoContext {
    fn drop(&mut self) {
        unsafe {
            crate::ffi::qv_crypto_free(self.0);
        }
    }
}

type CryptoOperation =
    unsafe extern "C" fn(*mut QvCryptoContext, *const u8, usize, *mut u8, usize) -> i32;

pub struct FfiCryptoProvider;

impl FfiCryptoProvider {
    fn process(
        &self,
        input: &[u8],
        operation: CryptoOperation,
        error: CryptoError,
    ) -> Result<Vec<u8>, CryptoError> {
        let context = unsafe { crate::ffi::qv_crypto_init() };

        if context.is_null() {
            return Err(CryptoError::BackendUnavailable);
        }

        let context = CryptoContext(context);
        let mut output = vec![0u8; input.len()];

        let status = unsafe {
            operation(
                context.0,
                input.as_ptr(),
                input.len(),
                output.as_mut_ptr(),
                output.len(),
            )
        };

        if status < 0 || status as usize != input.len() {
            return Err(error);
        }

        Ok(output)
    }
}

impl CryptoProvider for FfiCryptoProvider {
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.process(
            plaintext,
            crate::ffi::qv_crypto_encrypt,
            CryptoError::EncryptionFailed,
        )
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, CryptoError> {
        self.process(
            ciphertext,
            crate::ffi::qv_crypto_decrypt,
            CryptoError::DecryptionFailed,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_api_round_trip_smoke_test() {
        let provider = FfiCryptoProvider;
        let input = b"QuantumVault Rust to C FFI";

        let encrypted = provider.encrypt(input).unwrap();
        let decrypted = provider.decrypt(&encrypted).unwrap();

        assert_eq!(encrypted, input);
        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_ffi_api_empty_input() {
        let provider = FfiCryptoProvider;

        assert_eq!(provider.encrypt(b""), Ok(vec![]));
        assert_eq!(provider.decrypt(b""), Ok(vec![]));
    }
}
