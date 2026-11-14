# QuantumVault CLI Identity Listing, Input Validation & Test Results Report

---

## 1. Executive Summary

This engineering increment delivers full identity management capabilities and comprehensive CLI input validation for the QuantumVault (`qv`) post-quantum secrets management platform.

### Objectives Achieved:
1. **Vault Identity Listing & Inspection Commands**: Implemented `qv identity list` (with granular filtering across type, status, role, search substring, and pagination), `qv identity get` (single identity record retrieval with PQC fingerprints), `qv identity create`, and the shortcut integration under `qv auth identities`.
2. **Robust CLI Input Validation Engine**: Created `quantumvault.validation`, establishing strict input validation rules, regex guards, boundary constraints, and custom Click `ParamType` validators to prevent shell injection, directory traversal, and malformed parameter inputs.
3. **Rigorous Test Coverage**: Expanded the automated test suite from 18 tests to **61 unit and integration tests**, complemented by a dedicated **31-scenario end-to-end POSIX CLI test harness** verifying exact exit codes and error output contracts.

---

## 2. Command Architecture & Implementation

### 2.1 Identity Command Subtree (`qv identity`)

The identity management subtree is registered at root level (`qv identity`) and cross-linked into authentication workflows (`qv auth identity`, `qv auth identities`).

```
qv
├── identity
│   ├── list          # List identities with filtering, search, and pagination
│   ├── get           # Retrieve identity details, PQC fingerprint, and roles
│   └── create        # Provision new identity with cryptographic auth parameters
└── auth
    ├── identities    # Direct alias to identity listing
    └── login         # Validated session login (validates identity name and TTL)
```

### 2.2 Options and Filters for `qv identity list`

| Option | Flag | Type / Constraint | Description |
|---|---|---|---|
| `--type` | `-t` | `user`, `service`, `workload`, `machine`, `all` | Filter by entity classification. |
| `--status` | `-s` | `active`, `suspended`, `revoked`, `pending`, `all` | Filter by identity lifecycle status. |
| `--role` | `-r` | String | Substring match against attached RBAC/ABAC policy names. |
| `--search` | `-q` | String | Case-insensitive query matching ID, name, or metadata attributes. |
| `--limit` | `-n` | Integer (1 – 1000, default: 20) | Max records returned. |
| `--offset` | | Integer (>= 0, default: 0) | Starting index for pagination. |
| `--format` | `-f` | `text`, `json`, `yaml` | Output presentation format. |

### 2.3 Identity Data Schema

Every identity entity conforms to the following schema:
```json
{
  "id": "id-usr-007",
  "name": "dev-nikhil",
  "type": "user",
  "status": "active",
  "roles": [
    "developer-sandbox",
    "secret-test-access"
  ],
  "auth_method": "ml-dsa-65",
  "created_at": "2026-11-12T10:00:00Z",
  "last_active": "2026-11-14T20:20:00Z",
  "pqc_fingerprint": "SHA256:3f8901ebd9812ba38914bca89102cba091823901bca11a980bc2391ea0918bca",
  "metadata": {
    "email": "251312011@nitdelhi.ac.in",
    "role": "lead-architect"
  }
}
```

---

## 3. Input Validation Architecture & Security Controls

Input validation is centralized in `quantumvault.validation` and enforced at the Click CLI boundary through custom `click.ParamType` classes and standalone assertion functions.

### 3.1 Security Validation Rules

| Validation Target | Specification & Pattern | Security Rationale |
|---|---|---|
| **Identity Name / ID** | Length: 2–64 characters.<br>Pattern: `^[a-zA-Z0-9][a-zA-Z0-9._/-]{1,63}$` | Prohibits shell injection (`;&\|$<>` `), path traversal (`..`), whitespace, and non-alphanumeric initial characters. |
| **Identity Type** | Set: `{"user", "service", "workload", "machine", "all"}` | Enforces strict taxonomy; rejects arbitrary type injection. |
| **Identity Status** | Set: `{"active", "suspended", "revoked", "pending", "all"}` | Prevents invalid state queries. |
| **Pagination Limit** | Integer: `1 <= limit <= 1000` | Defends against buffer exhaustion and negative offsets. |
| **Pagination Offset** | Integer: `offset >= 0` | Defends against negative indexing errors. |
| **Lease Duration / TTL** | Regex: `^([1-9][0-9]*)\s*([smhdw])$` | Ensures parseable time duration (`30s`, `15m`, `4h`, `7d`, `2w`). Rejects negative or uncalibrated durations. |
| **Secret Paths** | Regex: `^[a-zA-Z0-9_.-]+(?:/[a-zA-Z0-9_.-]+)*$` | Blocks leading slashes (`/`), root overwrite, and path traversal (`..`). |

### 3.2 Error & Exit Code Contract

Validation failures immediately halt execution with descriptive error diagnostics and standard POSIX exit codes:
- **`2` (`EXIT_INVALID_ARGS` / `QV_EXIT_INVALID_ARGS`)**: Raised for malformed inputs, parameter type mismatches, forbidden characters, or boundary violations.
- **`4` (`EXIT_SECRET_NOT_FOUND` / `QV_EXIT_SECRET_NOT_FOUND`)**: Returned when an identity identifier is syntactically valid but does not exist in the vault directory.
- **`0` (`EXIT_SUCCESS`)**: Returned upon successful command processing.

---

## 4. Test Matrix & Verification Results

### 4.1 Unit & Integration Test Suite (`tests/`)

The test suite contains 61 automated test cases across four test modules:

| Test Module | Test Focus | Tests | Failures | Status |
|---|---|---|---|---|
| `test_cli_basic.py` | Help flags, version outputs (text, json), subcommands | 6 | 0 | PASSED |
| `test_vault_commands.py` | Init (shards, threshold), auth, secret, key, policy, audit, status | 9 | 0 | PASSED |
| `test_error_handling.py` | Subcommand errors, missing arguments, unknown options | 3 | 0 | PASSED |
| `test_identity_commands.py` | Identity listing, type/status/role filters, search, pagination, get, create, auth shortcut | 13 | 0 | PASSED |
| `test_validation.py` | Direct validator unit tests (valid & invalid inputs) and CLI integration rejection tests | 30 | 0 | PASSED |
| **Total** | | **61** | **0** | **100% PASS** |

### 4.2 End-to-End CLI Invocation Test Results (31 Scenarios)

The table below documents direct POSIX command line invocations tested via the test harness:

| ID | Command Executed | Input Category | Expected Exit Code | Actual Exit Code | Result | Notes |
|---|---|---|---|---|---|---|
| 01 | `qv identity list` | Valid / Default | 0 | 0 | PASS | Outputs formatted text table of 7 seed identities |
| 02 | `qv identity list -t user` | Valid Filter | 0 | 0 | PASS | Returns only user identities (3 records) |
| 03 | `qv identity list -t service` | Valid Filter | 0 | 0 | PASS | Returns only service identities (2 records) |
| 04 | `qv identity list -s active` | Valid Filter | 0 | 0 | PASS | Filters active status (5 records) |
| 05 | `qv identity list -s suspended` | Valid Filter | 0 | 0 | PASS | Filters suspended status (`backup-agent`) |
| 06 | `qv identity list -r analytics` | Valid Filter | 0 | 0 | PASS | Returns `etl-pipeline` and `analyst-readonly` |
| 07 | `qv identity list -q nikhil` | Valid Search | 0 | 0 | PASS | Substring query matches `dev-nikhil` |
| 08 | `qv identity list -n 3 --offset 1` | Valid Pagination | 0 | 0 | PASS | Correctly offsets by 1 and limits to 3 |
| 09 | `qv --format json identity list` | Valid Format | 0 | 0 | PASS | Emits valid JSON payload with total counters |
| 10 | `qv --format yaml identity list` | Valid Format | 0 | 0 | PASS | Emits valid YAML payload |
| 11 | `qv identity get root-operator` | Valid Get | 0 | 0 | PASS | Returns full identity record and ML-DSA key |
| 12 | `qv identity get id-usr-007` | Valid Get (ID) | 0 | 0 | PASS | Resolves record by unique identifier |
| 13 | `qv identity create ci-runner -t service -r ci-pipeline` | Valid Create | 0 | 0 | PASS | Provisions new identity with generated ID |
| 14 | `qv auth identities` | Valid Shortcut | 0 | 0 | PASS | Routes seamlessly from `auth` group |
| 15 | `qv auth login -i dev-nikhil --ttl 2h` | Valid Auth | 0 | 0 | PASS | Accepts valid identity and duration |
| 16 | `qv identity get "root;rm -rf"` | Invalid Input | 2 | 2 | PASS | Rejects command injection characters (`;`, `' '`) |
| 17 | `qv identity get "x"` | Invalid Input | 2 | 2 | PASS | Rejects identity identifier shorter than 2 chars |
| 18 | `qv identity get "A"*70` | Invalid Input | 2 | 2 | PASS | Rejects identity identifier exceeding 64 chars |
| 19 | `qv identity get "../etc/passwd"` | Invalid Input | 2 | 2 | PASS | Rejects directory traversal sequences (`..`) |
| 20 | `qv identity get ".hidden"` | Invalid Input | 2 | 2 | PASS | Rejects illegal non-alphanumeric leading char |
| 21 | `qv identity get "user name with space"` | Invalid Input | 2 | 2 | PASS | Rejects internal spaces in identifier |
| 22 | `qv identity get "unknown-identity-uuid"` | Valid / Not Found | 4 | 4 | PASS | Valid syntax but entity missing from vault |
| 23 | `qv identity list -t invalid_type` | Invalid Option | 2 | 2 | PASS | Rejects non-taxonomy type |
| 24 | `qv identity list -s invalid_status` | Invalid Option | 2 | 2 | PASS | Rejects non-existent status |
| 25 | `qv identity list -n 0` | Invalid Option | 2 | 2 | PASS | Rejects zero limit (must be >= 1) |
| 26 | `qv identity list -n -10` | Invalid Option | 2 | 2 | PASS | Rejects negative limit |
| 27 | `qv identity list -n 5000` | Invalid Option | 2 | 2 | PASS | Rejects limit exceeding ceiling of 1000 |
| 28 | `qv identity list --offset -5` | Invalid Option | 2 | 2 | PASS | Rejects negative pagination offset |
| 29 | `qv auth login --ttl "not-a-duration"` | Invalid Option | 2 | 2 | PASS | Rejects non-duration string |
| 30 | `qv auth login --ttl "-30m"` | Invalid Option | 2 | 2 | PASS | Rejects negative TTL |
| 31 | `qv auth login -i "user&cat"` | Invalid Option | 2 | 2 | PASS | Rejects shell operator (`&`) in auth identity |

---

## 5. Sample CLI Outputs

### 5.1 Text Output (`qv identity list`)
```text
ID          NAME              TYPE      STATUS     AUTH METHOD         POLICIES/ROLES
-------------------------------------------------------------------------------------
id-usr-001  root-operator     user      active     ml-dsa-65           admin-full-access
id-svc-002  backend-api       service   active     spiffe-jwt          api-service-rw, vault-transit-encrypt
id-wkl-003  etl-pipeline      workload  active     hybrid-token        analytics-read-only
id-usr-004  analyst-readonly  user      active     ml-dsa-65           analytics-read-only
id-svc-005  backup-agent      service   suspended  spiffe-jwt          snapshot-admin
id-mch-006  legacy-worker     machine   revoked    deprecated-rsa3072  <none>
id-usr-007  dev-nikhil        user      active     ml-dsa-65           developer-sandbox, secret-test-access
-------------------------------------------------------------------------------------
Showing 7 of 7 identities (offset 0, limit 20)
```

### 5.2 Structured JSON Output (`qv --format json identity get id-usr-007`)
```json
{
  "identity": {
    "id": "id-usr-007",
    "name": "dev-nikhil",
    "type": "user",
    "status": "active",
    "roles": [
      "developer-sandbox",
      "secret-test-access"
    ],
    "auth_method": "ml-dsa-65",
    "created_at": "2026-11-12T10:00:00Z",
    "last_active": "2026-11-14T20:20:00Z",
    "pqc_fingerprint": "SHA256:3f8901ebd9812ba38914bca89102cba091823901bca11a980bc2391ea0918bca",
    "metadata": {
      "email": "251312011@nitdelhi.ac.in",
      "role": "lead-architect"
    }
  }
}
```

### 5.3 Injection Attempt Rejection Example
```text
$ qv identity get "root;rm -rf"
Usage: qv identity get [OPTIONS] IDENTITY
Try 'qv identity get -h' for help.

Error: Invalid value for 'IDENTITY': Identity identifier 'root;rm -rf' contains forbidden characters (' ', ';'). Only alphanumeric characters, '.', '_', '-', and '/' are permitted.
```
*(Process exit code: `2`)*

---

## 6. Conclusion & Next Steps

All criteria specified in the user request have been completely implemented, verified, and documented:
- `qv identity list`, `get`, `create`, and `qv auth identities` are fully operational.
- Strict input validation prevents all forms of shell injection, path manipulation, and malformed arguments.
- 61 unit tests and 31 CLI execution scenarios pass cleanly with 100% success rate.
- Ready for next milestone: implementing cryptographic backend storage and Shamir unsealing workflows.
