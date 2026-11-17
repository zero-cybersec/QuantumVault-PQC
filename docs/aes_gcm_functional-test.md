# AES-256-GCM Functional Test

## Objective
Verify AES-256-GCM encryption, decryption and authentication.

## Test Cases

1. Generate a random 256-bit encryption key.
2. Generate a random 96-bit nonce.
3. Encrypt plaintext using AES-256-GCM.
4. Decrypt ciphertext and verify plaintext.
5. Modify the authentication tag.
6. Verify that modified authentication data is rejected.

## Expected Results

- Encryption: PASS
- Decryption: PASS
- Plaintext verification: PASS
- Tampered tag rejection: PASS

## Conclusion

AES-256-GCM encryption and authenticated decryption functional tests completed successfully.
