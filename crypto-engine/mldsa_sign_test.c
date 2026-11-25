#include <oqs/oqs.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
    OQS_SIG *sig = OQS_SIG_new(OQS_SIG_alg_ml_dsa_65);

    if (sig == NULL) {
        printf("[FAIL] ML-DSA-65 initialization failed\n");
        return 1;
    }

    uint8_t public_key[sig->length_public_key];
    uint8_t secret_key[sig->length_secret_key];
    uint8_t signature[sig->length_signature];
    size_t signature_len = 0;

    const uint8_t message[] = "QuantumVault-PQC ML-DSA-65 signature test";
    size_t message_len = sizeof(message) - 1;

    if (OQS_SIG_keypair(sig, public_key, secret_key) != OQS_SUCCESS) {
        printf("[FAIL] Keypair generation failed\n");
        OQS_SIG_free(sig);
        return 1;
    }

    printf("[PASS] ML-DSA-65 keypair generation\n");

    if (OQS_SIG_sign(sig, signature, &signature_len,
                     message, message_len, secret_key) != OQS_SUCCESS) {
        printf("[FAIL] Signature generation failed\n");
        OQS_MEM_cleanse(secret_key, sig->length_secret_key);
        OQS_SIG_free(sig);
        return 1;
    }

    printf("[PASS] Digital signature generation\n");

    if (OQS_SIG_verify(sig, message, message_len,
                       signature, signature_len, public_key) != OQS_SUCCESS) {
        printf("[FAIL] Valid signature verification failed\n");
        OQS_MEM_cleanse(secret_key, sig->length_secret_key);
        OQS_SIG_free(sig);
        return 1;
    }

    printf("[PASS] Valid signature verification\n");

    uint8_t modified_message[sizeof(message)];
    memcpy(modified_message, message, sizeof(message));
    modified_message[0] ^= 0x01;

    if (OQS_SIG_verify(sig, modified_message, message_len,
                       signature, signature_len, public_key) == OQS_SUCCESS) {
        printf("[FAIL] Modified message was accepted\n");
        OQS_MEM_cleanse(secret_key, sig->length_secret_key);
        OQS_SIG_free(sig);
        return 1;
    }

    printf("[PASS] Modified message rejected\n");

    signature[0] ^= 0x01;

    if (OQS_SIG_verify(sig, message, message_len,
                       signature, signature_len, public_key) == OQS_SUCCESS) {
        printf("[FAIL] Modified signature was accepted\n");
        OQS_MEM_cleanse(secret_key, sig->length_secret_key);
        OQS_SIG_free(sig);
        return 1;
    }

    printf("[PASS] Modified signature rejected\n");

    OQS_MEM_cleanse(secret_key, sig->length_secret_key);
    OQS_SIG_free(sig);

    printf("\nALL ML-DSA-65 TESTS PASSED\n");
    return 0;
}
