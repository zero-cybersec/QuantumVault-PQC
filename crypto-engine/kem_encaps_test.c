
#include <oqs/oqs.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void)
{
    OQS_KEM *kem = NULL;
    uint8_t *public_key = NULL;
    uint8_t *secret_key = NULL;
    uint8_t *ciphertext = NULL;
    uint8_t *shared_enc = NULL;
    uint8_t *shared_dec = NULL;
    int result = EXIT_FAILURE;

    kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);

    if (kem == NULL) {
        fprintf(stderr, "[ERROR] ML-KEM-768 unavailable\n");
        goto cleanup;
    }

    public_key = malloc(kem->length_public_key);
    secret_key = malloc(kem->length_secret_key);
    ciphertext = malloc(kem->length_ciphertext);
    shared_enc = malloc(kem->length_shared_secret);
    shared_dec = malloc(kem->length_shared_secret);

    if (!public_key || !secret_key || !ciphertext ||
        !shared_enc || !shared_dec) {
        fprintf(stderr, "[ERROR] Memory allocation failed\n");
        goto cleanup;
    }

    if (OQS_KEM_keypair(kem, public_key, secret_key) != OQS_SUCCESS) {
        fprintf(stderr, "[ERROR] Keypair generation failed\n");
        goto cleanup;
    }

    printf("[+] ML-KEM-768 keypair generated\n");

    if (OQS_KEM_encaps(kem, ciphertext, shared_enc,
                       public_key) != OQS_SUCCESS) {
        fprintf(stderr, "[ERROR] Encapsulation failed\n");
        goto cleanup;
    }

    printf("[+] Encapsulation successful\n");

    if (OQS_KEM_decaps(kem, shared_dec, ciphertext,
                       secret_key) != OQS_SUCCESS) {
        fprintf(stderr, "[ERROR] Decapsulation failed\n");
        goto cleanup;
    }

    printf("[+] Decapsulation successful\n");

    if (memcmp(shared_enc, shared_dec,
               kem->length_shared_secret) != 0) {
        fprintf(stderr, "[ERROR] Shared secrets do not match\n");
        goto cleanup;
    }

    printf("[SUCCESS] Shared secrets match!\n");
    printf("[INFO] Shared secret size: %zu bytes\n",
           kem->length_shared_secret);

    result = EXIT_SUCCESS;

cleanup:
    if (shared_enc)
        OQS_MEM_cleanse(shared_enc, kem ? kem->length_shared_secret : 0);
    if (shared_dec)
        OQS_MEM_cleanse(shared_dec, kem ? kem->length_shared_secret : 0);
    if (secret_key)
        OQS_MEM_cleanse(secret_key, kem ? kem->length_secret_key : 0);

    free(public_key);
    free(secret_key);
    free(ciphertext);
    free(shared_enc);
    free(shared_dec);

    OQS_KEM_free(kem);

    return result;
}
