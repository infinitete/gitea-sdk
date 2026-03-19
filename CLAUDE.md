# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Async Rust SDK for the Gitea REST API. Edition 2024, MSRV 1.88. Ported from the Go SDK. Uses `reqwest` for HTTP, `serde` for serialization, `thiserror` for errors, `tokio` for async, `wiremock` for test mocking.

## Build / Lint / Test Commands

```bash
cargo build                                                          # debug build
cargo check                                                          # fast type-check
cargo fmt --check                                                    # format check (CI enforces)
cargo clippy --all-targets --all-features -- -D warnings             # lint (CI enforces)
cargo doc --no-deps                                                  # doc generation (CI enforces)
cargo test --lib                                                     # unit tests (CI)
cargo test --doc                                                     # doc tests (CI)
cargo test                                                           # all tests
cargo test test_client_build_token                                   # single test by partial name
cargo test --lib -- test_client_build_token --exact                  # exact match
cargo test --test integration_test                                   # specific integration file
cargo test --test integration_test -- test_version_wiremock          # specific test in file
cargo test --test live_integration_test -- --ignored --nocapture     # live tests (need .env)
```

**Full CI check (run before committing):**
```bash
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo doc --no-deps && cargo test --lib && cargo test --doc
```

There is also a pre-commit hook in `.githooks/pre-commit` that runs fmt, clippy, and tests.

## Architecture

**Core components** (`src/`):
- `client.rs` — `Client` + `ClientBuilder` (builder pattern, thread-safe via `parking_lot::RwLock`)
- `error.rs` — `Error` enum (thiserror), `Result<T>` alias. Never use `anyhow` (library crate).
- `response.rs` — `Response` struct with status, headers, parsed pagination links
- `pagination.rs` — `ListOptions` + `QueryEncode` trait for URL query encoding
- `version.rs` — Server version detection (lazy `OnceLock`), `VERSION_*` constants via `version_const!` macro

**API modules** (`src/api/`): One struct per domain (`ReposApi`, `IssuesApi`, `PullsApi`, etc.), each holding a `&'a Client` reference. Methods return `crate::Result<(T, Response)>`. Paths are relative to `/api/v1` (client prepends this automatically).

**Types** (`src/types/`): Data structs mirroring Gitea API entities. `enums.rs` has shared enums with strum derives. `serde_helpers.rs` has `nullable_rfc3339` (handles Go's zero-time quirk) and `null_to_default`.

**Options** (`src/options/`): Request parameter structs. Pagination structs embed `ListOptions`. All implement `QueryEncode`.

**Auth** (`src/auth/`): SSH cert signing, SSH agent auth, HTTP signatures, Pageant (Windows), webhook HMAC-SHA256 verification.

**Internal** (`src/internal/`): Path validation/escaping (`escape.rs`), HTTP pipeline, query building, request helpers.

**Tests** (`tests/`): Integration tests use `wiremock` via helpers in `tests/common/`. Live tests (`live_*.rs`) are `#[ignore]` and require `.env` with `GITEA_URL` and `GITEA_TOKEN`.

## Key Patterns

- **All type structs** derive `Debug, Clone, Serialize, Deserialize`. Use `#[serde(default)]` on `Option<T>`/`Vec<T>` fields, `skip_serializing_if = "Option::is_none"` on optionals, `with = "nullable_rfc3339"` for `Option<OffsetDateTime>`.
- **Enums** derive `Display, AsRefStr` from strum with `#[serde(rename_all = "lowercase")]`. Use `#[serde(other)]` for catch-all variants.
- **User-supplied path segments** (owner, repo name) must go through `crate::internal::escape::validate_and_escape_segments`.
- **Imports**: std first, then external crates, then `crate::*`. Use `crate::` paths for cross-module references. For serde in type files: `use crate::{Deserialize, Serialize};`.
- **File headers**: Copyright comment on every source file (except test-only files).
- **Section separators**: `// ── Title ─────────────────────────────────` with box-drawing chars.
- **Never suppress clippy warnings** with `#[allow]` unless explicitly justified.

## Feature Flags

| Feature | Description |
|---|---|
| `rustls-tls` (default) | Use rustls for TLS |
| `native-tls` | System native TLS |
| `stream` | Streaming response support |
