# ZeroClaw Security Analysis (2026-02-21)

> **Status: Snapshot — Immutable once superseded**
>
> As-of date: **February 21, 2026**.
> This document is a point-in-time security analysis of the ZeroClaw codebase.
> For current runtime security behavior, see [config-reference.md](config-reference.md),
> [operations-runbook.md](operations-runbook.md), and [security/README.md](security/README.md).

---

## Executive Summary

ZeroClaw implements **defense-in-depth security** with multiple layered controls across
authentication, command sandboxing, secret management, network access, and supply chain.
The codebase demonstrates strong security practices with authenticated encryption, quote-aware
command validation, workspace isolation, rate limiting, and comprehensive secret handling.

**Overall Risk Level: LOW to MEDIUM** — well-designed security baseline with active defense
mechanisms. Some areas require configuration attention for production hardening.

---

## 1. Authentication and Authorization

### Gateway Pairing (`src/security/pairing.rs`)

| Control | Detail |
|---------|--------|
| One-time pairing code | Generated on startup, printed to terminal, single-use |
| Transport header | `X-Pairing-Code` header on first connect |
| Token storage | SHA-256 hashes only — no plaintext in config or git |
| Brute-force protection | Max 5 failed attempts, 5-minute lockout per client |
| Per-client tracking | Exponential state management per remote IP |
| Backward compatibility | Plaintext tokens auto-hashed on load |

**Strengths:** Token hashing prevents plaintext exposure; rate limiting prevents credential
stuffing; one-time code consumed after first use.

**Note:** Pairing code printed to stdout requires a secure terminal context. Deployments
using log aggregation should ensure pairing startup output is not captured by log sinks.

### OAuth and API Key Auth (`src/auth/`)

- OpenAI Codex OAuth uses PKCE flow with state/code\_verifier
- Tokens persisted via `SecretStore` (encrypted, see Section 2)
- JWT account ID extraction for profile binding
- Multiple auth profiles per provider supported

---

## 2. Secret and Credential Handling

### SecretStore (`src/security/secrets.rs`)

**Encryption scheme:**

| Property | Value |
|----------|-------|
| Algorithm | ChaCha20-Poly1305 AEAD |
| Key size | 256-bit via `OsRng` |
| Nonce | Fresh random 12-byte nonce per encryption |
| Format | `enc2:<hex(nonce \|\| ciphertext \|\| tag)>` |
| Key file | `~/.zeroclaw/.secret_key`, permissions `0600` (Unix) / `icacls` (Windows) |

**Migration path:** Legacy `enc:` format (XOR cipher) is auto-detected and migrated to
`enc2:` on next access. The `decrypt_and_migrate()` helper upgrades stored values in-place.
The XOR cipher is deprecated but retained for backward compatibility with a documented
migration path.

**Environment variable isolation (`src/tools/shell.rs`):**

- `cmd.env_clear()` called before subprocess launch
- Only allowlisted safe vars restored: `PATH`, `HOME`, `TERM`, `LANG`, `LC_ALL`,
  `LC_CTYPE`, `USER`, `SHELL`, `TMPDIR`
- Explicitly excludes `API_KEY`, `ZEROCLAW_API_KEY`, and all credential-like vars
- Test `shell_does_not_leak_api_key()` validates this boundary

**HTTP header redaction:** Authorization, API key, and token headers are scrubbed from
all log output by the HTTP request tool.

---

## 3. Command Execution and Shell Injection Prevention

### SecurityPolicy (`src/security/policy.rs`)

**Risk classification (per-segment):**

| Tier | Commands | Default behavior |
|------|----------|-----------------|
| High-risk | `rm`, `dd`, `mkfs`, `sudo`, `su`, `passwd`, `mount`, `curl`, `wget`, `ssh`, `scp`, `ftp`, `nc` | Blocked unless `block_high_risk_commands = false` |
| Medium-risk | `touch`, `mkdir`, `pip`, `npm install`, `git clone` | Require `approved=true` in Supervised mode |
| Low-risk | All others | Allowed in workspace scope |

**Shell injection defenses:**

- **Quote-aware parser**: splits commands on unquoted `;`, `\|`, `&&`, `\|\|` only
- Handles single/double quotes with escape sequences
- Prevents bypass via `ls && rm -rf /` style chains
- **Workspace-only mode** (default): blocks absolute paths
- **Forbidden path list** (always enforced, regardless of `workspace_only`):
  - System: `/etc`, `/root`, `/home`, `/usr`, `/bin`, `/var`, `/tmp`
  - Sensitive dotfiles: `~/.ssh`, `~/.gnupg`, `~/.aws`, `~/.config`
- **Path traversal**: `..` sequences rejected at validation boundary

**Resource limits:**

- Shell timeout: 60 seconds (process killed on overflow)
- Output truncation: 1 MB max for stdout/stderr combined
- Rate limiting: sliding 1-hour window (default: 20 actions/hour)
- Cost cap: configurable daily limit in cents (default: $5.00)
- Thread-safe action tracker via `parking_lot::Mutex`

**Test coverage verified:**
- `shell_blocks_disallowed_command()`
- `shell_blocks_readonly()`
- `validate_command_blocks_high_risk_by_default()`
- `validate_command_rejects_background_chain_bypass()`

---

## 4. Network and HTTP Security

### Gateway (`src/gateway/mod.rs`)

| Control | Value |
|---------|-------|
| Body size limit | 64 KB |
| Request timeout | 30 seconds |
| Rate limiting | Sliding window per client IP; dual-layer (pairing + webhook endpoints) |
| Stale entry cleanup | Every 5 minutes |

**Webhook security:**
- HMAC-SHA256 signature verification (WhatsApp, Linq)
- Secrets stored hashed to prevent plaintext config exposure
- Idempotency key tracking to prevent duplicate processing

### HTTP Request Tool (`src/tools/http_request.rs`)

| Control | Detail |
|---------|--------|
| Domain allowlisting | Explicit `allowed_domains` config required |
| Protocol restriction | `http`/`https` only |
| Private IP blocking | Blocks `127.0.0.1`, `192.168.*`, `10.*`, `localhost`, etc. (SSRF defense) |
| URL validation | No whitespace; method allowlist enforced |
| Response size limit | Configurable (OOM prevention) |
| Header redaction | Authorization/API-Key/Token scrubbed from logs |

---

## 5. Database and Memory Storage

- **SQLite backend** (`src/memory/sqlite.rs`): uses `rusqlite` prepared statements throughout —
  no raw SQL string concatenation found.
- **Postgres backend**: uses parameterized queries; credentials via `SecretStore`.
- **Code search result**: no SQL injection patterns detected across the database access layer.

---

## 6. Cryptographic Implementation

**Dependency versions (security-relevant):**

| Crate | Version | Role |
|-------|---------|------|
| `chacha20poly1305` | 0.10 | AEAD encryption |
| `sha2` | 0.10 | SHA-256 hashing |
| `hmac` | 0.12 | MAC authentication |
| `ring` | 0.17 | Additional crypto ops |
| `rustls` | 0.23 | TLS stack (no OpenSSL, TLS 1.2+) |
| `tokio` | 1.42 | Async runtime |
| `reqwest` | 0.12 | HTTP client (uses rustls-tls) |

**No weak crypto found:** No MD5, SHA1, or DES usage detected. Key generation uses `OsRng`
(OS entropy), not UUID or weak PRNG. Rustls enforces TLS 1.2+ with no downgrade path.

---

## 7. Unsafe Code Surface

Only 5 `unsafe` blocks found across the entire codebase:

| File | Count | Context |
|------|-------|---------|
| `src/agent/prompt.rs` | 2 | Performance optimization |
| `src/security/integrate.rs` | 1 | Integration boundary |
| `src/tools/screenshot.rs` | 2 | Platform-specific I/O |

All unsafe usage is isolated and minimal. No unsafe blocks in security-critical paths
(policy enforcement, secret handling, command validation).

---

## 8. CI/CD and Supply Chain

| Workflow | Trigger | Purpose |
|----------|---------|---------|
| `sec-codeql.yml` | Weekly + PR | Static analysis (CodeQL) |
| `sec-audit.yml` | Push to main + PR + weekly | `cargo-audit` + `cargo-deny` |

**`deny.toml` policy:**
- Unmaintained crates: flagged (all transitive)
- Yanked versions: DENY
- License allowlist: MIT, Apache-2.0, BSD variants, ISC
- Unknown registries: DENY
- One approved exception: RUSTSEC-2025-0141 (bincode v2.0.1 via probe-rs, marked complete)

---

## 9. Container Security

- Base image: `gcr.io/distroless/cc-debian12:nonroot` (minimal, no shell)
- Runtime user: UID 65534 (nonroot)
- Supports `--read-only` filesystem mode
- No setuid binaries

---

## 10. Autonomy Level Access Control

| Level | Shell | File Write | Net | Notes |
|-------|-------|------------|-----|-------|
| `ReadOnly` | No | No | Read-only | Observe only |
| `Supervised` | Allowlisted | Workspace-only | Allowlisted | Default; medium-risk requires approval |
| `Full` | Workspace-scoped | Workspace-only | Allowlisted | Full autonomy within sandbox |

---

## 11. Identified Gaps and Recommendations

The following items are not blocking issues but represent recommended hardening improvements.
Each is labeled by priority.

### 11.1 Webhook Replay Protection (MEDIUM)

**Gap:** Webhook handler verifies HMAC-SHA256 signatures but does not validate request
timestamps. A captured request could be replayed within a short window.

**Status by channel:**

| Channel | Replay protection |
|---------|-------------------|
| Linq | **Implemented** — `X-Webhook-Timestamp` validated within 300 s (`src/channels/linq.rs`) |
| Generic `/webhook` | Idempotency key (300 s TTL) mitigates replay for well-behaved senders |
| WhatsApp | **Platform limitation** — Meta's `X-Hub-Signature-256` API does not include a timestamp header; no server-side fix possible |
| Nextcloud Talk | `X-Nextcloud-Talk-Random` nonce included in signature; server-side seen-nonce store would be needed for full replay prevention |

**Residual risk:** WhatsApp and Nextcloud Talk channels remain without timestamp-based replay
protection. Mitigated by HMAC signature verification and rate limiting.

### 11.2 Bearer Token Expiration (MEDIUM) — **Addressed**

**Gap (original):** Gateway bearer tokens (from pairing) were stored as SHA-256 hashes
with no expiration field. A compromised token would remain valid indefinitely.

**Implemented fix** (`src/security/pairing.rs`, `src/config/schema.rs`, `src/gateway/mod.rs`):

- Added `paired_token_max_age_days: u64` to `GatewayConfig` (default `0` = unlimited,
  backward-compatible).
- Added `paired_token_created_at: BTreeMap<String, i64>` to `GatewayConfig` to persist
  Unix-second creation timestamps alongside token hashes.
- `PairingGuard::with_expiry()` constructor filters expired tokens on startup and rejects
  them in `is_authenticated()` at runtime.
- Tokens without a recorded creation time are never expired (backward compatibility for
  configs that pre-date this change).
- `persist_pairing_tokens()` now saves both hashes and creation timestamps so expiry
  survives restarts.

**Usage:** Set `paired_token_max_age_days = 90` in `[gateway]` to expire tokens after
90 days. Value `0` preserves existing behavior (no expiry).

**Blast radius:** Config schema addition only. Old configs load cleanly; no migration
required.

### 11.3 Legacy XOR Cipher Removal Schedule (LOW)

**Gap:** `enc:` format (XOR cipher) remains supported for backward compatibility. While
auto-migration exists, the legacy path is technically CWE-327.

**Recommendation:** Set a deprecation timeline (e.g., remove in v0.3.0). Add a startup
warning if any `enc:` secrets are detected. The migration path (`decrypt_and_migrate()`)
is already implemented.

**Blast radius:** Low. Migration tooling exists; removing the legacy read path is a
small, safe change after a deprecation window.

### 11.4 Complex Shell Metacharacter Coverage (LOW)

**Gap:** The quote-aware shell parser handles common injection vectors (`&&`, `|`, `;`).
Advanced shell constructs (brace expansion `{a,b}`, here-docs `<<EOF`, process
substitution `<(cmd)`) are not explicitly handled.

**Recommendation:** Add test cases for these constructs against the validator. Where
possible, add explicit rejection of `<(`, `{` sequences in high-risk contexts.

**Blast radius:** Tests only; no behavior change until explicit rules added.

### 11.5 Pairing Code Log Sink Exposure (LOW)

**Gap:** Pairing code is printed to stdout on startup. In environments with log
aggregation (Splunk, Loki, CloudWatch), this may be captured and stored in plaintext.

**Recommendation:** Document this in the operations runbook. Consider an option to
suppress pairing code from stdout and deliver it via a separate out-of-band mechanism
(e.g., write to a temp file with restricted permissions).

**Blast radius:** Documentation change only; code change optional.

---

## 12. Validation Evidence

The following checks were run as part of this analysis:

- Static codebase traversal across all `src/security/**`, `src/tools/**`, `src/gateway/**`,
  `src/runtime/**`, `src/memory/**`, `src/auth/**`
- Dependency version audit against `Cargo.toml` and `deny.toml`
- SQL injection pattern search (no raw concatenation found)
- Unsafe block enumeration (5 total, all isolated)
- Test function audit for security coverage (policy, shell, secrets, file, HTTP tools)
- CI workflow review (`sec-codeql.yml`, `sec-audit.yml`)
- Container configuration review (`Dockerfile`)

Full test suite command (not run as part of this document):
```bash
cargo test
cargo clippy --all-targets -- -D warnings
```

---

## 13. Summary Score

| Area | Rating | Notes |
|------|--------|-------|
| Authentication | Strong | Token hashing, rate limiting, PKCE |
| Secret storage | Strong | ChaCha20-Poly1305, random nonces, file permissions |
| Command sandboxing | Strong | Quote-aware parser, forbidden paths, timeouts |
| Network security | Strong | SSRF protection, HMAC verification, size limits |
| Database | Strong | Parameterized queries throughout |
| Supply chain | Strong | cargo-deny, CodeQL, weekly scans |
| Replay protection | Needs work | Webhook timestamp validation missing |
| Key rotation | Needs work | No expiration tracking for stored tokens |
| Container | Strong | Distroless nonroot, minimal surface |
| Unsafe code | Strong | 5 blocks total, none in security paths |

**Overall: Strong baseline. No critical vulnerabilities identified. Two medium-priority
hardening items recommended (replay protection, token expiration).**

---

## 14. References

- `src/security/policy.rs` — command validation and risk classification
- `src/security/secrets.rs` — secret encryption and migration
- `src/security/pairing.rs` — gateway authentication
- `src/security/audit.rs` — audit event types
- `src/tools/shell.rs` — shell execution and env isolation
- `src/tools/http_request.rs` — HTTP tool with SSRF protection
- `src/gateway/mod.rs` — gateway rate limiting and webhook verification
- `src/memory/sqlite.rs` — parameterized database access
- `Cargo.toml` / `deny.toml` — dependency and license policy
- `.github/workflows/sec-codeql.yml` — static analysis CI
- `.github/workflows/sec-audit.yml` — dependency vulnerability CI
- `docs/security-roadmap.md` — OS-level sandboxing proposals
- `docs/sandboxing.md` — sandboxing design proposals
- `docs/audit-logging.md` — audit logging proposals
- `SECURITY.md` — vulnerability reporting policy
