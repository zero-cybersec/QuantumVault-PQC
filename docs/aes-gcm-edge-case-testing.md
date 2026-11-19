# QuantumVault – AES-GCM Edge Case Testing

## Objective
Validate AES-256-GCM encryption and decryption against invalid inputs and authentication failures.

## Test Cases

| Test Case | Expected Result | Status |
|---|---|---|
| Initial encryption | Successful | PASS |
| Initial decryption | Successful | PASS |
| Wrong key | Rejected | PASS |
| Modified ciphertext | Rejected | PASS |
| Modified nonce | Rejected | PASS |
| NULL key | Rejected | PASS |
| NULL plaintext | Rejected | PASS |
| Zero-length plaintext | Successfully processed | PASS |

## Security Observations
- Authentication failures must not release unauthenticated plaintext.
- Modified ciphertext and nonce must fail authentication.
- Invalid NULL arguments must be rejected.
- AES-GCM supports zero-length plaintext.

## Verification
All edge-case tests passed successfully.

Test source: `crypto-engine/aes_gcm_edge_test.c`
Implementation: `crypto-engine/quantumvault_crypto.c`

## Conclusion
AES-256-GCM edge-case testing completed successfully.
