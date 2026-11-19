#include "quantumvault_crypto.h"

#include <limits.h>
#include <stdlib.h>
#include <string.h>

struct QvCryptoContext {
    unsigned int initialized;
};

QvCryptoContext *qv_crypto_init(void) {
    QvCryptoContext *ctx = malloc(sizeof(QvCryptoContext));

    if (ctx == NULL) {
        return NULL;
    }

    ctx->initialized = 1;
    return ctx;
}

static int qv_copy(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
) {
    if (ctx == NULL || ctx->initialized != 1) {
        return -1;
    }

    if (input_len > INT_MAX) {
        return -3;
    }

    if (output_capacity < input_len) {
        return -2;
    }

    if (input_len > 0 && (input == NULL || output == NULL)) {
        return -1;
    }

    if (input_len > 0) {
        memcpy(output, input, input_len);
    }

    return (int)input_len;
}

int qv_crypto_encrypt(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
) {
    return qv_copy(ctx, input, input_len, output, output_capacity);
}

int qv_crypto_decrypt(
    QvCryptoContext *ctx,
    const unsigned char *input,
    size_t input_len,
    unsigned char *output,
    size_t output_capacity
) {
    return qv_copy(ctx, input, input_len, output, output_capacity);
}

void qv_crypto_free(QvCryptoContext *ctx) {
    free(ctx);
}
