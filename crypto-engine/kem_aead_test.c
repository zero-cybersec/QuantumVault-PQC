#include <oqs/oqs.h>

#include <openssl/evp.h>
#include <openssl/kdf.h>
#include <openssl/rand.h>
#include <openssl/crypto.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define KEY_LEN 32
#define NONCE_LEN 12
#define TAG_LEN 16

static int derive_aes_key(const uint8_t *shared_secret,
                          size_t secret_len,
                          uint8_t *aes_key)
{
    EVP_PKEY_CTX *ctx = EVP_PKEY_CTX_new_id(EVP_PKEY_HKDF, NULL);
    size_t key_len = KEY_LEN;
    int ok = 0;

    if (!ctx)
        return 0;

    if (EVP_PKEY_derive_init(ctx) <= 0)
        goto cleanup;

    if (EVP_PKEY_CTX_set_hkdf_md(ctx, EVP_sha256()) <= 0)
        goto cleanup;

    if (EVP_PKEY_CTX_set1_hkdf_salt(
            ctx, (const unsigned char *)"QuantumVault-v1", 15) <= 0)
        goto cleanup;

    if (EVP_PKEY_CTX_set1_hkdf_key(ctx, shared_secret,
                                   (int)secret_len) <= 0)
        goto cleanup;

    if (EVP_PKEY_CTX_add1_hkdf_info(
            ctx, (const unsigned char *)"file-encryption", 15) <= 0)
        goto cleanup;

    if (EVP_PKEY_derive(ctx, aes_key, &key_len) <= 0)
        goto cleanup;

    ok = (key_len == KEY_LEN);

cleanup:
    EVP_PKEY_CTX_free(ctx);
    return ok;
}

static int encrypt_gcm(const uint8_t *key,
                       const uint8_t *nonce,
                       const uint8_t *plaintext,
                       int plaintext_len,
                       uint8_t *ciphertext,
                       uint8_t *tag)
{
    EVP_CIPHER_CTX *ctx = EVP_CIPHER_CTX_new();
    int len = 0, total = 0, ok = 0;

    if (!ctx)
        return 0;

    if (EVP_EncryptInit_ex(ctx, EVP_aes_256_gcm(),
                           NULL, NULL, NULL) != 1)
        goto cleanup;

    if (EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_IVLEN,
                            NONCE_LEN, NULL) != 1)
        goto cleanup;

    if (EVP_EncryptInit_ex(ctx, NULL, NULL, key, nonce) != 1)
        goto cleanup;

    if (EVP_EncryptUpdate(ctx, ciphertext, &len,
                          plaintext, plaintext_len) != 1)
        goto cleanup;

    total = len;

    if (EVP_EncryptFinal_ex(ctx, ciphertext + total, &len) != 1)
        goto cleanup;

    total += len;

    if (EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_GET_TAG,
                            TAG_LEN, tag) != 1)
        goto cleanup;

    ok = (total == plaintext_len);

cleanup:
    EVP_CIPHER_CTX_free(ctx);
    return ok;
}

static int decrypt_gcm(const uint8_t *key,
                       const uint8_t *nonce,
                       const uint8_t *ciphertext,
                       int ciphertext_len,
                       const uint8_t *tag,
                       uint8_t *plaintext)
{
    EVP_CIPHER_CTX *ctx = EVP_CIPHER_CTX_new();
    int len = 0, total = 0, ok = 0;

    if (!ctx)
        return 0;

    if (EVP_DecryptInit_ex(ctx, EVP_aes_256_gcm(),
                           NULL, NULL, NULL) != 1)
        goto cleanup;

    if (EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_IVLEN,
                            NONCE_LEN, NULL) != 1)
        goto cleanup;

    if (EVP_DecryptInit_ex(ctx, NULL, NULL, key, nonce) != 1)
        goto cleanup;

    if (EVP_DecryptUpdate(ctx, plaintext, &len,
                          ciphertext, ciphertext_len) != 1)
        goto cleanup;

    total = len;

    if (EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_TAG,
                            TAG_LEN, (void *)tag) != 1)
        goto cleanup;

    if (EVP_DecryptFinal_ex(ctx, plaintext + total, &len) != 1)
        goto cleanup;

    total += len;
    ok = (total == ciphertext_len);

cleanup:
    EVP_CIPHER_CTX_free(ctx);
    return ok;
}

int main(void)
{
    OQS_KEM *kem = NULL;
    uint8_t *pk = NULL, *sk = NULL;
    uint8_t *ct_kem = NULL, *ss_enc = NULL, *ss_dec = NULL;
    uint8_t aes_key[KEY_LEN] = {0};
    uint8_t nonce[NONCE_LEN] = {0};
    uint8_t tag[TAG_LEN] = {0};

    const uint8_t message[] = "QuantumVault Day 4 test";
    const int message_len = (int)(sizeof(message) - 1);

    uint8_t ciphertext[128] = {0};
    uint8_t recovered[128] = {0};
    uint8_t tampered[128] = {0};

    int result = EXIT_FAILURE;

    kem = OQS_KEM_new(OQS_KEM_alg_ml_kem_768);
    if (!kem) {
        fprintf(stderr, "[ERROR] ML-KEM-768 unavailable\n");
        goto cleanup;
    }

    pk = malloc(kem->length_public_key);
    sk = malloc(kem->length_secret_key);
    ct_kem = malloc(kem->length_ciphertext);
    ss_enc = malloc(kem->length_shared_secret);
    ss_dec = malloc(kem->length_shared_secret);

    if (!pk || !sk || !ct_kem || !ss_enc || !ss_dec) {
        fprintf(stderr, "[ERROR] Allocation failed\n");
        goto cleanup;
    }

    if (OQS_KEM_keypair(kem, pk, sk) != OQS_SUCCESS ||
        OQS_KEM_encaps(kem, ct_kem, ss_enc, pk) != OQS_SUCCESS ||
        OQS_KEM_decaps(kem, ss_dec, ct_kem, sk) != OQS_SUCCESS) {
        fprintf(stderr, "[ERROR] ML-KEM operation failed\n");
        goto cleanup;
    }

    if (memcmp(ss_enc, ss_dec, kem->length_shared_secret) != 0) {
        fprintf(stderr, "[ERROR] Shared secrets differ\n");
        goto cleanup;
    }

    printf("[+] ML-KEM shared secret verified\n");

    if (!derive_aes_key(ss_enc, kem->length_shared_secret, aes_key)) {
        fprintf(stderr, "[ERROR] HKDF key derivation failed\n");
        goto cleanup;
    }

    printf("[+] HKDF-SHA256 AES-256 key derived\n");

    if (RAND_bytes(nonce, sizeof(nonce)) != 1) {
        fprintf(stderr, "[ERROR] Nonce generation failed\n");
        goto cleanup;
    }

    if (!encrypt_gcm(aes_key, nonce, message, message_len,
                     ciphertext, tag)) {
        fprintf(stderr, "[ERROR] AES-GCM encryption failed\n");
        goto cleanup;
    }

    printf("[+] AES-256-GCM encryption successful\n");

    if (!decrypt_gcm(aes_key, nonce, ciphertext, message_len,
                     tag, recovered)) {
        fprintf(stderr, "[ERROR] AES-GCM decryption failed\n");
        goto cleanup;
    }

    if (memcmp(message, recovered, message_len) != 0) {
        fprintf(stderr, "[ERROR] Plaintext mismatch\n");
        goto cleanup;
    }

    printf("[+] AES-256-GCM decryption successful\n");
    printf("[SUCCESS] Plaintext round-trip verified\n");

    memcpy(tampered, ciphertext, message_len);
    tampered[0] ^= 0x01;

    if (decrypt_gcm(aes_key, nonce, tampered, message_len,
                    tag, recovered)) {
        fprintf(stderr, "[ERROR] Tampering was not detected\n");
        goto cleanup;
    }

    printf("[+] Modified ciphertext rejected by authentication\n");
    printf("[SUCCESS] AES-GCM integrity test passed\n");

    result = EXIT_SUCCESS;

cleanup:
    if (sk)
        OQS_MEM_cleanse(sk, kem->length_secret_key);
    if (ss_enc)
        OQS_MEM_cleanse(ss_enc, kem->length_shared_secret);
    if (ss_dec)
        OQS_MEM_cleanse(ss_dec, kem->length_shared_secret);

    OPENSSL_cleanse(aes_key, sizeof(aes_key));
    OPENSSL_cleanse(recovered, sizeof(recovered));

    free(pk);
    free(sk);
    free(ct_kem);
    free(ss_enc);
    free(ss_dec);
    OQS_KEM_free(kem);

    return result;
}
