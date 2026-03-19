# Pre-Release Audit Fixes Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Fix all issues identified in the 7-dimension code audit before publishing v0.1.0

**Architecture:** 8 independent fix tasks targeting specific files, grouped for maximum parallelism. Each task is self-contained — no task depends on another's output.

**Tech Stack:** Rust, serde, time crate, parking_lot

---

## Parallel Group A — Type Safety Fixes (4 tasks, all independent)

### Task 1: Fix Secret.created serde helper

**Files:**
- Modify: `src/types/secret.rs:7-25`

**Context:** The `created` field uses `time::serde::rfc3339::option` which cannot handle Go's zero-time value (`0001-01-01T00:00:00Z`). Must use the project's custom `nullable_rfc3339` helper.

**Step 1: Update imports in secret.rs**

Replace lines 7-9:
```rust
use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;
```
With:
```rust
use crate::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::serde_helpers::nullable_rfc3339;
```

**Step 2: Fix the serde attribute on `created` field**

Replace lines 23-25:
```rust
    #[serde(default, with = "rfc3339::option")]
    pub created: Option<OffsetDateTime>,
```
With:
```rust
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<OffsetDateTime>,
```

**Step 3: Run tests**

Run: `cargo test --lib -- secret`
Expected: All secret tests pass

---

### Task 2: Custom Debug for Oauth2 (redact client_secret)

**Files:**
- Modify: `src/types/oauth2.rs:14-15`

**Context:** `Oauth2` derives `Debug` which exposes `client_secret` in logs. Must implement custom Debug that redacts it, following the pattern in `src/client.rs:41-61`.

**Step 1: Remove Debug from derive, add manual impl**

Change line 14 from:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
```
To:
```rust
#[derive(Clone, Serialize, Deserialize)]
```

Then add after the struct definition (before `#[cfg(test)]`):
```rust
impl std::fmt::Debug for Oauth2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Oauth2")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("client_id", &self.client_id)
            .field(
                "client_secret",
                &if self.client_secret.is_empty() {
                    ""
                } else {
                    "***"
                },
            )
            .field("redirect_uris", &self.redirect_uris)
            .field("confidential_client", &self.confidential_client)
            .field("created", &self.created)
            .finish()
    }
}
```

**Step 2: Add a test verifying secret is redacted**

Add to the existing `mod tests` block:
```rust
    #[test]
    fn test_oauth2_debug_redacts_secret() {
        let app = Oauth2 {
            id: 1,
            name: "My App".to_string(),
            client_id: "abc123".to_string(),
            client_secret: "supersecret".to_string(),
            redirect_uris: vec![],
            confidential_client: false,
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
        };
        let debug = format!("{:?}", app);
        assert!(!debug.contains("supersecret"), "client_secret must be redacted in Debug output");
        assert!(debug.contains("***"), "Debug output should contain redaction marker");
    }
```

**Step 3: Run tests**

Run: `cargo test --lib -- oauth2`
Expected: All oauth2 tests pass

---

### Task 3: Custom Debug for AccessToken (redact token)

**Files:**
- Modify: `src/types/user.rs:14`

**Context:** `AccessToken` derives `Debug` which exposes the full `token` field. Must redact it, showing only `token_last_eight`.

**Step 1: Remove Debug from AccessToken derive, add manual impl**

Change line 14 from:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
```
To:
```rust
#[derive(Clone, Serialize, Deserialize)]
```

Then add after `AccessToken` struct definition (after the closing `}` on line 37, before `UserHeatmapData`):
```rust
impl std::fmt::Debug for AccessToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AccessToken")
            .field("id", &self.id)
            .field("name", &self.name)
            .field(
                "token",
                &if self.token.is_empty() {
                    ""
                } else {
                    "***"
                },
            )
            .field("token_last_eight", &self.token_last_eight)
            .field("scopes", &self.scopes)
            .field("created", &self.created)
            .field("updated", &self.updated)
            .finish()
    }
}
```

**Step 2: Add a test verifying token is redacted**

Add to the existing `mod tests` block:
```rust
    #[test]
    fn test_access_token_debug_redacts_token() {
        let token = AccessToken {
            id: 1,
            name: "ci-token".to_string(),
            token: "ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string(),
            token_last_eight: "xxxxxxxx".to_string(),
            scopes: vec![],
            created: None,
            updated: None,
        };
        let debug = format!("{:?}", token);
        assert!(!debug.contains("ghp_"), "Full token must be redacted in Debug output");
        assert!(debug.contains("***"), "Debug output should contain redaction marker");
        assert!(debug.contains("xxxxxxxx"), "token_last_eight should still be visible");
    }
```

**Step 3: Run tests**

Run: `cargo test --lib -- user`
Expected: All user tests pass

---

### Task 4: Fix PullReviewComment line_num types (u64 → i64)

**Files:**
- Modify: `src/types/pull_request.rs:232-234`

**Context:** `line_num` and `old_line_num` use `u64` but the Gitea API returns signed integers. Using `u64` can cause deserialization failures.

**Step 1: Change types**

Replace lines 231-234:
```rust
    #[serde(rename = "position")]
    pub line_num: u64,
    #[serde(rename = "original_position")]
    pub old_line_num: u64,
```
With:
```rust
    #[serde(rename = "position")]
    pub line_num: i64,
    #[serde(rename = "original_position")]
    pub old_line_num: i64,
```

**Step 2: Update test values if needed**

Check existing test in same file — the test creates `PullReviewComment` with these fields. Update any `u64` literal values if the compiler complains (they should auto-coerce, but verify).

**Step 3: Run tests**

Run: `cargo test --lib -- pull_request`
Expected: All pull_request tests pass

---

## Parallel Group B — Core Fixes (2 tasks, independent)

### Task 5: Fix ClientBuilder::build() panic → Result

**Files:**
- Modify: `src/client.rs:610-618`

**Context:** `reqwest::Client::builder().build().expect(...)` is the only panic point in library code. Must convert to proper error propagation.

**Step 1: Replace expect with map_err and ?**

Replace lines 610-618:
```rust
        let http = self.http_client.unwrap_or_else(|| {
            reqwest::Client::builder()
                .timeout(timeout)
                .connect_timeout(connect_timeout)
                .tcp_keepalive(tcp_keepalive)
                .pool_max_idle_per_host(pool_max_idle_per_host)
                .build()
                .expect("failed to build default HTTP client")
        });
```
With:
```rust
        let http = match self.http_client {
            Some(client) => client,
            None => reqwest::Client::builder()
                .timeout(timeout)
                .connect_timeout(connect_timeout)
                .tcp_keepalive(tcp_keepalive)
                .pool_max_idle_per_host(pool_max_idle_per_host)
                .build()?,
        };
```

Note: `reqwest::Error` already has `#[from]` conversion in `crate::Error::Request`, so `?` works directly.

**Step 2: Run tests**

Run: `cargo test --lib -- client`
Expected: All client tests pass (existing tests use `.unwrap()` on `build()` which is fine in test code)

---

### Task 6: Fix enums.rs serde import convention

**Files:**
- Modify: `src/types/enums.rs:7`

**Context:** Per CLAUDE.md, type files should use `use crate::{Deserialize, Serialize};` instead of `use serde::{Deserialize, Serialize};`.

**Step 1: Replace import**

Change line 7 from:
```rust
use serde::{Deserialize, Serialize};
```
To:
```rust
use crate::{Deserialize, Serialize};
```

**Step 2: Run tests**

Run: `cargo test --lib -- enums`
Expected: All enum tests pass

---

## Parallel Group C — Metadata Fixes (2 tasks, independent)

### Task 7: Add copyright headers + IssueBlockedBy export to lib.rs

**Files:**
- Modify: `src/lib.rs:1` (add copyright header)
- Modify: `src/lib.rs:42-62` (add IssueBlockedBy to re-exports)

**Step 1: Add copyright header**

Prepend to the very top of lib.rs (before `//! Gitea API client`):
```rust
// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

```

**Step 2: Add IssueBlockedBy to type re-exports**

In the `pub use types::{...}` block, add `IssueBlockedBy` after `Issue,` on the line that currently reads:
```rust
    InternalTracker, Issue, IssueFormElement, IssueFormElementAttributes,
```
Change to:
```rust
    InternalTracker, Issue, IssueBlockedBy, IssueFormElement, IssueFormElementAttributes,
```

**Step 3: Run tests**

Run: `cargo test --lib -- test_public`
Expected: Compilation succeeds, existing tests pass

---

### Task 8: Add copyright header to pagination.rs

**Files:**
- Modify: `src/pagination.rs:1`

**Step 1: Add copyright header**

Prepend to the very top of pagination.rs (before `//! Pagination options`):
```rust
// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

```

**Step 2: Run tests**

Run: `cargo test --lib -- pagination`
Expected: All pagination tests pass

---

## Final Verification

### Task 9: Full CI check

**Step 1: Run complete CI pipeline**

Run: `cargo fmt --check && cargo clippy --all-targets --all-features -- -D warnings && cargo doc --no-deps && cargo test --lib && cargo test --doc`

Expected: All checks pass with zero warnings.

**Step 2: Commit all changes**

```bash
git add src/types/secret.rs src/types/oauth2.rs src/types/user.rs src/types/pull_request.rs src/client.rs src/types/enums.rs src/lib.rs src/pagination.rs
git commit -m "fix: 发布前审计问题修复

- 修复 Secret.created 使用错误的 serde helper (rfc3339::option → nullable_rfc3339)
- 为 Oauth2, AccessToken 实现自定义 Debug 以隐藏敏感字段
- 修复 PullReviewComment.line_num/old_line_num 类型 (u64 → i64)
- 消除 ClientBuilder::build() 中唯一的 panic 点 (expect → ?)
- 修复 enums.rs 中 serde 导入不一致 (serde:: → crate::)
- 补充 lib.rs 和 pagination.rs 缺失的版权注释
- 补充 IssueBlockedBy 类型的 lib.rs re-export"
```

---

## Parallelism Map

```
Group A (parallel):  Task 1  |  Task 2  |  Task 3  |  Task 4
Group B (parallel):  Task 5  |  Task 6
Group C (parallel):  Task 7  |  Task 8
Final:               Task 9 (sequential, depends on all above)
```

All tasks in Groups A, B, and C are fully independent and can execute simultaneously.
Total: 8 fix tasks + 1 verification task.
