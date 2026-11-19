use std::ffi::{c_int, c_uchar, c_void};

/// Opaque context owned by the native crypto library.
#[repr(C)]
pub struct QvCryptoContext {
    _private: [u8; 0],
}
// Native crypto library interface.
unsafe extern "C" {
    pub fn qv_crypto_init() -> *mut QvCryptoContext;

    pub fn qv_crypto_encrypt(
        ctx: *mut QvCryptoContext,
        input: *const c_uchar,
        input_len: usize,
        output: *mut c_uchar,
        output_capacity: usize,
    ) -> c_int;

    pub fn qv_crypto_decrypt(
        ctx: *mut QvCryptoContext,
        input: *const c_uchar,
        input_len: usize,
        output: *mut c_uchar,
        output_capacity: usize,
    ) -> c_int;

    pub fn qv_crypto_free(ctx: *mut QvCryptoContext);
}

pub type CryptoStatus = c_int;
pub type NativeHandle = *mut c_void;
