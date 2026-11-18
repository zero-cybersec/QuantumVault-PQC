#include "quantumvault_crypto.h"
#include <stdio.h>

int main(void)
{
    unsigned char buffer[QV_MLKEM768_SECRET_KEY_SIZE] = {0};

    if (qv_mlkem768_keypair(NULL, sizeof(buffer),
                            buffer, sizeof(buffer)) != -1) {
        printf("FAIL: NULL public key accepted\n");
        return 1;
    }
    printf("PASS: NULL public key rejected\n");

    if (qv_mlkem768_keypair(buffer, 10,
                            buffer, sizeof(buffer)) != -1) {
        printf("FAIL: Insufficient public key capacity accepted\n");
        return 1;
    }
    printf("PASS: Insufficient key capacity rejected\n");

    if (qv_mlkem768_encaps(buffer, 10,
                           buffer, sizeof(buffer),
                           buffer, sizeof(buffer)) != -1) {
        printf("FAIL: Invalid public key length accepted\n");
        return 1;
    }
    printf("PASS: Invalid public key length rejected\n");

    if (qv_mlkem768_decaps(buffer, 10,
                           buffer, sizeof(buffer),
                           buffer, sizeof(buffer)) != -1) {
        printf("FAIL: Invalid ciphertext length accepted\n");
        return 1;
    }
    printf("PASS: Invalid ciphertext length rejected\n");

    return 0;
}
