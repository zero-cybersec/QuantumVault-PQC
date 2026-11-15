# QuantumVault CLI Project Structure, Mount/Unmount & Identity Interfaces, and Validation Test Report

---

## 1. Executive Summary

This milestone establishes the modular CLI project structure for QuantumVault (`qv`), defines the command interfaces for secret engine/virtual filesystem mounts (`qv mount`, `qv unmount`) and identity lifecycle management (`qv identity`), and implements comprehensive test cases for CLI input validation.

### Key Deliverables:
1. **Modular CLI Project Architecture**:
   - Organized into clean modules under `quantumvault/` (`cli.py`, `constants.py`, `output.py`, `validation.py`, `version.py`) and modular command implementations under `quantumvault/commands/` (`mount_cmd.py`, `identity_cmd.py`, `auth_cmd.py`, `secret_cmd.py`, etc.).
   - Standardized `tests/` test directory with dedicated test runners.
2. **Mount and Unmount Command Interfaces**:
   - `qv mount` command group supporting `list`, `enable` (aliased as `create`), `tune`, and `disable` (aliased as `remove`).
   - Symmetrical top-level `qv unmount` command for unmounting secret engines and virtual filesystems.
   - Guardrails preventing unauthorized unmounting or tuning of protected system mounts (`sys/`) without `--force`.
3. **Comprehensive Identity Command Interfaces**:
   - `qv identity list` with filtering by type, status, role, substring search, and limit/offset pagination.
   - `qv identity get` for querying identity records and post-quantum credentials by name or ID.
   - `qv identity create`, `qv identity update` (status, role assignments), `qv identity revoke` (immediate session and lease revocation), and `qv identity delete`.
   - Cross-linking from authentication commands via `qv auth identities`.
4. **Defense-in-Depth CLI Input Validation**:
   - Parameter validators for mount paths, mount engine types, post-quantum cryptographic algorithms, identity identifiers, durations/TTLs, and secret paths.
   - Protection against shell command injection, directory traversal attacks, whitespace pollution, and boundary overruns.
5. **Rigorous Test Suite Execution**:
   - Test suite expanded to **102 unit and integration tests** across 6 test modules with 0 failures and 0 errors.
   - **30 end-to-end POSIX subprocess CLI tests** validating expected exit codes (`0` success, `2` invalid arguments, `4` not found).

---

## 2. CLI Project Structure

```text
quantumvault/
├── pyproject.toml                     # Modern build and project configuration
├── setup.py                           # Setuptools package descriptor
├── README.md                          # Quick start, command specifications, and security policies
├── quantumvault/                      # QuantumVault package root
│   ├── __init__.py                    # Public package exports
│   ├── cli.py                         # Click CLI group and global option handling
│   ├── constants.py                   # POSIX exit codes, cipher defaults, and system constants
│   ├── output.py                      # Multi-format output formatter (Text table, JSON, YAML)
│   ├── validation.py                  # Input validation engine & Click custom ParamTypes
│   ├── version.py                     # Version metadata, PQC algorithm identifiers
│   └── commands/                      # Command implementation subtree
│       ├── __init__.py
│       ├── audit_cmd.py               # Audit trail inspection
│       ├── auth_cmd.py                # Session authentication and auth shortcuts
│       ├── identity_cmd.py            # Identity and access management
│       ├── init_cmd.py                # Shamir secret sharing initialization
│       ├── key_cmd.py                 # Post-quantum cryptographic key management
│       ├── mount_cmd.py               # Secret engine and VFS mount/unmount operations
│       ├── policy_cmd.py              # Zero-trust access policies
│       ├── secret_cmd.py              # Secret storage and retrieval
│       ├── status_cmd.py              # System health and lock state
│       └── version_cmd.py             # Version inspection command
└── tests/                             # Automated test suite (102 tests)
    ├── test_cli_basic.py              # CLI help and version tests (6 tests)
    ├── test_error_handling.py         # Subcommand error and missing argument tests (3 tests)
    ├── test_identity_commands.py      # Identity list, get, create, update, revoke, delete (20 tests)
    ├── test_mount_commands.py         # Mount list, enable, tune, disable, unmount (17 tests)
    ├── test_validation.py             # Validation unit and CLI injection rejection tests (47 tests)
    └── test_vault_commands.py         # Vault init, secret, status, policy, key tests (9 tests)
```

---

## 3. Command Interfaces Specification

### 3.1 Mount and Unmount Commands

#### `qv mount` / `qv mount list`
List active secret engines and virtual filesystem mounts.
- Flags: `--type, -t` (filter by engine), `--search, -q` (search substring), `--limit, -n`, `--offset`, `--format, -f`.

#### `qv mount enable PATH` / `qv mount create PATH`
Mount an encrypted secret engine or virtual filesystem at `PATH`.
- Options:
  - `--type, -t`: Supported types: `kv`, `kv-v2`, `pki-pqc`, `transit`, `fuse-vfs`, `totp`, `database`. Default: `kv-v2`.
  - `--description, -d`: Human-readable description.
  - `--pqc-algorithm, -a`: Post-quantum algorithm (`ml-kem-768`, `ml-kem-1024`, `ml-dsa-65`, `hybrid-kyber768-x25519`). Default: `ml-kem-768`.
  - `--default-lease-ttl`: Default lease duration (e.g. `1h`).
  - `--max-lease-ttl`: Maximum lease duration (e.g. `24h`).
  - `--read-only, --ro`: Mount as read-only volume.

#### `qv mount tune PATH`
Tune configuration parameters for an active mount.
- Options: `--default-lease-ttl`, `--max-lease-ttl`, `--description`.
- Protected system mounts (`sys/`) are guarded against modification.

#### `qv mount disable PATH` / `qv mount remove PATH` / `qv unmount PATH`
Unmount and disable an active secret engine or virtual filesystem.
- Options: `--force, -f` (required to unmount protected system mount points).

### 3.2 Identity Commands (`qv identity`)

#### `qv identity list`
List registered identities and workloads.
- Filters: `--type` (`user`, `service`, `workload`, `machine`, `all`), `--status` (`active`, `suspended`, `revoked`, `pending`, `all`), `--role`, `--search`, `--limit`, `--offset`.

#### `qv identity get IDENTITY`
Inspect identity profile and cryptographic credentials by ID or name.

#### `qv identity create NAME`
Register a new identity principal.
- Options: `--type`, `--role` (repeatable), `--auth-method` (e.g., `ml-dsa-65`, `spiffe-jwt`).

#### `qv identity update IDENTITY`
Update an existing identity's lifecycle status or role assignments.
- Options: `--status`, `--add-role`.

#### `qv identity revoke IDENTITY`
Immediately revoke identity credentials, invalidate authentication tokens, and terminate active secret leases.

#### `qv identity delete IDENTITY`
Remove an identity from the vault catalog.

---

## 4. Input Validation & Security Test Matrix

### 4.1 Unit & Integration Test Summary (102 Tests)

| Test Module | Focus Area | Test Count | Failures | Status |
|---|---|---|---|---|
| `test_cli_basic.py` | Help flags, formatted version (text/JSON), subcommands | 6 | 0 | PASSED |
| `test_error_handling.py` | Unknown commands, unknown options, missing arguments | 3 | 0 | PASSED |
| `test_identity_commands.py` | Identity list, filtering, search, pagination, get, update, revoke, delete, auth alias | 20 | 0 | PASSED |
| `test_mount_commands.py` | Mount list, enable, tune, disable, unmount, system mount protections | 17 | 0 | PASSED |
| `test_validation.py` | Unit validation functions and CLI injection/traversal rejection integration tests | 47 | 0 | PASSED |
| `test_vault_commands.py` | Vault initialization (Shamir), secrets, keys, policies, audit, health status | 9 | 0 | PASSED |
| **Total** | | **102** | **0** | **100% PASS** |

### 4.2 CLI Input Validation Scenarios Tested

| Category | Input Tested | Expected Result | Exit Code | Verification |
|---|---|---|---|---|
| **Mount Path** | `secret/kv` | Accepted (normalized to `secret/kv/`) | 0 | PASS |
| **Mount Path** | `/secret/kv` | Rejected (absolute path / leading slash) | 2 | PASS |
| **Mount Path** | `secret/../escape` | Rejected (directory traversal `..`) | 2 | PASS |
| **Mount Path** | `secret;rm -rf` | Rejected (command injection characters `;`) | 2 | PASS |
| **Mount Path** | `""` or `" "` | Rejected (empty / whitespace) | 2 | PASS |
| **Mount Type** | `kv-v2`, `transit` | Accepted (supported taxonomy) | 0 | PASS |
| **Mount Type** | `invalid-engine` | Rejected (unknown engine) | 2 | PASS |
| **PQC Algorithm**| `ml-kem-768`, `ml-dsa-65` | Accepted (approved PQC algorithms) | 0 | PASS |
| **PQC Algorithm**| `rsa2048` | Rejected (non-PQC cipher) | 2 | PASS |
| **Unmount System**| `sys/` without `--force` | Rejected (protected system mount) | 2 | PASS |
| **Unmount System**| `sys/` with `--force` | Accepted | 0 | PASS |
| **Unmount Missing**| `qv unmount` | Rejected (missing required argument) | 2 | PASS |
| **Unmount Nonexistent**| `qv unmount ghost/path` | Rejected (mount not found) | 4 | PASS |
| **Identity Name**| `dev-nikhil` | Accepted | 0 | PASS |
| **Identity Name**| `bad;user` | Rejected (forbidden characters) | 2 | PASS |
| **Identity Name**| `.hidden` | Rejected (illegal start character) | 2 | PASS |
| **Identity Name**| `../escape` | Rejected (directory traversal) | 2 | PASS |
| **Identity Name**| `"x"` | Rejected (too short, min 2) | 2 | PASS |
| **Identity Status**| `active`, `suspended` | Accepted | 0 | PASS |
| **Identity Status**| `bogus_status` | Rejected | 2 | PASS |
| **Lease Duration**| `30s`, `15m`, `4h`, `7d` | Accepted | 0 | PASS |
| **Lease Duration**| `-1h`, `infinite` | Rejected | 2 | PASS |
| **Pagination** | `--limit 0`, `--limit -5` | Rejected (must be > 0) | 2 | PASS |
| **Pagination** | `--limit 5000` | Rejected (exceeds max 1000) | 2 | PASS |
| **Pagination** | `--offset -2` | Rejected (must be >= 0) | 2 | PASS |

---

## 5. Verification Log

```text
Ran 102 tests in 0.054s

OK

Subprocess POSIX CLI Test Suite:
All 30/30 POSIX CLI scenarios passed successfully!
```
