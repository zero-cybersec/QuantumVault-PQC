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

/*
 * Encrypt plaintext using AES-256-GCM.
 *
 * Returns:
 *   0  = success
 *  -1  = invalid arguments
 *  -2  = encryption failure
 */
int qv_aes256gcm_encrypt(
    const uint8_t *key,
    const uint8_t *nonce,
    const uint8_t *plaintext,
    size_t plaintext_len,
    uint8_t *ciphertext,
    uint8_t *tag
);

/*
 * Decrypt and authenticate AES-256-GCM ciphertext.
 *
 * Returns:
 *   0  = success
 *  -1  = invalid arguments
 *  -2  = authentication/decryption failure
 */
int qv_aes256gcm_decrypt(
    const uint8_t *key,
    const uint8_t *nonce,
    const uint8_t *ciphertext,
    size_t ciphertext_len,
    const uint8_t *tag,
    uint8_t *plaintext
);

#ifdef __cplusplus
}
#endif

#endif
