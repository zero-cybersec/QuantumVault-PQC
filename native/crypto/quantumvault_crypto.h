#ifndef QUANTUMVAULT_CRYPTO_H
#define QUANTUMVAULT_CRYPTO_H

#include <stddef.h>

#define QV_CRYPTO_NONCE_SIZE 12
#define QV_CRYPTO_TAG_SIZE 16
#define QV_CRYPTO_OVERHEAD 28

typedef struct QvCryptoContext QvCryptoContext;

QvCryptoContext *qv_crypto_init(void);

int qv_crypto_encrypt(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
);

int qv_crypto_decrypt(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
);

void qv_crypto_free(QvCryptoContext *ctx);

#endif
