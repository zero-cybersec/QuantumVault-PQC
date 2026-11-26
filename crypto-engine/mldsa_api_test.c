#include "include/quantumvault_crypto.h"
#include <oqs/oqs.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
    uint8_t public_key[QV_MLDSA65_PUBLIC_KEY_SIZE];
    uint8_t secret_key[QV_MLDSA65_SECRET_KEY_SIZE];
    uint8_t signature[QV_MLDSA65_SIGNATURE_SIZE];

    const uint8_t message[] = "QuantumVault Day 12 API test";
    uint8_t modified_message[sizeof(message)];

    memcpy(modified_message, message, sizeof(message));

    if (qv_mldsa65_keypair(
        public_key, sizeof(public_key),
        secret_key, sizeof(secret_key)) != 0) {
        printf("FAIL: Keypair generation\n");
        return 1;
    }

    printf("PASS: ML-DSA-65 API keypair generation\n");

    if (qv_mldsa65_sign(
        message, sizeof(message) - 1,
        secret_key, sizeof(secret_key),
        signature, sizeof(signature)) != 0) {
        printf("FAIL: Signature generation\n");
        return 1;
    }

    printf("PASS: ML-DSA-65 API signature generation\n");

    if (qv_mldsa65_verify(
        message, sizeof(message) - 1,
        signature, sizeof(signature),
        public_key, sizeof(public_key)) != 0) {
        printf("FAIL: Signature verification\n");
        return 1;
    }

    printf("PASS: Valid signature verification\n");

    modified_message[0] ^= 0x01;

    if (qv_mldsa65_verify(
        modified_message, sizeof(message) - 1,
        signature, sizeof(signature),
        public_key, sizeof(public_key)) == 0) {
        printf("FAIL: Modified message accepted\n");
        return 1;
    }

    printf("PASS: Modified message rejected\n");

    OQS_MEM_cleanse(secret_key, sizeof(secret_key));

    printf("\nALL ML-DSA API TESTS PASSED\n");
    return 0;
}
