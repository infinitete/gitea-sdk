# AGENTS.md — Gitea Rust SDK

Guidelines for agentic coding agents working in this repository.

## Project Overview

Async Rust SDK for the Gitea REST API (edition 2024, MSRV 1.88). Ported from the Go SDK (`go-sdk/gitea`). Uses `reqwest` for HTTP, `serde` for serialization, `thiserror` for errors, `tokio` for async runtime, `wiremock` for integration test mocking.

## Build / Lint / Test Commands

```bash
# Build
cargo build                          # debug
cargo build --release                # release
cargo check                          # fast type-check (used in MSRV CI)

# Format (no rustfmt.toml — uses Rust defaults)
cargo fmt                            # auto-format
cargo fmt --check                    # check only (CI enforces this)

# Lint
cargo clippy --all-targets --all-features -- -D warnings   # CI enforces this
cargo clippy --all-targets -- -D warnings                  # without extra features

# Doc
cargo doc --no-deps                  # CI enforces this

# Test
cargo test --lib                     # unit tests only (CI)
cargo test --doc                     # doc tests only (CI)
cargo test                           # all tests (lib + doc + integration)

# Single test
cargo test test_client_build_token                        # unit test by partial name
cargo test --lib -- test_client_build_token --exact        # exact match
cargo test --test integration_test                        # specific integration file
cargo test --test integration_test -- test_version_wiremock  # specific test in file
cargo test --lib pagination::tests::test_                  # module-scoped tests

# Live tests (require real Gitea instance + .env file with GITEA_URL, GITEA_TOKEN)
cargo test --test live_integration_test -- --ignored --nocapture
```

**CI full check** (run locally before committing):
```bash
cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo doc --no-deps && cargo test --lib && cargo test --doc
```

## Project Structure

```
src/
  lib.rs              # crate root, re-exports, top-level tests
  client.rs           # Client + ClientBuilder
  error.rs            # Error enum (thiserror), Result alias
  response.rs         # Response struct, Link header parsing
  pagination.rs       # ListOptions, QueryEncode trait
  version.rs          # Server version detection, VERSION_* constants
  api/                # One file per API domain (repos, issues, pulls, etc.)
  types/              # Data structs mirroring Gitea API entities
    enums.rs          # Shared enums (StateType, etc.) using strum + serde
    serde_helpers.rs  # Custom serde helpers (nullable_rfc3339, null_to_default)
  options/            # Request option structs (ListReposOptions, etc.)
  auth/               # SSH signing, HTTP signatures, agent auth
  internal/           # Private helpers (escape, http pipeline, query)
tests/
  common/             # wiremock test helpers (create_test_client, mock_json_response)
  live/               # Live test helpers (live_client, CleanupRegistry, fixtures)
  *_test.rs           # Integration tests (mocked with wiremock)
  live_*_test.rs      # Live tests against real Gitea (#[ignore])
examples/
  basic_usage.rs      # Basic API usage examples
  authentication.rs   # Authentication method examples
```

## Code Style Guidelines

### Formatting

- Use `cargo fmt` defaults (no custom rustfmt config). **Always run `cargo fmt` before committing.**
- Max line length: 100 (Rust default).
- Section separators use `// ── Title ─────────────────────────────────` with box-drawing chars.

### Imports

```rust
// Order: std → external crates → crate-internal
use std::sync::{Arc, OnceLock};

use parking_lot::RwLock;

use crate::Client;
use crate::api::{ReposApi, IssuesApi};
use crate::pagination::{ListOptions, QueryEncode};
```

- Group `std` first, then external crates, then `crate::*`.
- Use `crate::` paths (not `self::` or relative) for cross-module references within the crate.
- For serde in type files: `use crate::{Deserialize, Serialize};` (re-exported from lib.rs).

### File Header

Every source file starts with a copyright comment (except test-only files):
```rust
// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
```

### Documentation

- Module-level: `//!` doc comments describing the module's purpose.
- Public items: `///` doc comments with descriptions. Include `# Examples` and `# Errors` sections where applicable.
- Inline code in docs uses backtick notation.
- Doc comment style for entities: `/// Entity description from Gitea docs.` followed by struct.

### Naming Conventions

- **Structs**: `PascalCase` matching Gitea API entity names (`Repository`, `PullRequest`, `AccessToken`).
- **Enums**: `PascalCase` variants (`StateType::Open`, `IssueType::Pull`).
- **API structs**: `PascalCase` + `Api` suffix (`ReposApi`, `IssuesApi`).
- **Options structs**: `PascalCase` + `Option` suffix (`ListReposOptions`, `CreateRepoOption`).
- **Functions**: `snake_case` (`list_my_repos`, `create_repo`).
- **Module files**: `snake_case` matching the API domain (`repos.rs`, `pulls.rs`).

### Type Definitions (types/)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Entity description from Gitea docs.
pub struct EntityName {
    pub id: i64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub optional_field: Option<String>,
    #[serde(default, rename = "api_field_name")]
    pub rust_field_name: String,
    #[serde(default, with = "nullable_rfc3339", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<OffsetDateTime>,
}
```

- Always derive `Debug, Clone, Serialize, Deserialize`.
- Use `#[serde(default)]` on all `Option<T>` and `Vec<T>` fields to handle missing JSON keys.
- Use `#[serde(rename = "...")]` to match Gitea's JSON field names when they differ from Rust naming.
- Use `skip_serializing_if = "Option::is_none"` on optional fields.
- Use `with = "nullable_rfc3339"` for `Option<OffsetDateTime>` fields (handles Go's zero-time quirk).
- Import serde helpers via `use super::serde_helpers::{null_to_default, nullable_rfc3339};`.

### Enums (types/enums.rs)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum StateType {
    Open,
    Closed,
    #[serde(other)]
    All,
}
```

- Derive `Display` and `AsRefStr` from strum for serialization as strings.
- Use `#[serde(other)]` catch-all variant when appropriate.

### Options (options/)

```rust
#[derive(Debug, Clone, Default)]
pub struct ListReposOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListReposOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}
```

- Options with pagination embed `ListOptions`.
- Implement `QueryEncode` for all option types that produce URL query strings.
- Fields use `String` (not `&str`), `bool`, `i64`, `Option<bool>`, `Option<String>`.
- Validation logic lives on the option struct (e.g., `opt.validate()?`).

### API Methods (api/)

```rust
pub struct ReposApi<'a> { client: &'a Client }

impl<'a> ReposApi<'a> {
    pub fn new(client: &'a Client) -> Self { Self { client } }
    pub(crate) fn client(&self) -> &'a Client { self.client }

    /// ListMyRepos list all repositories of the authenticated user
    pub async fn list_my_repos(
        &self,
        opt: ListReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let path = format!("/user/repos?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}
```

- Each API struct holds a `&'a Client` reference.
- Methods return `crate::Result<(T, Response)>` — parsed body + response metadata.
- Doc comments preserve Go SDK method names: `/// ListMyRepos list all...`
- Path segments from user input must go through `crate::internal::escape::validate_and_escape_segments`.
- Use `reqwest::Method::GET`/`POST`/`DELETE`/`PATCH` explicitly.

### Error Handling

- Use `crate::Error` (thiserror enum) and `crate::Result<T>` type alias.
- Never use `anyhow` — this is a library crate.
- Error variants: `Request`, `Api`, `UnknownApi`, `Validation`, `Version`, `UnknownVersion`, `SshSign`, `Json`, `Url`.
- Use `#[from]` for transparent conversions where appropriate.
- Construct errors with descriptive messages: `Error::Validation("owner is empty".into())`.

### Async Patterns

- All API methods are `async fn` using `reqwest` (no explicit tokio runtime pinning).
- Tests use `#[tokio::test]` for async tests.
- Thread-safety via `parking_lot::RwLock` (not `std::sync::RwLock`) and `tokio::sync::Mutex` for async locks.

### Tests

- **Unit tests**: Inline `#[cfg(test)] mod tests { ... }` at the bottom of each source file.
- **Integration tests**: `tests/*.rs` using `wiremock` for HTTP mocking. Use `tests::common::*` helpers.
- **Live tests**: `tests/live_*.rs` with `#[ignore]` attribute. Require `.env` file with Gitea credentials.
- Test naming: `test_<what>_<expectation>` (e.g., `test_client_build_invalid_url`).
- Prefer assertion macros (`assert!`, `assert_eq!`, `assert!(result.is_err())`) over `unwrap()` in tests.

## Feature Flags

| Feature | Description |
|---|---|
| `rustls-tls` (default) | Use rustls for TLS |
| `native-tls` | Use system native TLS (`reqwest/native-tls`) |
| `stream` | Enable streaming response support |

Test with `--all-features` to cover all configurations.

## Common Pitfalls

- **Never suppress clippy warnings** with `#[allow]` unless explicitly justified.
- **Never use `as any`** — this crate has strict typing.
- **Go zero-time**: Gitea serializes unset `time.Time` as `"0001-01-01T00:00:00Z"`. Always use `nullable_rfc3339` serde helper for datetime fields.
- **Missing JSON fields**: Always use `#[serde(default)]` on optional/vec fields — the API may omit them.
- **Path escaping**: User-supplied path segments (owner, repo name) must go through `validate_and_escape_segments` before interpolation.
- **API version prefix**: Paths are relative to `/api/v1` — the client prepends this automatically. Do not include it in method paths.
