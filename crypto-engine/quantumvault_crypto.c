#include "include/quantumvault_crypto.h"
#include <openssl/evp.h>
#include <openssl/crypto.h>
#include <limits.h>

int qv_aes256gcm_encrypt(
    const uint8_t *key,
    const uint8_t *nonce,
    const uint8_t *plaintext,
    size_t plaintext_len,
    uint8_t *ciphertext,
    uint8_t *tag)
{
    if (!key || !nonce || !plaintext || !ciphertext || !tag ||
        plaintext_len > INT_MAX)
        return -1;

    EVP_CIPHER_CTX *ctx = EVP_CIPHER_CTX_new();
    if (!ctx)
        return -2;

    int len = 0;
    int total_len = 0;
    int result = -2;

    if (EVP_EncryptInit_ex(ctx, EVP_aes_256_gcm(), NULL, NULL, NULL) != 1)
        goto cleanup;

    if (EVP_EncryptInit_ex(ctx, NULL, NULL, key, nonce) != 1)
        goto cleanup;

    if (EVP_EncryptUpdate(ctx, ciphertext, &len,
                         plaintext, (int)plaintext_len) != 1)
        goto cleanup;

    total_len = len;

    if (EVP_EncryptFinal_ex(ctx, ciphertext + total_len, &len) != 1)
        goto cleanup;

    if (EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_GET_TAG,
                            QV_GCM_TAG_SIZE, tag) != 1)
        goto cleanup;

    result = 0;

cleanup:
    EVP_CIPHER_CTX_free(ctx);
    return result;
}

int qv_aes256gcm_decrypt(
    const uint8_t *key,
    const uint8_t *nonce,
    const uint8_t *ciphertext,
    size_t ciphertext_len,
    const uint8_t *tag,
    uint8_t *plaintext)
{
    if (!key || !nonce || !ciphertext || !tag || !plaintext ||
        ciphertext_len > INT_MAX)
        return -1;

    EVP_CIPHER_CTX *ctx = EVP_CIPHER_CTX_new();
    if (!ctx)
        return -2;

    int len = 0;
    int total_len = 0;
    int result = -2;

    if (EVP_DecryptInit_ex(ctx, EVP_aes_256_gcm(), NULL, NULL, NULL) != 1)
        goto cleanup;

    if (EVP_DecryptInit_ex(ctx, NULL, NULL, key, nonce) != 1)
        goto cleanup;

    if (EVP_DecryptUpdate(ctx, plaintext, &len,
                          ciphertext, (int)ciphertext_len) != 1)
        goto cleanup;

    total_len = len;

    if (EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_TAG,
                            QV_GCM_TAG_SIZE, (void *)tag) != 1)
        goto cleanup;

    if (EVP_DecryptFinal_ex(ctx, plaintext + total_len, &len) != 1) {
        OPENSSL_cleanse(plaintext, ciphertext_len);
        goto cleanup;
    }

    result = 0;

cleanup:
    EVP_CIPHER_CTX_free(ctx);
    return result;
}
