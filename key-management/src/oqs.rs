use zeroize::Zeroizing;

unsafe extern "C" {
    fn qv_mlkem768_keypair(public_key: *mut u8, secret_key: *mut u8) -> i32;

    fn qv_mlkem768_public_key_length() -> usize;
    fn qv_mlkem768_secret_key_length() -> usize;
}

pub struct MlKem768KeyPair {
    pub public_key: Vec<u8>,
    pub secret_key: Zeroizing<Vec<u8>>,
}

pub fn generate_mlkem768_keypair() -> Result<MlKem768KeyPair, String> {
    let public_len = unsafe { qv_mlkem768_public_key_length() };
    let secret_len = unsafe { qv_mlkem768_secret_key_length() };

    if public_len == 0 || secret_len == 0 {
        return Err("ML-KEM-768 initialization failed".into());
    }

    let mut public_key = vec![0u8; public_len];
    let mut secret_key = Zeroizing::new(vec![0u8; secret_len]);

    let result = unsafe { qv_mlkem768_keypair(public_key.as_mut_ptr(), secret_key.as_mut_ptr()) };

    if result != 0 {
        return Err("ML-KEM-768 key generation failed".into());
    }

    Ok(MlKem768KeyPair {
        public_key,
        secret_key,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_mlkem768_key_generation() {
        let pair = generate_mlkem768_keypair().unwrap();

        assert_eq!(pair.public_key.len(), 1184);
        assert_eq!(pair.secret_key.len(), 2400);
    }
}
