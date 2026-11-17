#include <oqs/oqs.h>
#include <stdint.h>
#include <stddef.h>

int qv_mlkem768_keypair(uint8_t *public_key, uint8_t *secret_key) {
    OQS_KEM *kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);

    if (kem == NULL) {
        return -1;
    }

    OQS_STATUS status = OQS_KEM_keypair(kem, public_key, secret_key);

    OQS_KEM_free(kem);
    return status == OQS_SUCCESS ? 0 : -1;
}

size_t qv_mlkem768_public_key_length(void) {
    OQS_KEM *kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (kem == NULL) return 0;

    size_t length = kem->length_public_key;
    OQS_KEM_free(kem);
    return length;
}

size_t qv_mlkem768_secret_key_length(void) {
    OQS_KEM *kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (kem == NULL) return 0;

    size_t length = kem->length_secret_key;
    OQS_KEM_free(kem);
    return length;
}
