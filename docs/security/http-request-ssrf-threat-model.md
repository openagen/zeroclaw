# SSRF Threat Model — `http_request` Tool

**Status:** Current behavior (not a proposal)
**Date:** 2026-02-22
**Scope:** `src/tools/http_request.rs`

---

## 1. Threat: Server-Side Request Forgery (SSRF)

SSRF allows an attacker to cause the agent to issue HTTP requests to unintended
targets — typically internal services, metadata endpoints, or loopback addresses —
by controlling the `url` parameter of the `http_request` tool.

---

## 2. Existing Defenses

### 2.1 Required Domain Allowlist (fail-closed)

The tool refuses all requests if `http_request.allowed_domains` is empty.
An agent without an explicit allowlist cannot make any external HTTP request.

```toml
[http_request]
allowed_domains = ["api.example.com", "cdn.example.com"]
```

Wildcard subdomains are supported: a domain entry `example.com` also permits
`api.example.com`, `v2.api.example.com`, etc., via suffix matching.

### 2.2 Private and Local Host Blocking

The `is_private_or_local_host()` function blocks:

| Range | Example |
|---|---|
| Loopback IPv4 | `127.0.0.1` – `127.255.255.255` |
| Private RFC 1918 | `10.x`, `172.16–31.x`, `192.168.x` |
| Link-local | `169.254.x.x` |
| Shared address space (RFC 6598) | `100.64–127.x.x` |
| Cloud metadata typical range | `169.254.169.254` (via link-local) |
| Documentation ranges | `192.0.2/24`, `198.51.100/24`, `203.0.113/24` |
| Benchmarking range | `198.18–19.x.x` |
| Broadcast / unspecified | `255.255.255.255`, `0.0.0.0` |
| Multicast | `224.0.0.0/4` |
| Reserved | `240.0.0.0/4` |
| Loopback / link-local IPv6 | `::1`, `fe80::/10` |
| Unique-local IPv6 | `fc00::/7` |
| IPv4-mapped IPv6 | `::ffff:127.0.0.1`, `::ffff:192.168.x.x` |
| `.localhost` subdomains | `evil.localhost` |
| `.local` TLD | `service.local` |

### 2.3 Redirect Following Disabled

`reqwest` is configured with `Policy::none()` — the client does not follow HTTP
redirects. A redirect to `http://169.254.169.254/` cannot succeed.

### 2.4 Scheme Restriction

Only `http://` and `https://` schemes are accepted. `file://`, `ftp://`,
`gopher://`, and all other schemes are rejected at validation.

### 2.5 URL Userinfo Blocked

URLs containing `@` in the authority component (`user@host`) are rejected to
prevent credential-embedding bypass.

### 2.6 IPv6 Literal Blocked

IPv6 literal hosts (`[::1]`) are rejected entirely, as they would require
additional parsing to detect private ranges.

### 2.7 Alternate IP Notation (defense-in-depth)

Octal (`0177.0.0.1`), hex (`0x7f000001`), and integer-decimal (`2130706433`)
notations are not parsed as IP addresses by Rust's standard library. These fall
through to allowlist rejection because no allowlist entry matches them.
Tests in `src/tools/http_request.rs` document and verify this behavior.

### 2.8 Autonomy and Rate-Limit Gating

The tool is blocked entirely in `ReadOnly` autonomy mode, and subject to
`max_actions_per_hour` rate limiting.

---

## 3. Known Gaps and Residual Risks

### 3.1 DNS Rebinding (Post-Resolution Attack)

**Risk level:** Medium

**Description:** The IP check in `is_private_or_local_host()` operates on the
hostname string, not on the IP address resolved at connection time. An attacker
who controls a DNS server can:
1. First resolution: return a public IP → allowlist passes, host-block passes.
2. Second resolution (at `connect()` time): return an internal IP.

This race window is inherent to any SSRF defense that does not pin the resolved
IP after the check. The `Policy::none()` redirect defense does not protect against
this.

**Mitigation path:** Connect-then-verify (post-`connect()` IP check) or a local
DNS resolver with TTL=0 rejection. Neither is currently implemented. This is a
known accepted risk for the current implementation.

**Recommended operator action:** Run the agent behind a network-level egress
filter (firewall, proxy) that blocks all non-public destinations, rather than
relying solely on the application-layer check.

### 3.2 Cloud Metadata Endpoints via Hostname

**Risk level:** Low (mitigated by allowlist)

**Description:** Cloud metadata endpoints such as `http://metadata.google.internal/`
resolve to link-local IPs, which are blocked. However, some provider-specific
metadata hostnames may resolve to public-range IPs depending on the cloud provider.

**Mitigation:** The required allowlist is the primary defense: metadata hostnames
are not in any legitimate allowlist.

### 3.3 HTTP (Non-TLS) Requests Permitted

**Risk level:** Low

**Description:** `http://` URLs are accepted in addition to `https://`. Plaintext
HTTP is susceptible to interception and MITM. In environments where the agent
operates over untrusted networks, an HTTPS-only policy would be stronger.

**Operator option:** Restrict `allowed_domains` to services known to enforce HTTPS.
No config key exists today to enforce HTTPS-only; this would be a future hardening
option.

### 3.4 Header Injection via User-Controlled Keys

**Risk level:** Low

**Description:** The `headers` parameter accepts arbitrary key-value pairs.
`reqwest` normalizes headers and rejects most invalid formats, but deliberately
crafted header values that include CRLF could, in theory, affect HTTP/1.1 framing.
`reqwest` guards against CRLF injection in header values.

**Status:** Accepted risk, delegated to `reqwest`.

---

## 4. Allowlist Configuration Guidance

```toml
[http_request]
# Only allowlist domains the agent legitimately needs.
# Wildcards match subdomains automatically.
allowed_domains = [
    "api.openai.com",
    "slack.com",
]

# Keep the list as narrow as possible.
# Do NOT add *.com, *.io, or other broad entries.
# Empty list = tool disabled (fail-closed default).
```

---

## 5. Test Coverage

The following test categories exist in `src/tools/http_request.rs`:

- Allowlist enforcement (exact + subdomain + miss)
- All RFC 1918 / loopback / multicast / reserved ranges
- IPv4-mapped IPv6 SSRF variants
- Alternate notation bypass (octal, hex, decimal integer, zero-padded)
- Userinfo rejection, IPv6 literal rejection
- Redirect policy validation (structural)
- Rate-limit and read-only autonomy blocking
- Header redaction for sensitive keys

---

## 6. Rollback

This document describes current behavior. No code changes are associated.
If `http_request` tool behavior changes, update this document to keep it in sync
with `src/tools/http_request.rs`.
