#ifndef QUANTUMVAULT_CRYPTO_H
#define QUANTUMVAULT_CRYPTO_H

#include <stddef.h>

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
