#include "quantumvault_crypto.h"

#include <oqs/oqs.h>
#include <string.h>

int qv_mlkem768_keypair(
    uint8_t *public_key,
    size_t public_key_capacity,
    uint8_t *secret_key,
    size_t secret_key_capacity)
{
    OQS_KEM *kem = NULL;
    int result = -1;

    if (!public_key || !secret_key ||
        public_key_capacity < QV_MLKEM768_PUBLIC_KEY_SIZE ||
        secret_key_capacity < QV_MLKEM768_SECRET_KEY_SIZE) {
        return -1;
    }

    kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!kem) {
        return -2;
    }

    if (kem->length_public_key != QV_MLKEM768_PUBLIC_KEY_SIZE ||
        kem->length_secret_key != QV_MLKEM768_SECRET_KEY_SIZE) {
        result = -3;
        goto cleanup;
    }

    if (OQS_KEM_keypair(kem, public_key, secret_key) != OQS_SUCCESS) {
        OQS_MEM_cleanse(secret_key, kem->length_secret_key);
        result = -4;
        goto cleanup;
    }

    result = 0;

cleanup:
    OQS_KEM_free(kem);
    return result;
}

int qv_mlkem768_encaps(
    const uint8_t *public_key,
    size_t public_key_len,
    uint8_t *ciphertext,
    size_t ciphertext_capacity,
    uint8_t *shared_secret,
    size_t shared_secret_capacity)
{
    OQS_KEM *kem = NULL;
    int result = -1;

    if (!public_key || !ciphertext || !shared_secret ||
        public_key_len != QV_MLKEM768_PUBLIC_KEY_SIZE ||
        ciphertext_capacity < QV_MLKEM768_CIPHERTEXT_SIZE ||
        shared_secret_capacity < QV_MLKEM768_SHARED_SECRET_SIZE) {
        return -1;
    }

    kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!kem) {
        return -2;
    }

    if (kem->length_public_key != QV_MLKEM768_PUBLIC_KEY_SIZE ||
        kem->length_ciphertext != QV_MLKEM768_CIPHERTEXT_SIZE ||
        kem->length_shared_secret != QV_MLKEM768_SHARED_SECRET_SIZE) {
        result = -3;
        goto cleanup;
    }

    if (OQS_KEM_encaps(kem, ciphertext, shared_secret,
                       public_key) != OQS_SUCCESS) {
        OQS_MEM_cleanse(shared_secret, kem->length_shared_secret);
        result = -4;
        goto cleanup;
    }

    result = 0;

cleanup:
    OQS_KEM_free(kem);
    return result;
}

int qv_mlkem768_decaps(
    const uint8_t *ciphertext,
    size_t ciphertext_len,
    const uint8_t *secret_key,
    size_t secret_key_len,
    uint8_t *shared_secret,
    size_t shared_secret_capacity)
{
    OQS_KEM *kem = NULL;
    int result = -1;

    if (!ciphertext || !secret_key || !shared_secret ||
        ciphertext_len != QV_MLKEM768_CIPHERTEXT_SIZE ||
        secret_key_len != QV_MLKEM768_SECRET_KEY_SIZE ||
        shared_secret_capacity < QV_MLKEM768_SHARED_SECRET_SIZE) {
        return -1;
    }

    kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!kem) {
        return -2;
    }

    if (kem->length_secret_key != QV_MLKEM768_SECRET_KEY_SIZE ||
        kem->length_ciphertext != QV_MLKEM768_CIPHERTEXT_SIZE ||
        kem->length_shared_secret != QV_MLKEM768_SHARED_SECRET_SIZE) {
        result = -3;
        goto cleanup;
    }

    if (OQS_KEM_decaps(kem, shared_secret, ciphertext,
                       secret_key) != OQS_SUCCESS) {
        OQS_MEM_cleanse(shared_secret, kem->length_shared_secret);
        result = -4;
        goto cleanup;
    }

    result = 0;

cleanup:
    OQS_KEM_free(kem);
    return result;
}
