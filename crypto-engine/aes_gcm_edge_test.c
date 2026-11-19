#include "include/quantumvault_crypto.h"
#include <openssl/rand.h>
#include <stdio.h>
#include <string.h>

#define TEST_SIZE 32

static int check_failure(const char *name, int result)
{
    if (result == 0) {
        printf("FAIL: %s was accepted\n", name);
        return 1;
    }

    printf("PASS: %s rejected\n", name);
    return 0;
}

int main(void)
{
    uint8_t key[QV_AES256_KEY_SIZE];
    uint8_t wrong_key[QV_AES256_KEY_SIZE];
    uint8_t nonce[QV_GCM_NONCE_SIZE];
    uint8_t wrong_nonce[QV_GCM_NONCE_SIZE];
    uint8_t tag[QV_GCM_TAG_SIZE];

    const uint8_t plaintext[] = "QuantumVault Day 8 security test";

    uint8_t ciphertext[sizeof(plaintext)];
    uint8_t decrypted[sizeof(plaintext)];

    int failures = 0;

    if (RAND_bytes(key, sizeof(key)) != 1 ||
        RAND_bytes(wrong_key, sizeof(wrong_key)) != 1 ||
        RAND_bytes(nonce, sizeof(nonce)) != 1) {
        printf("FAIL: Random generation\n");
        return 1;
    }

    memcpy(wrong_nonce, nonce, sizeof(nonce));
    wrong_nonce[0] ^= 0x01;

    size_t len = sizeof(plaintext);

    if (qv_aes256gcm_encrypt(
            key, nonce, plaintext, len, ciphertext, tag) != 0) {
        printf("FAIL: Initial encryption\n");
        return 1;
    }

    printf("PASS: Initial encryption\n");

    if (qv_aes256gcm_decrypt(
            key, nonce, ciphertext, len, tag, decrypted) != 0 ||
        memcmp(plaintext, decrypted, len) != 0) {
        printf("FAIL: Initial decryption\n");
        return 1;
    }

    printf("PASS: Initial decryption and plaintext verification\n");

    /* Test 1: Wrong key */
    memset(decrypted, 0xAA, sizeof(decrypted));

    failures += check_failure(
        "Wrong key",
        qv_aes256gcm_decrypt(
            wrong_key, nonce, ciphertext, len, tag, decrypted));

    /* Test 2: Modified ciphertext */
    ciphertext[0] ^= 0x01;
    memset(decrypted, 0xAA, sizeof(decrypted));

    failures += check_failure(
        "Modified ciphertext",
        qv_aes256gcm_decrypt(
            key, nonce, ciphertext, len, tag, decrypted));

    ciphertext[0] ^= 0x01;

    /* Test 3: Modified nonce */
    memset(decrypted, 0xAA, sizeof(decrypted));

    failures += check_failure(
        "Modified nonce",
        qv_aes256gcm_decrypt(
            key, wrong_nonce, ciphertext, len, tag, decrypted));

    /* Test 4: NULL key */
    failures += check_failure(
        "NULL key",
        qv_aes256gcm_decrypt(
            NULL, nonce, ciphertext, len, tag, decrypted));

    /* Test 5: NULL plaintext input */
    failures += check_failure(
        "NULL encryption plaintext",
        qv_aes256gcm_encrypt(
            key, nonce, NULL, len, ciphertext, tag));

    /* Test 6: Zero-length plaintext is valid for AES-GCM */
    if (qv_aes256gcm_encrypt(
            key, nonce, plaintext, 0, ciphertext, tag) != 0) {
        printf("FAIL: Zero-length encryption failed\n");
        failures++;
    } else if (qv_aes256gcm_decrypt(
                   key, nonce, ciphertext, 0, tag, decrypted) != 0) {
        printf("FAIL: Zero-length decryption failed\n");
        failures++;
    } else {
        printf("PASS: Zero-length encryption/decryption\n");
    }

    if (failures == 0) {
        printf("\nALL EDGE CASE TESTS PASSED\n");
        return 0;
    }

    printf("\n%d EDGE CASE TEST(S) FAILED\n", failures);
    return 1;
}
