#!/bin/bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

echo "=================================="
echo " QuantumVault Crypto Test Runner"
echo "=================================="

echo "[1/4] Checking dependencies..."

if ! pkg-config --exists liboqs openssl; then
    echo "[ERROR] liboqs or OpenSSL development files missing"
    exit 1
fi

echo "[2/4] Compiling ML-KEM test..."

gcc -Wall -Wextra -O2 \
    crypto-engine/kem_encaps_test.c \
    -o crypto-engine/kem_encaps_test \
    $(pkg-config --cflags --libs liboqs openssl)

echo "[3/4] Compiling AES-GCM test..."

gcc -Wall -Wextra -O2 \
    crypto-engine/kem_aead_test.c \
    -o crypto-engine/kem_aead_test \
    $(pkg-config --cflags --libs liboqs openssl)

echo "[4/4] Running tests..."

echo ""
echo "--- ML-KEM-768 Test ---"
./crypto-engine/kem_encaps_test

echo ""
echo "--- HKDF + AES-256-GCM Test ---"
./crypto-engine/kem_aead_test

echo ""
echo "[SUCCESS] All crypto tests passed."
