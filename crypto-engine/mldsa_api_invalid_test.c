#include "include/quantumvault_crypto.h"
#include <oqs/oqs.h>
#include <stdio.h>
#include <string.h>

#define CHECK(name, condition) do { \
    if (condition) { \
        printf("PASS: %s\n", name); \
    } else { \
        printf("FAIL: %s\n", name); \
        failures++; \
    } \
} while (0)

int main(void) {
    int failures = 0;

    uint8_t pk[QV_MLDSA65_PUBLIC_KEY_SIZE];
    uint8_t sk[QV_MLDSA65_SECRET_KEY_SIZE];
    uint8_t sig[QV_MLDSA65_SIGNATURE_SIZE];

    const uint8_t msg[] = "QuantumVault invalid input test";

    CHECK("NULL public key rejected",
        qv_mldsa65_keypair(NULL, sizeof(pk), sk, sizeof(sk)) != 0);

    CHECK("Insufficient public key capacity rejected",
        qv_mldsa65_keypair(pk, 10, sk, sizeof(sk)) != 0);

    CHECK("NULL secret key rejected",
        qv_mldsa65_keypair(pk, sizeof(pk), NULL, sizeof(sk)) != 0);

    CHECK("Valid keypair generated",
        qv_mldsa65_keypair(pk, sizeof(pk), sk, sizeof(sk)) == 0);

    CHECK("NULL message rejected",
        qv_mldsa65_sign(NULL, sizeof(msg) - 1, sk, sizeof(sk),
                        sig, sizeof(sig)) != 0);

    CHECK("Invalid secret key length rejected",
        qv_mldsa65_sign(msg, sizeof(msg) - 1, sk, 10,
                        sig, sizeof(sig)) != 0);

    CHECK("Insufficient signature capacity rejected",
        qv_mldsa65_sign(msg, sizeof(msg) - 1, sk, sizeof(sk),
                        sig, 10) != 0);

    CHECK("Valid signature generated",
        qv_mldsa65_sign(msg, sizeof(msg) - 1, sk, sizeof(sk),
                        sig, sizeof(sig)) == 0);

    CHECK("Invalid signature length rejected",
        qv_mldsa65_verify(msg, sizeof(msg) - 1, sig, 10,
                          pk, sizeof(pk)) != 0);

    CHECK("Invalid public key length rejected",
        qv_mldsa65_verify(msg, sizeof(msg) - 1, sig, sizeof(sig),
                          pk, 10) != 0);

    CHECK("NULL public key rejected during verification",
        qv_mldsa65_verify(msg, sizeof(msg) - 1, sig, sizeof(sig),
                          NULL, sizeof(pk)) != 0);

    OQS_MEM_cleanse(sk, sizeof(sk));

    printf("\nInvalid input test failures: %d\n", failures);

    return failures == 0 ? 0 : 1;
}
