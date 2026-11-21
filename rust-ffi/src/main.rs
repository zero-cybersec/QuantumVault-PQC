use std::ffi::c_int;

const KEY_SIZE: usize = 32;
const NONCE_SIZE: usize = 12;
const TAG_SIZE: usize = 16;

unsafe extern "C" {
    fn qv_aes256gcm_encrypt(
        key: *const u8,
        nonce: *const u8,
        plaintext: *const u8,
        plaintext_len: usize,
        ciphertext: *mut u8,
        tag: *mut u8,
    ) -> c_int;

    fn qv_aes256gcm_decrypt(
        key: *const u8,
        nonce: *const u8,
        ciphertext: *const u8,
        ciphertext_len: usize,
        tag: *const u8,
        plaintext: *mut u8,
    ) -> c_int;
}

fn main() {
    // Fixed values are only for this FFI test.
    let key = [0x11u8; KEY_SIZE];
    let nonce = [0x22u8; NONCE_SIZE];

    let plaintext = b"QuantumVault Rust FFI Day 9";
    let mut ciphertext = vec![0u8; plaintext.len()];
    let mut tag = [0u8; TAG_SIZE];
    let mut decrypted = vec![0u8; plaintext.len()];

    let encrypt_result = unsafe {
        qv_aes256gcm_encrypt(
            key.as_ptr(),
            nonce.as_ptr(),
            plaintext.as_ptr(),
            plaintext.len(),
            ciphertext.as_mut_ptr(),
            tag.as_mut_ptr(),
        )
    };

    assert_eq!(encrypt_result, 0, "Encryption failed");
    println!("PASS: Rust called C AES-256-GCM encryption");

    let decrypt_result = unsafe {
        qv_aes256gcm_decrypt(
            key.as_ptr(),
            nonce.as_ptr(),
            ciphertext.as_ptr(),
            ciphertext.len(),
            tag.as_ptr(),
            decrypted.as_mut_ptr(),
        )
    };

    assert_eq!(decrypt_result, 0, "Decryption failed");
    assert_eq!(decrypted, plaintext);
    println!("PASS: Rust called C AES-256-GCM decryption");
    println!("PASS: Decrypted plaintext matches original");

    // Modify authentication tag and verify rejection.
    let mut bad_tag = tag;
    bad_tag[0] ^= 0x01;

    let tamper_result = unsafe {
        qv_aes256gcm_decrypt(
            key.as_ptr(),
            nonce.as_ptr(),
            ciphertext.as_ptr(),
            ciphertext.len(),
            bad_tag.as_ptr(),
            decrypted.as_mut_ptr(),
        )
    };

    assert_ne!(tamper_result, 0, "Modified tag was accepted");
    println!("PASS: Modified authentication tag rejected");

    println!("\nALL RUST FFI TESTS PASSED");
}
