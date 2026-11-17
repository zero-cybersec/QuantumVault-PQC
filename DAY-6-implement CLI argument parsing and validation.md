# QuantumVault CLI Argument Parsing, Vault Command Arguments & Input Validation Test Report

**Milestone:** CLI Argument Parsing, Vault Command Arguments, and Invalid-Argument Validation  
**Test Result:** PASSED (100% Pass Rate — 150/150 Unit Tests, 49/49 POSIX CLI Scenarios)

---

## 1. Executive Summary

This milestone implements standard CLI argument parsing and strict input validation across all core QuantumVault (`qv`) commands, defines the comprehensive argument and option specifications for vault operations (`secret`, `init`, `key`, `policy`, `audit`, `mount`, `identity`), and adds 48 dedicated invalid-argument test cases.

### Milestone Deliverables:
1. **CLI Argument Parsing Engine**:
   - Comprehensive argument definitions across `secret put/get/list/delete`, `init`, `key rotate/export-public/status`, `policy list/test`, and `audit tail/verify`.
   - Rich option parsing including positional arguments, typed options, multiple flags, and custom Click `ParamType` converters.
2. **Initial Vault Command Arguments Defined**:
   - `secret put PATH [--value TEXT] [--file PATH] [--stdin] [--ttl DURATION] [--pqc-algorithm ALGO] [--tag KEY=VALUE]`
   - `secret get PATH [--version INT] [--field TEXT] [--metadata-only]`
   - `secret delete PATH [--version INT] [--hard] [--force]`
   - `secret list [PREFIX] [--recursive]`
   - `init [--shares INT] [--threshold INT] [--algorithm ALGO] [--storage-backend TYPE] [--non-interactive]`
   - `key rotate [--key-id ID] [--algorithm ALGO] [--re-encrypt-all]`
   - `key export-public [KEY_ID] [--output PATH] [--format FORMAT]`
   - `key status [--key-id ID]`
   - `policy test [POLICY_NAME] --identity IDENTITY --path PATH`
   - `audit tail [--lines INT] [--since DURATION]`
   - `audit verify [--root-hash HASH]`
3. **Input Validation for CLI Commands**:
   - Custom Click ParamTypes: `SecretPathParamType`, `SecretPrefixParamType`, `PolicyNameParamType`, `KeyIDParamType`, `SharesParamType`, `ThresholdParamType`, `PQCAlgorithmParamType`, `DurationParamType`, `PositiveIntParamType`, and `NonNegativeIntParamType`.
   - Pre-execution validation blocking shell command injection, directory traversal (`..`), leading slashes, out-of-range thresholds, and classical/weak cryptographic ciphers.
4. **Invalid-Argument Test Suite**:
   - Created `tests/test_argument_parsing.py` containing 48 focused test cases evaluating error handling, parameter rejection, and exit codes.
5. **Testing Verification**:
   - Total test suite expanded to **150 automated unit tests** across 7 test modules (0 failures, 0 errors, 0.17s runtime).
   - **49 end-to-end POSIX subprocess CLI tests** validating expected exit codes (`0` success, `2` invalid arguments).

---

## 2. Command Argument & Option Specifications

### 2.1 Secret Storage (`qv secret`)

| Command | Arguments | Options | Input Validation Rules |
|---|---|---|---|
| `qv secret put` | `PATH` (required) | `--value, -v`, `--file, -F`, `--stdin`, `--tag`, `--ttl`, `--pqc-algorithm, -a` | `PATH`: relative path, alphanumeric segments, no `..`, no leading `/`. `--ttl`: format `^[1-9][0-9]*[smhdw]$`. `--pqc-algorithm`: allowed PQC taxonomy. |
| `qv secret get` | `PATH` (required) | `--version`, `--field`, `--metadata-only` | `PATH`: validated relative path. `--version`: positive integer >= 1. |
| `qv secret list` | `PREFIX` (optional) | `--recursive, -r` | `PREFIX`: relative prefix, no `..`, no leading `/`. |
| `qv secret delete` | `PATH` (required) | `--version`, `--hard`, `--force, -f` | `PATH`: validated relative path. `--version`: positive integer >= 1. |

### 2.2 Vault Initialization (`qv init`)

| Option | Type | Default | Validation & Constraints |
|---|---|---|---|
| `--shares, -n` | `SharesParamType` | `5` | Integer between `1` and `255`. |
| `--threshold, -m`| `ThresholdParamType` | `3` | Integer between `1` and `shares`. Fails with code 2 if `threshold > shares`. |
| `--algorithm, -a`| `PQCAlgorithmParamType`| `ml-kem-768` | Allowed: `ml-kem-768`, `ml-kem-1024`, `ml-dsa-65`, `hybrid-kyber768-x25519`. |
| `--storage-backend` | `Choice` | `local-encrypted` | Choices: `in-memory`, `local-encrypted`, `raft-cluster`. |
| `--non-interactive` | `Flag` | `False` | Boolean flag. |

### 2.3 Cryptographic Key Operations (`qv key`)

| Command | Arguments | Options | Validation Rules |
|---|---|---|---|
| `qv key rotate` | None | `--key-id`, `--algorithm, -a`, `--re-encrypt-all` | `--key-id`: 2-64 alphanumeric chars, no `..`. `--algorithm`: allowed PQC suite. |
| `qv key export-public`| `KEY_ID` (optional, default: `root-kek-01`) | `--output, -o`, `--format, -f` | `KEY_ID`: valid key identifier. `--format`: `pem`, `der`, `json`. |
| `qv key status` | None | `--key-id` | `--key-id`: valid key identifier. |

### 2.4 Policy & Audit Operations (`qv policy`, `qv audit`)

| Command | Arguments / Options | Validation Rules |
|---|---|---|
| `qv policy test` | `POLICY_NAME` (optional), `--identity, -i` (required), `--path, -p` (required) | `POLICY_NAME`: valid policy identifier. `--identity`: valid principal. `--path`: valid secret path. |
| `qv audit tail` | `--lines, -n` (default: 5), `--since` (optional) | `--lines`: positive integer between 1 and 10000. `--since`: valid duration TTL. |
| `qv audit verify`| `--root-hash, -r` (optional) | `--root-hash`: valid hex string or default root. |

---

## 3. Test Execution Matrix (150 Tests)

```text
========================================================================================
Test Module                  Test Count    Failures    Errors    Runtime    Status
========================================================================================
test_argument_parsing.py     48            0           0         0.045s     PASSED
test_cli_basic.py            6             0           0         0.008s     PASSED
test_error_handling.py       3             0           0         0.004s     PASSED
test_identity_commands.py    20            0           0         0.015s     PASSED
test_mount_commands.py       17            0           0         0.012s     PASSED
test_validation.py           47            0           0         0.018s     PASSED
test_vault_commands.py       9             0           0         0.005s     PASSED
----------------------------------------------------------------------------------------
TOTAL                        150           0           0         0.176s     100% PASS
========================================================================================
```

---

## 4. Invalid-Argument Test Scenarios

| Category | Input Tested | Expected Error Behavior | Exit Code | Status |
|---|---|---|---|---|
| **Secret Put** | Missing `PATH` argument | Click reports missing required argument | 2 | PASS |
| **Secret Put** | `PATH = /app/stripe/token` | Rejects leading slash (`must be relative`) | 2 | PASS |
| **Secret Put** | `PATH = app/../stripe/token` | Rejects directory traversal (`..`) | 2 | PASS |
| **Secret Put** | `PATH = app/token;rm -rf` | Rejects forbidden command injection characters | 2 | PASS |
| **Secret Put** | `--ttl -1h` | Rejects negative lease duration | 2 | PASS |
| **Secret Put** | `--ttl 10x` | Rejects unsupported duration unit | 2 | PASS |
| **Secret Put** | `--pqc-algorithm rsa2048` | Rejects non-PQC classical cipher | 2 | PASS |
| **Secret Get** | Missing `PATH` argument | Click reports missing required argument | 2 | PASS |
| **Secret Get** | `PATH = app/../token` | Rejects directory traversal (`..`) | 2 | PASS |
| **Secret Get** | `--version 0` | Rejects non-positive version | 2 | PASS |
| **Secret Get** | `--version -3` | Rejects negative version | 2 | PASS |
| **Secret Delete** | Missing `PATH` argument | Click reports missing required argument | 2 | PASS |
| **Secret Delete** | `PATH = app/../token` | Rejects directory traversal (`..`) | 2 | PASS |
| **Secret Delete** | `--version -1` | Rejects negative version | 2 | PASS |
| **Secret List** | `PREFIX = /api` | Rejects leading slash | 2 | PASS |
| **Secret List** | `PREFIX = api/../token` | Rejects directory traversal (`..`) | 2 | PASS |
| **Init** | `--shares 0` | Rejects zero shares (must be >= 1) | 2 | PASS |
| **Init** | `--shares -5` | Rejects negative shares | 2 | PASS |
| **Init** | `--shares 300` | Rejects excessive shares (> 255) | 2 | PASS |
| **Init** | `--threshold 0` | Rejects zero threshold (must be >= 1) | 2 | PASS |
| **Init** | `--threshold -1` | Rejects negative threshold | 2 | PASS |
| **Init** | `--shares 3 --threshold 5`| Rejects threshold exceeding total shares | 2 | PASS |
| **Init** | `--algorithm rsa2048` | Rejects unsupported algorithm | 2 | PASS |
| **Init** | `--storage-backend cloud` | Rejects unsupported storage backend | 2 | PASS |
| **Key Rotate** | `--algorithm des` | Rejects unsupported cipher | 2 | PASS |
| **Key Rotate** | `--key-id ../escape` | Rejects directory traversal in key identifier | 2 | PASS |
| **Key Rotate** | `--key-id "key;rm"` | Rejects shell injection in key identifier | 2 | PASS |
| **Key Export** | `--format xml` | Rejects invalid format choice | 2 | PASS |
| **Key Export** | `KEY_ID = ../bad_id` | Rejects directory traversal in key identifier | 2 | PASS |
| **Key Status** | `--key-id ../bad_id` | Rejects directory traversal in key identifier | 2 | PASS |
| **Policy Test** | Missing `--identity` | Rejects missing required option | 2 | PASS |
| **Policy Test** | Missing `--path` | Rejects missing required option | 2 | PASS |
| **Policy Test** | `POLICY = bad;policy` | Rejects invalid policy name | 2 | PASS |
| **Policy Test** | `--identity "bad;user"` | Rejects forbidden characters in identity | 2 | PASS |
| **Policy Test** | `--path /reports/q3` | Rejects leading slash in secret path | 2 | PASS |
| **Audit Tail** | `--lines 0` | Rejects zero lines (must be >= 1) | 2 | PASS |
| **Audit Tail** | `--lines -5` | Rejects negative lines | 2 | PASS |
| **Audit Tail** | `--lines 50000` | Rejects lines exceeding 10000 | 2 | PASS |
| **Audit Tail** | `--since -1h` | Rejects negative since duration | 2 | PASS |

---

## 5. Subprocess POSIX Execution Verification

```text
Ran 150 tests in 0.176s

OK

Subprocess POSIX CLI Test Suite:
All 49/49 POSIX CLI scenarios passed successfully!
```
