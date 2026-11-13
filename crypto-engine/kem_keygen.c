#include <oqs/oqs.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main(void)
{
    OQS_KEM *kem = NULL;
    uint8_t *public_key = NULL;
    uint8_t *secret_key = NULL;
    int result = EXIT_FAILURE;

    /* Initialize ML-KEM-768 */
    kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);

    if (kem == NULL) {
        fprintf(stderr, "[ERROR] ML-KEM-768 is unavailable\n");
        goto cleanup;
    }

    printf("[INFO] Algorithm: %s\n", kem->method_name);

    /* Allocate key buffers */
    public_key = malloc(kem->length_public_key);
    secret_key = malloc(kem->length_secret_key);

    if (public_key == NULL || secret_key == NULL) {
        fprintf(stderr, "[ERROR] Memory allocation failed\n");
        goto cleanup;
    }

    /* Generate keypair */
    if (OQS_KEM_keypair(kem, public_key, secret_key) != OQS_SUCCESS) {
        fprintf(stderr, "[ERROR] Keypair generation failed\n");
        goto cleanup;
    }

    printf("[SUCCESS] ML-KEM-768 keypair generated\n");
    printf("[INFO] Public key size: %zu bytes\n",
           kem->length_public_key);
    printf("[INFO] Secret key size: %zu bytes\n",
           kem->length_secret_key);

    /* Do not print or persist secret key material */
    result = EXIT_SUCCESS;

cleanup:
    if (secret_key != NULL) {
        OQS_MEM_cleanse(secret_key, kem->length_secret_key);
        free(secret_key);
    }

    free(public_key);
    OQS_KEM_free(kem);

    return result;
}
