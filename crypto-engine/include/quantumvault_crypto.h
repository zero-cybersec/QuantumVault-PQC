#ifndef QUANTUMVAULT_CRYPTO_H
#define QUANTUMVAULT_CRYPTO_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define QV_AES256_KEY_SIZE 32
#define QV_GCM_NONCE_SIZE 12
#define QV_GCM_TAG_SIZE 16

#define QV_MLKEM768_PUBLIC_KEY_SIZE 1184
#define QV_MLKEM768_SECRET_KEY_SIZE 2400
#define QV_MLKEM768_CIPHERTEXT_SIZE 1088
#define QV_MLKEM768_SHARED_SECRET_SIZE 32

/* AES-256-GCM encryption */
int qv_aes256gcm_encrypt(
    const uint8_t *key,
    const uint8_t *nonce,
    const uint8_t *plaintext,
    size_t plaintext_len,
    uint8_t *ciphertext,
    uint8_t *tag
);

/* AES-256-GCM decryption */
int qv_aes256gcm_decrypt(
    const uint8_t *key,
    const uint8_t *nonce,
    const uint8_t *ciphertext,
    size_t ciphertext_len,
    const uint8_t *tag,
    uint8_t *plaintext
);

/* ML-KEM-768 keypair generation */
int qv_mlkem768_keypair(
    uint8_t *public_key,
    size_t public_key_capacity,
    uint8_t *secret_key,
    size_t secret_key_capacity
);

/* ML-KEM-768 encapsulation */
int qv_mlkem768_encaps(
    const uint8_t *public_key,
    size_t public_key_len,
    uint8_t *ciphertext,
    size_t ciphertext_capacity,
    uint8_t *shared_secret,
    size_t shared_secret_capacity
);

/* ML-KEM-768 decapsulation */
int qv_mlkem768_decaps(
    const uint8_t *ciphertext,
    size_t ciphertext_len,
    const uint8_t *secret_key,
    size_t secret_key_len,
    uint8_t *shared_secret,
    size_t shared_secret_capacity
);

#ifdef __cplusplus
}
#endif

#endif
