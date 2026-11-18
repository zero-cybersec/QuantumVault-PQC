#include "quantumvault_crypto.h"
#include <oqs/oqs.h>

#include <stdio.h>
#include <string.h>

int main(void)
{
    uint8_t public_key[QV_MLKEM768_PUBLIC_KEY_SIZE];
    uint8_t secret_key[QV_MLKEM768_SECRET_KEY_SIZE];
    uint8_t ciphertext[QV_MLKEM768_CIPHERTEXT_SIZE];
    uint8_t shared_enc[QV_MLKEM768_SHARED_SECRET_SIZE];
    uint8_t shared_dec[QV_MLKEM768_SHARED_SECRET_SIZE];

    memset(secret_key, 0, sizeof(secret_key));
    memset(shared_enc, 0, sizeof(shared_enc));
    memset(shared_dec, 0, sizeof(shared_dec));

    if (qv_mlkem768_keypair(
            public_key, sizeof(public_key),
            secret_key, sizeof(secret_key)) != 0) {
        fprintf(stderr, "FAIL: Keypair generation\n");
        return 1;
    }

    printf("PASS: ML-KEM-768 keypair generation\n");

    if (qv_mlkem768_encaps(
            public_key, sizeof(public_key),
            ciphertext, sizeof(ciphertext),
            shared_enc, sizeof(shared_enc)) != 0) {
        fprintf(stderr, "FAIL: Encapsulation\n");
        return 1;
    }

    printf("PASS: ML-KEM-768 encapsulation\n");

    if (qv_mlkem768_decaps(
            ciphertext, sizeof(ciphertext),
            secret_key, sizeof(secret_key),
            shared_dec, sizeof(shared_dec)) != 0) {
        fprintf(stderr, "FAIL: Decapsulation\n");
        return 1;
    }

    printf("PASS: ML-KEM-768 decapsulation\n");

    if (memcmp(shared_enc, shared_dec, sizeof(shared_enc)) != 0) {
        fprintf(stderr, "FAIL: Shared secrets do not match\n");
        return 1;
    }

    printf("PASS: Shared secrets match\n");

    OQS_MEM_cleanse(secret_key, sizeof(secret_key));
    OQS_MEM_cleanse(shared_enc, sizeof(shared_enc));
    OQS_MEM_cleanse(shared_dec, sizeof(shared_dec));

    return 0;
}
