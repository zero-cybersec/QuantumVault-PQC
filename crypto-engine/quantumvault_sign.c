#include "include/quantumvault_crypto.h"
#include <oqs/oqs.h>
#include <string.h>

int qv_mldsa65_keypair(
    uint8_t *public_key,
    size_t public_key_capacity,
    uint8_t *secret_key,
    size_t secret_key_capacity)
{
    if (!public_key || !secret_key ||
        public_key_capacity < QV_MLDSA65_PUBLIC_KEY_SIZE ||
        secret_key_capacity < QV_MLDSA65_SECRET_KEY_SIZE)
        return -1;

    OQS_SIG *sig = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    if (!sig)
        return -2;

    int result = -1;

    if (sig->length_public_key != QV_MLDSA65_PUBLIC_KEY_SIZE ||
        sig->length_secret_key != QV_MLDSA65_SECRET_KEY_SIZE)
        goto cleanup;

    if (OQS_SIG_keypair(sig, public_key, secret_key) != OQS_SUCCESS) {
        OQS_MEM_cleanse(secret_key, QV_MLDSA65_SECRET_KEY_SIZE);
        goto cleanup;
    }

    result = 0;

cleanup:
    OQS_SIG_free(sig);
    return result;
}

int qv_mldsa65_sign(
    const uint8_t *message,
    size_t message_len,
    const uint8_t *secret_key,
    size_t secret_key_len,
    uint8_t *signature,
    size_t signature_capacity)
{
    if (!message || !secret_key || !signature ||
        secret_key_len != QV_MLDSA65_SECRET_KEY_SIZE ||
        signature_capacity < QV_MLDSA65_SIGNATURE_SIZE)
        return -1;

    OQS_SIG *sig = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    if (!sig)
        return -2;

    size_t signature_len = 0;
    int result = -3;

    if (sig->length_signature != QV_MLDSA65_SIGNATURE_SIZE ||
        OQS_SIG_sign(sig, signature, &signature_len,
                     message, message_len, secret_key) != OQS_SUCCESS)
        goto cleanup;

    if (signature_len != QV_MLDSA65_SIGNATURE_SIZE)
        goto cleanup;

    result = 0;

cleanup:
    OQS_SIG_free(sig);
    return result;
}

int qv_mldsa65_verify(
    const uint8_t *message,
    size_t message_len,
    const uint8_t *signature,
    size_t signature_len,
    const uint8_t *public_key,
    size_t public_key_len)
{
    if (!message || !signature || !public_key ||
        signature_len != QV_MLDSA65_SIGNATURE_SIZE ||
        public_key_len != QV_MLDSA65_PUBLIC_KEY_SIZE)
        return -1;

    OQS_SIG *sig = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);
    if (!sig)
        return -2;

    int result = -3;

    if (OQS_SIG_verify(sig, message, message_len,
                       signature, signature_len,
                       public_key) == OQS_SUCCESS)
        result = 0;

    OQS_SIG_free(sig);
    return result;
}
