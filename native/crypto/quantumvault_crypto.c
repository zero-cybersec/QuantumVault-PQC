#include "quantumvault_crypto.h"

#include <limits.h>
#include <stdint.h>
#include <stdlib.h>

#include <openssl/crypto.h>
#include <openssl/evp.h>
#include <openssl/rand.h>

#define QV_KEY_SIZE 32

struct QvCryptoContext {
    unsigned char key[QV_KEY_SIZE];
    unsigned int initialized;
};

QvCryptoContext *qv_crypto_init(void) {
    QvCryptoContext *ctx = calloc(1, sizeof(QvCryptoContext));

    if (ctx == NULL) {
        return NULL;
    }

    if (RAND_bytes(ctx->key, QV_KEY_SIZE) != 1) {
        OPENSSL_cleanse(ctx, sizeof(*ctx));
        free(ctx);
        return NULL;
    }

    ctx->initialized = 1;
    return ctx;
}

int qv_crypto_encrypt(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
) {
    if (ctx == NULL || ctx->initialized != 1 || output == NULL) {
        return -1;
    }

    if (input_len > (size_t)(INT_MAX - QV_CRYPTO_OVERHEAD)) {
        return -3;
    }

    if (input_len > 0 && input == NULL) {
        return -1;
    }

    size_t required = input_len + QV_CRYPTO_OVERHEAD;

    if (output_capacity < required) {
        return -2;
    }

    unsigned char *nonce = output;
    unsigned char *tag = output + QV_CRYPTO_NONCE_SIZE;
    unsigned char *ciphertext = output + QV_CRYPTO_OVERHEAD;

    if (RAND_bytes(nonce, QV_CRYPTO_NONCE_SIZE) != 1) {
        return -4;
    }

    EVP_CIPHER_CTX *evp = EVP_CIPHER_CTX_new();

    if (evp == NULL) {
        return -4;
    }

    int len = 0;
    int total = 0;
    int success = 0;

    if (EVP_EncryptInit_ex(evp, EVP_aes_256_gcm(), NULL, NULL, NULL) != 1)
        goto cleanup;

    if (EVP_CIPHER_CTX_ctrl(
            evp, EVP_CTRL_GCM_SET_IVLEN,
            QV_CRYPTO_NONCE_SIZE, NULL) != 1)
        goto cleanup;

    if (EVP_EncryptInit_ex(evp, NULL, NULL, ctx->key, nonce) != 1)
        goto cleanup;

    if (input_len > 0) {
        if (EVP_EncryptUpdate(
                evp, ciphertext, &len, input, (int)input_len) != 1)
            goto cleanup;

        total = len;
    }

    if (EVP_EncryptFinal_ex(evp, ciphertext + total, &len) != 1)
        goto cleanup;

    total += len;

    if (EVP_CIPHER_CTX_ctrl(
            evp, EVP_CTRL_GCM_GET_TAG, QV_CRYPTO_TAG_SIZE, tag) != 1)
        goto cleanup;

    success = 1;

cleanup:
    EVP_CIPHER_CTX_free(evp);

    if (!success) {
        OPENSSL_cleanse(output, required);
        return -4;
    }

    return QV_CRYPTO_OVERHEAD + total;
}

int qv_crypto_decrypt(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
) {
    if (ctx == NULL || ctx->initialized != 1 || input == NULL) {
        return -1;
    }

    if (input_len < QV_CRYPTO_OVERHEAD) {
        return -1;
    }

    size_t ciphertext_len = input_len - QV_CRYPTO_OVERHEAD;

    if (ciphertext_len > INT_MAX) {
        return -3;
    }

    if (output_capacity < ciphertext_len) {
        return -2;
    }

    if (ciphertext_len > 0 && output == NULL) {
        return -1;
    }

    const unsigned char *nonce = input;
    const unsigned char *tag = input + QV_CRYPTO_NONCE_SIZE;
    const unsigned char *ciphertext = input + QV_CRYPTO_OVERHEAD;

    EVP_CIPHER_CTX *evp = EVP_CIPHER_CTX_new();

    if (evp == NULL) {
        return -4;
    }

    unsigned char final_buffer[16];
    unsigned char *final_output =
        ciphertext_len > 0 ? output + ciphertext_len : final_buffer;

    int len = 0;
    int total = 0;
    int success = 0;

    if (EVP_DecryptInit_ex(evp, EVP_aes_256_gcm(), NULL, NULL, NULL) != 1)
        goto cleanup;

    if (EVP_CIPHER_CTX_ctrl(
            evp, EVP_CTRL_GCM_SET_IVLEN,
            QV_CRYPTO_NONCE_SIZE, NULL) != 1)
        goto cleanup;

    if (EVP_DecryptInit_ex(evp, NULL, NULL, ctx->key, nonce) != 1)
        goto cleanup;

    if (ciphertext_len > 0) {
        if (EVP_DecryptUpdate(
                evp, output, &len, ciphertext, (int)ciphertext_len) != 1)
            goto cleanup;

        total = len;
    }

    if (EVP_CIPHER_CTX_ctrl(
            evp, EVP_CTRL_GCM_SET_TAG,
            QV_CRYPTO_TAG_SIZE, (void *)tag) != 1)
        goto cleanup;

    if (EVP_DecryptFinal_ex(evp, final_output, &len) != 1)
        goto cleanup;

    total += len;
    success = 1;

cleanup:
    EVP_CIPHER_CTX_free(evp);

    if (!success) {
        if (output != NULL && ciphertext_len > 0) {
            OPENSSL_cleanse(output, ciphertext_len);
        }

        OPENSSL_cleanse(final_buffer, sizeof(final_buffer));
        return -4;
    }

    OPENSSL_cleanse(final_buffer, sizeof(final_buffer));
    return total;
}

void qv_crypto_free(QvCryptoContext *ctx) {
    if (ctx != NULL) {
        OPENSSL_cleanse(ctx, sizeof(*ctx));
        free(ctx);
    }
}
