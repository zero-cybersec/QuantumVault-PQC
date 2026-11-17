#include "include/quantumvault_crypto.h"
#include <openssl/rand.h>
#include <stdio.h>
#include <string.h>

int main(void)
{
    uint8_t key[QV_AES256_KEY_SIZE];
    uint8_t nonce[QV_GCM_NONCE_SIZE];
    uint8_t tag[QV_GCM_TAG_SIZE];

    const uint8_t plaintext[] = "QuantumVault AES-256-GCM test";
    uint8_t ciphertext[sizeof(plaintext)];
    uint8_t decrypted[sizeof(plaintext)];

    size_t len = sizeof(plaintext);

    if (RAND_bytes(key, sizeof(key)) != 1 ||
        RAND_bytes(nonce, sizeof(nonce)) != 1) {
        printf("[FAIL] Random generation failed\n");
        return 1;
    }

    if (qv_aes256gcm_encrypt(key, nonce, plaintext,
                             len, ciphertext, tag) != 0) {
        printf("[FAIL] Encryption failed\n");
        return 1;
    }

    printf("[PASS] AES-256-GCM encryption\n");

    if (qv_aes256gcm_decrypt(key, nonce, ciphertext,
                             len, tag, decrypted) != 0) {
        printf("[FAIL] Decryption failed\n");
        return 1;
    }

    if (memcmp(plaintext, decrypted, len) != 0) {
        printf("[FAIL] Plaintext mismatch\n");
        return 1;
    }

    printf("[PASS] Decryption and plaintext verification\n");

    tag[0] ^= 0x01;

    if (qv_aes256gcm_decrypt(key, nonce, ciphertext,
                             len, tag, decrypted) == 0) {
        printf("[FAIL] Tampered tag was accepted\n");
        return 1;
    }

    printf("[PASS] Tampered authentication tag rejected\n");

    return 0;
}
