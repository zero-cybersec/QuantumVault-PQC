# QuantumVault: CLI Specification, Integration & Testing Plan



## 1. Executive Overview

QuantumVault is a zero-trust, post-quantum cryptographic secrets vault engineered to protect enterprise credentials, cryptographic keys, and application configurations against classical and quantum cryptanalytic threats. 

This specification establishes:
1. **CLI Surface & Command Hierarchy**: The primary interface (`qv`) for local and remote operations.
2. **Vault Workflow Test Cases**: End-to-end integration journeys verifying initialization, authentication, secret storage, key rotation, and disaster recovery.
3. **CLI I/O Contract**: Canonical syntax, options, structured JSON outputs, and POSIX exit codes.
4. **Initial Testing Plan**: Verification strategy spanning cryptographic Known Answer Tests (KATs), CLI integration harnesses, fuzz testing, and CI/CD gating.

---

## 2. CLI Command Specification

The command-line interface binary is named `qv` (aliased to `quantumvault`).

### 2.1 Global Flags & Environment Variables

| Flag | Short | Environment Variable | Default | Description |
| :--- | :--- | :--- | :--- | :--- |
| `--format` | `-f` | `QV_FORMAT` | `text` | Output format: `text`, `json`, or `yaml`. |
| `--vault-dir` | `-d` | `QV_VAULT_DIR` | `~/.quantumvault` | Directory containing encrypted vault storage. |
| `--config` | `-c` | `QV_CONFIG` | `~/.quantumvault/config.toml` | Path to client configuration file. |
| `--verbose` | `-v` | `QV_VERBOSE` | `false` | Enable verbose diagnostic logging (`DEBUG`). |
| `--quiet` | `-q` | `QV_QUIET` | `false` | Silence non-essential stdout output. |
| `--no-color` | | `NO_COLOR` | `false` | Strip ANSI escape sequences from stdout/stderr. |
| `--version` | `-V` | | | Display version, build commit, and PQC engine info. |

---

### 2.2 Command Hierarchy

```
qv
├── init                  # Initialize vault, derive root keys, output recovery shards
├── auth
│   ├── login             # Authenticate and unlock session token into secure memory
│   ├── logout            # Invalidate active session and scrub ephemeral keys
│   └── status            # Show current session status, lease TTL, and identity
├── secret
│   ├── put               # Store or update an encrypted secret payload
│   ├── get               # Decrypt and display or extract a secret payload
│   ├── list              # List secret paths matching hierarchy or prefix
│   ├── delete            # Soft or hard delete secret entry
│   └── history           # View historical versions and audit metadata of a path
├── key
│   ├── status            # Inspect active KEM/signature key generation & cipher suites
│   ├── rotate            # Trigger root/sub-key rotation and optional re-encryption
│   └── export-public     # Export public key for asymmetric client-side encryption
├── policy
│   ├── list              # List active access control policies
│   ├── apply             # Apply/update RBAC policy rules from file
│   └── test              # Evaluate access rules against a hypothetical identity
├── audit
│   ├── verify            # Validate Merkle/hash-chain integrity of audit logs
│   └── tail              # Stream or inspect recent audit log entries
└── status                # Health check, lock state, and cryptographic engine details
```

---

### 2.3 Detailed Command Definitions

#### 2.3.1 `qv init`
Initializes a new encrypted vault storage layer, establishes the master key hierarchy using Argon2id + ML-KEM-768/1024 hybrid encapsulation, and generates Shamir M-of-N post-quantum recovery shards.

- **Syntax:** `qv init [OPTIONS]`
- **Options:**
  - `--shares <N>`: Total recovery shards to generate (default: `5`).
  - `--threshold <M>`: Minimum shards required for unsealing/recovery (default: `3`).
  - `--algorithm <ALG>`: PQC algorithm suite (`hybrid-kyber768-x25519` [default], `ml-kem-1024`, `classic-aes256`).
  - `--non-interactive`: Disable interactive prompts; reads master passphrase from `QV_MASTER_PASSPHRASE`.

#### 2.3.2 `qv auth login`
Authenticates an operator or workload, unseals the local cryptographic cache or issues a time-bound session lease stored in OS-level protected memory (e.g., Linux keyring / macOS Keychain).

- **Syntax:** `qv auth login [OPTIONS]`
- **Options:**
  - `--ttl <DURATION>`: Session validity duration (e.g., `1h`, `8h`, default: `4h`).
  - `--identity <ID>`: Specific role or identity identifier.
  - `--key-file <PATH>`: Path to local private signing key (ML-DSA-65) if public-key auth is configured.

#### 2.3.3 `qv secret put`
Encrypts and commits an arbitrary secret (key-value, JSON string, or binary file) to the vault path.

- **Syntax:** `qv secret put <PATH> [OPTIONS]`
- **Arguments:**
  - `<PATH>`: Secret identifier (e.g., `production/database/postgres_url`).
- **Options:**
  - `--value <STR>` / `-v`: Inline plaintext secret value.
  - `--file <PATH>` / `-F`: Read secret value from target file.
  - `--stdin`: Read secret value from standard input pipe.
  - `--tag <KEY=VALUE>`: Custom metadata attributes (repeatable).
  - `--ttl <DURATION>`: Automatic secret expiry window (optional).

#### 2.3.4 `qv secret get`
Fetches, verifies integrity tag, and decrypts the secret at `<PATH>`.

- **Syntax:** `qv secret get <PATH> [OPTIONS]`
- **Arguments:**
  - `<PATH>`: Secret identifier.
- **Options:**
  - `--version <INT>`: Retrieve specific historical revision (default: latest).
  - `--field <KEY>`: Extract specific key from JSON-structured secrets.
  - `--output-file <PATH>`: Write raw secret directly to file with `0600` permissions.
  - `--metadata-only`: Return metadata (version, author, modified date) without decrypting payload.

#### 2.3.5 `qv secret list`
Lists all stored secret paths matching a prefix or recursive pattern.

- **Syntax:** `qv secret list [PREFIX] [OPTIONS]`
- **Options:**
  - `--recursive` / `-r`: Traverse full path hierarchy.
  - `--detailed`: Include version count, creation timestamp, and encryption algorithm tag.

#### 2.3.6 `qv secret delete`
Deletes an existing secret or specific version.

- **Syntax:** `qv secret delete <PATH> [OPTIONS]`
- **Options:**
  - `--version <INT>`: Delete only specified version.
  - `--hard` / `--permanent`: Permanently shred data blocks (bypasses tombstone grace period).
  - `--force` / `-f`: Suppress confirmation prompt.

#### 2.3.7 `qv key rotate`
Rotates the data-encryption key (DEK) or key-encryption key (KEK) using quantum-resistant key encapsulation and optional background re-encryption of stored ciphertexts.

- **Syntax:** `qv key rotate [OPTIONS]`
- **Options:**
  - `--re-encrypt-all`: Re-encrypt all existing ciphertexts using the new key version.
  - `--algorithm <ALG>`: Switch or upgrade PQC suite during rotation.

---

## 3. Expected CLI Inputs and Outputs

### 3.1 POSIX Exit Codes

| Exit Code | Constant | Meaning |
| :---: | :--- | :--- |
| `0` | `QV_EXIT_SUCCESS` | Command completed successfully. |
| `1` | `QV_EXIT_GENERAL_ERROR` | Unhandled error or internal runtime exception. |
| `2` | `QV_EXIT_INVALID_ARGS` | Invalid flag, missing argument, or malformed parameter. |
| `3` | `QV_EXIT_VAULT_LOCKED` | Vault is uninitialized, locked, or session lease has expired. |
| `4` | `QV_EXIT_SECRET_NOT_FOUND` | Path does not exist or target version was destroyed. |
| `5` | `QV_EXIT_ACCESS_DENIED` | Policy engine denied access (RBAC/ABAC violation). |
| `6` | `QV_EXIT_INTEGRITY_FAIL` | Cryptographic signature or HMAC/GCM authentication tag failure. |
| `7` | `QV_EXIT_PQC_ALGO_ERROR` | Unsupported post-quantum parameter set or hardware acceleration fault. |

---

### 3.2 Canonical Input/Output Examples

#### 3.2.1 Vault Initialization (`qv init`)

**Command:**
```bash
qv init --shares 5 --threshold 3 --algorithm hybrid-kyber768-x25519 --format json
```

**Standard Output (JSON):**
```json
{
  "status": "initialized",
  "vault_id": "qv-vault-88fa3910c2",
  "pqc_suite": "hybrid-kyber768-x25519",
  "shares_total": 5,
  "shares_threshold": 3,
  "recovery_shards": [
    "qv-shard-01-7fa890e0c5b3d4f198...",
    "qv-shard-02-1a4c9b88f3e2710dae...",
    "qv-shard-03-99b0c43ef81200fa44...",
    "qv-shard-04-c3e10fa789b524d671...",
    "qv-shard-05-65d28b17a04e389fc2..."
  ],
  "root_fingerprint": "SHA256:d8b2e1f4098ca39f60b81209e5adcb9e283",
  "created_at": "2026-11-11T18:30:00Z"
}
```

#### 3.2.2 Storing a Secret (`qv secret put`)

**Command:**
```bash
echo -n "sk_live_quantum_secure_key_99812" | qv secret put api/stripe/primary --stdin --tag environment=prod --format json
```

**Standard Output (JSON):**
```json
{
  "path": "api/stripe/primary",
  "version": 1,
  "checksum_sha256": "4b227777d4dd1fc61c6f884f48641d02b4d121d3fd328cb08b5531fcacdabf8a",
  "cipher_metadata": {
    "algorithm": "ML-KEM-768+AES-256-GCM",
    "key_version": 1,
    "payload_bytes": 33
  },
  "tags": {
    "environment": "prod"
  },
  "created_at": "2026-11-11T18:32:15Z"
}
```

#### 3.2.3 Retrieving a Secret (`qv secret get`)

**Command (Human-Readable Plaintext):**
```bash
qv secret get api/stripe/primary
```
**Standard Output:**
```
sk_live_quantum_secure_key_99812
```

**Command (Structured JSON with Metadata):**
```bash
qv secret get api/stripe/primary --format json
```

**Standard Output (JSON):**
```json
{
  "path": "api/stripe/primary",
  "value": "sk_live_quantum_secure_key_99812",
  "version": 1,
  "created_at": "2026-11-11T18:32:15Z",
  "last_accessed_at": "2026-11-11T18:35:40Z",
  "pqc_verification": "VALID"
}
```

#### 3.2.4 Error Representation (JSON)

When `--format json` is active, errors on stderr follow standard RFC-7807 problem details:

```json
{
  "error": {
    "code": "QV_EXIT_VAULT_LOCKED",
    "exit_code": 3,
    "message": "Vault is currently locked. An active session is required.",
    "resolution": "Run 'qv auth login' or provide valid QV_SESSION_TOKEN environment variable."
  }
}
```

---

## 4. Vault Workflow Test Cases

The test suite validates both happy-path operator workflows and resilient boundary conditions.

| Test Case ID | Workflow Name | Description & Target | Prerequisites | Step-by-Step Procedure | Expected Result | Exit Code |
| :--- | :--- | :--- | :--- | :--- | :--- | :---: |
| **TC-WF-01** | Initial Setup & Key Derivation | Verify clean vault initialization and generation of threshold recovery shards. | Empty target directory `~/.qv-test-vault`. | 1. Run `qv init --vault-dir ~/.qv-test-vault --shares 5 --threshold 3`.<br>2. Verify generated filesystem tree.<br>3. Inspect manifest for algorithm header `hybrid-kyber768-x25519`. | Key files created, 5 recovery shards emitted, manifest integrity verified. | `0` |
| **TC-WF-02** | Master Authentication & Session Lease | Test passphrase authentication, session token issuance, and TTL expiration. | Initialized vault (TC-WF-01). | 1. Run `qv auth login --ttl 2s`.<br>2. Check `qv auth status` (verifies session active).<br>3. Sleep for 3 seconds.<br>4. Re-run `qv auth status`. | Immediate status succeeds; post-TTL command fails with session expired notice. | `0` (step 2), `3` (step 4) |
| **TC-WF-03** | Secret CRUD & Cryptographic Round-Trip | Ensure end-to-end encryption, byte-for-byte decryption fidelity, version incrementing, and deletion. | Active authenticated session. | 1. Store plaintext string via `qv secret put app/db/url --value "postgres://prod"`.<br>2. Retrieve via `qv secret get app/db/url`.<br>3. Put updated value.<br>4. Retrieve with `--version 1` and `--version 2`.<br>5. Delete path with `qv secret delete app/db/url`. | Plaintext restored exactly; version history preserved; subsequent `get` returns 404/not found. | `0` (1-4), `4` (post-delete) |
| **TC-WF-04** | PQC Key Rotation & Re-encryption | Validate dynamic rotation of Root Key Encryption Key (KEK) using ML-KEM without data corruption. | Vault with existing secrets. | 1. Store 10 secrets across paths.<br>2. Execute `qv key rotate --re-encrypt-all`.<br>3. Inspect key version header.<br>4. Read each of the 10 secrets. | KEK version increments; all 10 secrets decrypt cleanly with zero data loss. | `0` |
| **TC-WF-05** | Access Control & Policy Enforcement | Verify that unauthorized role cannot read restricted vault paths. | Admin session & read-only role token. | 1. Apply policy granting role `analytics` access only to `reports/*`.<br>2. Authenticate as `analytics`.<br>3. Attempt `qv secret get reports/daily` (Allowed).<br>4. Attempt `qv secret get credentials/ssh` (Forbidden). | Allowed path returns payload; restricted path rejected with Policy Violation error. | `0` (step 3), `5` (step 4) |
| **TC-WF-06** | Data Tampering & Integrity Detection | Verify cryptographic authentication tag failure when raw ciphertext is manipulated on disk. | Stored secret at `data/blob`. | 1. Locate encrypted storage block on filesystem.<br>2. Flip 1 bit in ciphertext payload.<br>3. Attempt `qv secret get data/blob`. | Decryption halts immediately; integrity fault reported; no plaintext leakage. | `6` |
| **TC-WF-07** | Disaster Recovery via M-of-N Shards | Test unsealing / master key reconstruction using threshold recovery shards. | Locked vault with lost master passphrase. | 1. Execute `qv init unseal`.<br>2. Supply 2 shards (below threshold 3).<br>3. Supply 3rd valid shard. | First 2 shards return progress `2/3 required`; 3rd shard unlocks vault and restores operations. | `3` (step 2), `0` (step 3) |

---

## 5. Initial Testing Plan

### 5.1 Testing Objectives
1. **Cryptographic Soundness**: Validate that all hybrid PQC algorithms (ML-KEM / Kyber and ML-DSA / Dilithium) match NIST FIPS 203/204 test vectors.
2. **Interface Stability**: Ensure the CLI adheres strictly to the defined input/output contract across all platforms.
3. **Defense-in-Depth**: Ensure zero memory residue of sensitive passphrases, plaintext secrets, or intermediate key materials upon process termination.
4. **Cross-Platform Consistency**: Equal functionality and security behavior across Linux (x86_64, aarch64), macOS (Apple Silicon), and Windows.

---

### 5.2 Test Levels & Methodologies

```
┌────────────────────────────────────────────────────────┐
│            End-to-End Workflow & CLI Blackbox          │  (Bats / Python Pytest CLI runner)
├────────────────────────────────────────────────────────┤
│           Integration Tests (Storage & OS Keystores)   │  (Mock filesystem, memory leaks, IPC)
├────────────────────────────────────────────────────────┤
│       Unit Tests: Cryptographic Primitives & KATs      │  (NIST FIPS 203/204 Known Answer Tests)
└────────────────────────────────────────────────────────┘
```

#### 5.2.1 Level 1: Cryptographic Unit Tests (FIPS Known Answer Tests)
- **Scope**: Direct testing of liboqs / PQC algorithm wrappers and symmetric primitives.
- **Tools**: Rust `cargo test` / Go `testing` harness.
- **Coverage**:
  - NIST FIPS 203 (ML-KEM-768, ML-KEM-1024) decapsulation against NIST standard KAT files.
  - NIST FIPS 204 (ML-DSA-65) signature generation and verification.
  - HKDF-SHA512 key expansion and Argon2id memory-cost parameter enforcement.
  - Secret zeroization verification using `zeroize` memory assertions.

#### 5.2.2 Level 2: Component Integration Tests
- **Scope**: Vault state machine, session lease manager, and local storage backends.
- **Verification**:
  - File locking to prevent concurrent write corruption.
  - In-memory key caching using protected OS memory APIs (`mlock`, Windows `VirtualLock`).
  - Merkle-tree audit log append and tamper verification.

#### 5.2.3 Level 3: CLI End-to-End (E2E) Workflow Tests
- **Scope**: Invoking the compiled `qv` binary as a subprocess across all commands.
- **Framework**: `bats-core` (Bash Automated Testing System) and `pytest` with `subprocess`.
- **Test Scenarios**: Execution of test cases `TC-WF-01` through `TC-WF-07`.

#### 5.2.4 Level 4: Security & Fuzz Testing
- **Fuzzing Targets**: CLI argument parser, JSON/YAML deserializers, and binary ciphertext packet parser.
- **Tooling**: `cargo-fuzz` (libFuzzer) and `AFL++`.
- **Objective**: Ensure no panics, buffer overflows, or out-of-memory states on malformed or malicious inputs.

---

### 5.3 Test Execution Matrix & Environment

| Layer | Environment | Trigger | Target Threshold |
| :--- | :--- | :--- | :--- |
| **Unit & KATs** | Linux x86_64, macOS aarch64 | Every git push / PR | 100% Pass; >90% code coverage |
| **CLI Workflows** | Ubuntu 22.04/24.04, macOS 14+, Windows Server 2022 | PR review & Nightly | All 7 workflows pass; 0 flakiness |
| **Fuzz Suite** | Linux x86_64 (Clang AddressSanitizer) | Nightly scheduled build | 12-hour fuzz run with 0 crashes |
| **Security Audit** | Static analysis (`cargo audit`, `semgrep`, `bandit`) | Every git push | 0 High/Critical findings |

---

### 5.4 CI/CD Pipeline Implementation Blueprint

```yaml
name: QuantumVault Test Suite

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  crypto-unit-tests:
    name: Crypto KATs & Unit Tests
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - name: Run Cryptographic Unit Tests
        run: cargo test --lib -- --nocapture
      - name: Verify Memory Zeroization
        run: cargo test --features secure-memory-audit

  cli-e2e-workflow:
    name: CLI Integration Workflows
    needs: crypto-unit-tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Binary
        run: cargo build --release --bin qv
      - name: Install Bats
        run: sudo apt-get update && sudo apt-get install -y bats
      - name: Run E2E Test Cases (TC-WF-01 to TC-WF-07)
        run: bats tests/e2e/test_vault_workflows.bats

  security-scan:
    name: Security & Static Analysis
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Check Dependencies
        run: cargo audit
      - name: Clippy / Linter
        run: cargo clippy -- -D warnings
```

---

## 6. Document Sign-Off & Next Steps

- [x] Initial CLI command taxonomy defined.
- [x] End-to-end vault workflow test cases documented (`TC-WF-01` to `TC-WF-07`).
- [x] I/O contract, error schemas, and exit codes defined.
- [x] Multi-tier testing strategy, matrix, and CI/CD pipeline formulated.
- [ ] Next Action: Implement `qv init` and `qv auth` handlers in sprint milestone `feat(cli): core command runner`.
