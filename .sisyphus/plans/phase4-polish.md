# Phase 4: Polish & Documentation

## TL;DR

> **Quick Summary**: Polish the completed Rust SDK for crates.io publication — fix documentation bugs, clean up lint suppressions, add missing metadata/files, write README+CHANGELOG, and set up CI.
> 
> **Deliverables**:
> - Fixed 131 dual doc comment bugs across 26 type files (rustdoc data loss)
> - Clean lint: zero `#[allow(dead_code)]`, zero `#[allow(unused_imports)]`
> - Complete `Cargo.toml` crates.io metadata + LICENSE file
> - Enriched crate-level docs in `lib.rs`
> - Professional README.md with usage examples
> - CHANGELOG.md for v0.1.0
> - GitHub Actions CI workflow
> 
> **Estimated Effort**: Medium
> **Parallel Execution**: NO — sequential (5 tasks with dependencies)
> **Critical Path**: Task 1 → Task 2 → Task 3 → Task 4 → Task 5

---

## Context

### Original Request
Phase 4 of the Go → Rust SDK migration plan (`docs/migration-plan.md`): Polish & Documentation. The SDK is fully implemented (73 source files, 412+ API methods, 270 types, 15 API sub-modules).

### Interview Summary
**Key Discussions**:
- Phase 4 has 6 tasks: rustdoc, README, CHANGELOG, CI/CD, clippy zero warnings, publish prep
- SDK is functionally complete — no new features needed

**Research Findings**:
- **418 method-level docs already exist** across 15 API modules (NOT missing as initially assumed)
- **131 dual doc comment bugs** across 26 type files — descriptive `///` before `#[derive(...)]` is silently dropped by rustdoc; only generic "X payload type." after derive is shown
- **21 `#[allow(dead_code)]`** across 9 files; **26 `#[allow(unused_imports)]`** across 2 files
- Blanket `#![allow(dead_code)]` in `api/mod.rs`
- No CI, no CHANGELOG, no LICENSE file, missing Cargo.toml metadata
- `repository` URL in Cargo.toml incorrectly points to Go SDK
- README.md is minimal (19 lines, only git hooks info)

### Metis Review
**Identified Gaps** (addressed):
- Biggest misconception corrected: API methods are already documented. Real doc issue is dual-comment bug.
- Must NOT create task to "add method-level docs" — they exist.
- Must NOT scope-creep into adding `# Examples` to all 412 methods.
- Dual-comment fix is mechanical: move descriptive `///` after derive, delete generic "X payload type."
- CI should start minimal (single job), not over-engineered.
- License should match Go SDK (MIT-only) unless user decides otherwise.
- `types/mod.rs` `#[allow(unused_imports)]` are safe to remove (verified consumed internally).

---

## Work Objectives

### Core Objective
Prepare the Rust SDK for crates.io publication with professional documentation, clean code quality, and CI.

### Concrete Deliverables
- All 26 type files with correct single rustdoc comment per struct/enum
- Zero lint suppressions that can be safely removed
- `Cargo.toml` with all required crates.io metadata fields
- `LICENSE` file (MIT)
- `README.md` with usage examples, feature list, badge
- `CHANGELOG.md` for v0.1.0
- `.github/workflows/ci.yml` (build, test, clippy, fmt, doc)

### Definition of Done
- [ ] `cargo clippy --all-features -- -D warnings` exits 0
- [ ] `cargo doc --no-deps` completes without warnings
- [ ] `cargo publish --dry-run` succeeds
- [ ] README.md contains valid `no_run` code examples
- [ ] CI workflow file is valid YAML

### Must Have
- All 131 dual doc comment instances fixed (no "payload type." remnants)
- Blanket `#![allow(dead_code)]` removed from `api/mod.rs`
- `types/mod.rs` `#[allow(unused_imports)]` all removed
- `Cargo.toml` has: keywords, categories, readme, documentation, homepage
- LICENSE file exists
- CHANGELOG.md exists
- CI runs on push and PR

### Must NOT Have (Guardrails)
- **NO** adding `# Examples` sections to all 412 API methods (separate project)
- **NO** rewriting existing 200+ method-level doc comments (they work fine)
- **NO** dual-licensing MIT OR Apache-2.0 (match Go SDK: MIT-only)
- **NO** over-engineered CI matrix (no cross-platform on first pass)
- **NO** `cargo-semver-checks` in v0.1.0 (pre-first-release)
- **NO** removing `#[allow(dead_code)]` from auth/internal without triage
- **NO** fixing `ssh_sign.rs` LSP error (compiles fine, false positive)
- **NO** scope creep into functional code changes

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.

### Test Decision
- **Infrastructure exists**: YES (wiremock, unit tests)
- **Automated tests**: NO new tests needed (this is polish, not functional changes)
- **Framework**: N/A
- **Verification**: cargo check, clippy, doc, cargo publish --dry-run

### QA Policy
Every task has agent-executed QA scenarios using Bash (cargo commands) and grep (structural verification).
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

---

## Execution Strategy

### Sequential Execution (5 tasks with dependencies)

```
Wave 1 (Start Immediately — documentation fix):
└── Task 1: Fix dual doc comments (26 type files) [writing]

Wave 2 (After Task 1 — lint cleanup, depends on doc fix):
└── Task 2: Clean up lint allows + fix warnings [unspecified-high]

Wave 3 (After Task 2 — metadata, depends on clean lint):
└── Task 3: Cargo.toml metadata + LICENSE + lib.rs docs [quick]

Wave 4 (After Task 3 — documentation, depends on metadata):
└── Task 4: README.md + CHANGELOG.md [writing]

Wave 5 (After Task 4 — CI, validates everything):
└── Task 5: GitHub Actions CI [quick]

Wave FINAL (After ALL tasks — verification):
├── F1: Plan compliance audit (oracle)
├── F2: Code quality review (unspecified-high)
├── F3: Real manual QA (unspecified-high)
└── F4: Scope fidelity check (deep)

Critical Path: Task 1 → 2 → 3 → 4 → 5 → F1-F4 → user okay
Max Concurrent: 1 (sequential due to dependencies)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| 1 | None | 2 |
| 2 | 1 | 3 |
| 3 | 2 | 4 |
| 4 | 3 | 5 |
| 5 | 4 | F1-F4 |

### Agent Dispatch Summary

- **Wave 1**: 1 task — T1 → `writing`
- **Wave 2**: 1 task — T2 → `unspecified-high`
- **Wave 3**: 1 task — T3 → `quick`
- **Wave 4**: 1 task — T4 → `writing`
- **Wave 5**: 1 task — T5 → `quick`
- **FINAL**: 4 tasks — F1→`oracle`, F2→`unspecified-high`, F3→`unspecified-high`, F4→`deep`

---

## TODOs

- [ ] 1. Fix Dual Doc Comments on Type Structs

  **What to do**:
  - Fix 131 dual doc comment instances across 26 type files in `src/types/`
  - **The bug**: Each struct has a descriptive `///` comment BEFORE `#[derive(...)]` (e.g., `/// AccessToken represents an API access token`) and a generic `///` comment AFTER derive (e.g., `/// Access Token payload type.`). Rustdoc only uses the comment immediately before the item, so the descriptive comment is silently dropped.
  - **The fix**: For each instance:
    1. Move the descriptive `///` comment to after the `#[derive(...)]` line
    2. Delete the generic `/// X payload type.` line
  - Process ALL 26 files identified by: `rg "payload type\." rust-sdk/src/types/`
  - After fixing all files, run `cargo doc --no-deps` to verify no doc-link warnings
  - The existing descriptive comments are generally good quality (e.g., "AccessToken represents an API access token", "User represents a user"). Keep them as-is.

  **Must NOT do**:
  - Do NOT rewrite or improve the descriptive comments — just move them
  - Do NOT add `# Examples` sections to types
  - Do NOT touch API module files (`src/api/*.rs`)
  - Do NOT touch option files (`src/options/*.rs`)
  - Do NOT use regex/sed — use ast_grep or careful Edit operations

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: Mechanical documentation fix across many files, no code logic changes
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `brainstorming`: Not creative work, this is a known bug with known fix pattern

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 1 (solo)
  - **Blocks**: Task 2
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References** (the bug pattern):
  - `src/types/user.rs:11-14` — Example of the dual-comment bug: descriptive `///` at line 11, `#[derive]` at line 12, generic `/// Access Token payload type.` at line 13
  - `src/types/repository.rs:78-80` — Another example with `Repository` struct
  - `src/types/enums.rs:12-14` — Example on enum (same pattern applies)

  **Files to Process** (all 26 type files with "payload type." instances):
  - `src/types/user.rs` (7 instances), `src/types/repository.rs` (47 instances), `src/types/enums.rs` (19 instances), `src/types/issue.rs` (14 instances), `src/types/node_info.rs` (7 instances), `src/types/pull_request.rs` (6 instances), `src/types/action.rs` (7 instances), `src/types/settings.rs` (4 instances), `src/types/package.rs` (2 instances), `src/types/notification.rs` (2 instances), `src/types/license.rs` (2 instances), `src/types/organization.rs` (2 instances), `src/types/release.rs` (2 instances), `src/types/status.rs` (2 instances), `src/types/comment.rs` (1), `src/types/activity.rs` (1), `src/types/team.rs` (1), `src/types/hook.rs` (1), `src/types/label.rs` (1), `src/types/secret.rs` (1), `src/types/reaction.rs` (1), `src/types/milestone.rs` (1), `src/types/badge.rs` (1), `src/types/cron_task.rs` (1), `src/types/oauth2.rs` (1), `src/types/user_settings.rs` (1)

  **WHY Each Reference Matters**:
  - `src/types/user.rs:11-14` — Canonical example of the fix pattern: move line 11 to after line 12, delete line 13
  - The file list above is the complete inventory of affected files — every file with "payload type." needs fixing

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: No "payload type." remnants in type files
    Tool: Bash (grep)
    Preconditions: All fixes applied
    Steps:
      1. Run: rg "payload type\." rust-sdk/src/types/ --count-matches
      2. Assert output shows 0 matches
    Expected Result: No files contain "payload type." string
    Failure Indicators: Any file returns count > 0
    Evidence: .sisyphus/evidence/task-1-no-payload-type-grep.txt

  Scenario: Cargo doc succeeds without warnings
    Tool: Bash (cargo doc)
    Preconditions: All fixes applied
    Steps:
      1. Run: cargo doc --no-deps 2>&1
      2. Assert exit code 0
      3. Assert no "warning" in output
    Expected Result: Documentation generates successfully with zero warnings
    Failure Indicators: Non-zero exit code, doc-link warnings, "cannot resolve" errors
    Evidence: .sisyphus/evidence/task-1-cargo-doc.txt

  Scenario: Dual doc comment pattern eliminated
    Tool: Bash (grep)
    Preconditions: All fixes applied
    Steps:
      1. Run: rg -U '#\[derive\([^]]*\)\]\n/// ' rust-sdk/src/types/ --multiline -c
      2. Assert all files show 0 (no doc comment immediately after derive without struct)
    Expected Result: No derive-then-doc-then-struct patterns
    Failure Indicators: Any file shows count > 0
    Evidence: .sisyphus/evidence/task-1-no-dual-docs.txt

  Scenario: Descriptive docs preserved (spot check)
    Tool: Bash (grep)
    Preconditions: All fixes applied
    Steps:
      1. Run: rg "AccessToken represents" rust-sdk/src/types/user.rs
      2. Assert match found (descriptive doc was preserved, not deleted)
    Expected Result: Descriptive doc comment exists in file
    Failure Indicators: No match found (descriptive doc was accidentally deleted)
    Evidence: .sisyphus/evidence/task-1-descriptive-docs-preserved.txt
  ```

  **Commit**: YES
  - Message: `docs: fix dual doc comments on type structs`
  - Files: `src/types/*.rs` (26 files)
  - Pre-commit: `cargo check --all-features && cargo doc --no-deps`

- [ ] 2. Clean Up Lint Allows and Fix Warnings

  **What to do**:
  - Remove blanket `#![allow(dead_code)]` from `src/api/mod.rs` (line 5)
  - Remove all 25 `#[allow(unused_imports)]` from `src/types/mod.rs` (lines 5, 10, 12, 14, 16, 18, 20, 26, 28, 30, 32, 37, 39, 41, 43, 45, 49, 51, 53, 64, 66, 70, 72, 74, 76)
  - Remove `#[allow(unused_imports)]` from `src/api/admin.rs` (line 462)
  - After removing blanket `#![allow(dead_code)]` from `api/mod.rs`, run `cargo check --all-features` to expose all dead_code warnings
  - Triage each dead_code warning:
    - If code is genuinely unused and not needed → delete it
    - If code is `pub(crate)` helper that will be used later → add targeted `#[allow(dead_code)]` with a TODO comment
    - If code is part of the public API → it should not trigger the warning (verify)
  - For the 15 item-level `#[allow(dead_code)]` in other files (version.rs, pagination.rs, client.rs, internal/http.rs, internal/escape.rs, internal/query.rs, auth/ssh_sign.rs, auth/httpsig.rs, auth/ssh_agent.rs): remove each one, run `cargo check`, and triage
  - Final state: `cargo clippy --all-features -- -D warnings` must exit 0

  **Must NOT do**:
  - Do NOT remove `#[allow(dead_code)]` from auth/internal modules without verification — some may be needed for future use or cross-platform
  - Do NOT delete genuinely useful code just to eliminate warnings
  - Do NOT add `#![allow(...)]` at module level — use item-level `#[allow]` only when truly needed
  - Do NOT fix the `ssh_sign.rs:105` LSP error — it compiles fine (false positive)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Lint triage requires careful judgment about what to keep vs delete
  - **Skills**: [`systematic-debugging`]
    - `systematic-debugging`: Useful methodology for triaging warnings systematically — categorize, investigate, resolve

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 2 (solo)
  - **Blocks**: Task 3
  - **Blocked By**: Task 1 (doc fixes first to avoid merge conflicts)

  **References**:

  **Pattern References** (lint allow locations):
  - `src/api/mod.rs:5` — `#![allow(dead_code)]` (blanket module-level, remove first)
  - `src/types/mod.rs:5-76` — 25× `#[allow(unused_imports)]` (safe to remove, consumed internally)
  - `src/api/admin.rs:462` — `#[allow(unused_imports)]` (investigate)
  - `src/version.rs:15,138` — `#[allow(dead_code)]` on version constants
  - `src/client.rs:173` — `#[allow(dead_code)]` on `write_config()`
  - `src/internal/http.rs:104,128,172` — `#[allow(dead_code)]` on HTTP helpers
  - `src/internal/escape.rs:7,15,32` — `#[allow(dead_code)]` on escape helpers
  - `src/internal/query.rs:9` — `#[allow(dead_code)]` on query helper
  - `src/auth/ssh_sign.rs:13` — `#[allow(dead_code)]`
  - `src/auth/httpsig.rs:5,22,79,91` — `#[allow(dead_code)]` on signing helpers
  - `src/auth/ssh_agent.rs:9,17,54,70,79` — `#[allow(dead_code)]` on agent helpers

  **WHY Each Reference Matters**:
  - `src/api/mod.rs:5` — This is the highest-impact removal: blanket suppression hides unknown number of warnings. Must triage individually.
  - `src/types/mod.rs` — These are barrel re-exports consumed by `lib.rs` and internal modules. Removing the allows is safe but must verify with `cargo check`.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Blanket dead_code allow removed
    Tool: Bash (grep)
    Preconditions: All lint cleanup applied
    Steps:
      1. Run: rg '#!\[allow\(dead_code\)\]' rust-sdk/src/api/mod.rs
      2. Assert no matches found
    Expected Result: No blanket allow in api/mod.rs
    Failure Indicators: Match found (blanket allow still present)
    Evidence: .sisyphus/evidence/task-2-no-blanket-allow.txt

  Scenario: All unused_imports allows removed
    Tool: Bash (grep)
    Preconditions: All lint cleanup applied
    Steps:
      1. Run: rg '#\[allow\(unused_imports\)\]' rust-sdk/src/ --count-matches
      2. Assert total count is 0
    Expected Result: Zero unused_imports allows in entire src/
    Failure Indicators: Any file still has the allow
    Evidence: .sisyphus/evidence/task-2-no-unused-imports.txt

  Scenario: Clippy passes with zero warnings
    Tool: Bash (cargo clippy)
    Preconditions: All lint cleanup applied
    Steps:
      1. Run: cargo clippy --all-features -- -D warnings 2>&1
      2. Assert exit code 0
    Expected Result: Zero clippy warnings with deny warnings
    Failure Indicators: Non-zero exit code, any warning output
    Evidence: .sisyphus/evidence/task-2-clippy-clean.txt

  Scenario: Cargo check passes
    Tool: Bash (cargo check)
    Preconditions: All lint cleanup applied
    Steps:
      1. Run: cargo check --all-features 2>&1
      2. Assert exit code 0
    Expected Result: Clean compilation with no errors
    Failure Indicators: Compilation errors from removed allows
    Evidence: .sisyphus/evidence/task-2-cargo-check.txt
  ```

  **Commit**: YES
  - Message: `refactor: clean up lint allows and fix warnings`
  - Files: `src/api/mod.rs`, `src/types/mod.rs`, `src/api/admin.rs`, + other files as needed
  - Pre-commit: `cargo check --all-features && cargo clippy --all-features -- -D warnings`

- [ ] 3. Complete Cargo.toml Metadata, Create LICENSE, Enrich lib.rs Crate Docs

  **What to do**:

  **3a. Cargo.toml metadata**:
  - Fix `repository` URL: change from `https://gitea.com/gitea/go-sdk` to the correct Rust SDK repository URL
  - Add `keywords = ["gitea", "api", "sdk", "async", "http"]` (max 5)
  - Add `categories = ["api-bindings", "web-programming::http-client", "asynchronous"]` (max 3)
  - Add `readme = "README.md"`
  - Add `documentation = "https://docs.rs/gitea-sdk"` (placeholder, update if published elsewhere)
  - Add `homepage = "https://gitea.com/gitea/go-sdk"` (or appropriate URL)
  - Add `[package.metadata.docs.rs]` section:
    ```toml
    [package.metadata.docs.rs]
    all-features = true
    rustdoc-args = ["--cfg", "docsrs"]
    ```

  **3b. LICENSE file**:
  - Create `LICENSE` file with MIT license text (matching Go SDK's license)
  - Every source file header references `a MIT-style license that can be found in the LICENSE file`

  **3c. lib.rs crate-level docs**:
  - Enrich the existing 13-line crate-level documentation
  - Add: feature overview (async, builder pattern, sub-struct API, SSH auth, pagination)
  - Add: module table listing all public modules and their purpose
  - Replace `/// ignore` example with `/// no_run` example (compilable but not executed)
  - Add: authentication section showing token and basic auth examples
  - Keep it concise — this is crate-level docs, not a tutorial

  **Must NOT do**:
  - Do NOT dual-license (MIT OR Apache-2.0) — Go SDK is MIT-only
  - Do NOT change version number (stay at 0.1.0)
  - Do NOT add new dependencies
  - Do NOT write a full tutorial in lib.rs

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Metadata fields are well-defined, LICENSE is standard text, lib.rs enrichment is bounded
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `brainstorming`: No creative decisions needed, follow crates.io conventions

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (solo)
  - **Blocks**: Task 4
  - **Blocked By**: Task 2 (clean lint before metadata for clean publish dry-run)

  **References**:

  **Pattern References**:
  - `Cargo.toml:1-8` — Current metadata (needs keywords, categories, readme, documentation, homepage)
  - `src/lib.rs:1-13` — Current crate-level docs (needs enrichment)
  - `go-sdk/LICENSE` — Go SDK's MIT license (match this exactly)

  **External References**:
  - crates.io metadata spec: https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section
  - categories list: https://crates.io/categories (api-bindings, web-programming::http-client, asynchronous)
  - docs.rs configuration: https://docs.rs/about/builds#cross-compiling

  **WHY Each Reference Matters**:
  - `Cargo.toml:1-8` — Shows what exists vs what's missing. The `repository` URL is wrong (points to go-sdk).
  - `go-sdk/LICENSE` — Must match the Go SDK's license for consistency within the monorepo.

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: LICENSE file exists and is valid MIT
    Tool: Bash
    Preconditions: LICENSE file created
    Steps:
      1. Run: test -f rust-sdk/LICENSE
      2. Run: head -1 rust-sdk/LICENSE
      3. Assert it contains "MIT License"
    Expected Result: LICENSE file exists with MIT license header
    Failure Indicators: File doesn't exist or wrong license type
    Evidence: .sisyphus/evidence/task-3-license-exists.txt

  Scenario: Cargo.toml has all required crates.io metadata
    Tool: Bash (cargo metadata)
    Preconditions: Cargo.toml updated
    Steps:
      1. Run: cargo metadata --format-version 1 | python3 -c "import sys,json; p=[x for x in json.load(sys.stdin)['packages'] if x['name']=='gitea-sdk'][0]; assert len(p.get('keywords',[]))>0, 'missing keywords'; assert len(p.get('categories',[]))>0, 'missing categories'; assert p.get('readme')=='README.md', 'missing readme'; assert 'go-sdk' not in p.get('repository',''), 'repository still points to go-sdk'; print('ALL METADATA OK')"
    Expected Result: All assertions pass, "ALL METADATA OK" printed
    Failure Indicators: Any assertion fails with descriptive error
    Evidence: .sisyphus/evidence/task-3-metadata-check.txt

  Scenario: lib.rs has enriched crate docs
    Tool: Bash (grep)
    Preconditions: lib.rs updated
    Steps:
      1. Run: wc -l rust-sdk/src/lib.rs
      2. Assert line count > 30 (enriched from current 96 lines total, but doc section should be longer)
      3. Run: rg "no_run" rust-sdk/src/lib.rs
      4. Assert match found (example uses no_run instead of ignore)
    Expected Result: Crate docs are enriched, example uses no_run
    Failure Indicators: lib.rs unchanged, or example still uses ignore
    Evidence: .sisyphus/evidence/task-3-librs-docs.txt

  Scenario: cargo check still passes after changes
    Tool: Bash (cargo check)
    Preconditions: All changes applied
    Steps:
      1. Run: cargo check --all-features 2>&1
      2. Assert exit code 0
    Expected Result: Clean compilation
    Failure Indicators: Compilation errors from metadata changes
    Evidence: .sisyphus/evidence/task-3-cargo-check.txt
  ```

  **Commit**: YES
  - Message: `metadata: add crates.io fields, LICENSE, and crate-level docs`
  - Files: `Cargo.toml`, `LICENSE`, `src/lib.rs`
  - Pre-commit: `cargo check --all-features && cargo clippy --all-features -- -D warnings`

- [ ] 4. Write README.md and CHANGELOG.md

  **What to do**:

  **4a. README.md** (replace existing 19-line file):
  - Header: `# gitea-sdk` with brief description
  - Badges: CI status badge (after Task 5 creates CI), crates.io version badge, license badge, docs.rs badge
    - Note: Badge URLs reference GitHub Actions and crates.io. Use placeholder URLs if CI not yet live.
  - Features section: list key capabilities (async, builder pattern, sub-struct API, all 15 API domains, SSH auth, pagination, wiremock-tested)
  - Installation: `cargo add gitea-sdk` or `[dependencies]` snippet
  - Quick Start section with `no_run` code blocks:
    - Creating a client with token auth
    - Getting a repository
    - Listing issues with options
    - Creating an issue
  - Authentication section: show token, basic auth, OTP, SSH cert, SSH pubkey patterns
  - Feature flags section: document `default`, `rustls-tls`, `native-tls`, `stream`
  - All code examples MUST use ` ```no_run ` (not `ignore`) so they compile under `cargo test --doc`

  **4b. CHANGELOG.md**:
  - Use [keep-a-changelog](https://keepachangelog.com/) format
  - Header: `# Changelog` with link to format spec
  - v0.1.0 section: "Initial release" with subsections:
    - Added: list all 15 API domains (Repos, Issues, Pulls, Orgs, Users, Admin, Hooks, Notifications, Actions, Releases, Settings, OAuth2, Misc, ActivityPub, Status)
    - Added: list key capabilities (async client, builder pattern, pagination, SSH auth, webhook verification)
    - Added: list test infrastructure (wiremock, SSH fixtures)

  **Must NOT do**:
  - Do NOT use ` ```ignore ` in code blocks — use ` ```no_run ` so examples compile
  - Do NOT include a full API reference (that's what docs.rs is for)
  - Do NOT include migration notes or comparison with Go SDK
  - Do NOT over-document — README should be concise (< 200 lines)
  - Do NOT add sections not listed above (e.g., Contributing, Architecture)

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: Documentation writing — README and CHANGELOG are prose tasks
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `brainstorming`: README structure is defined, no creative exploration needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 4 (solo)
  - **Blocks**: Task 5
  - **Blocked By**: Task 3 (README references metadata like feature flags, crate name)

  **References**:

  **Pattern References**:
  - `rust-sdk/README.md` — Current minimal README to replace (19 lines)
  - `rust-sdk/Cargo.toml:39-43` — Feature flags to document in README
  - `rust-sdk/src/lib.rs` — Crate-level docs (reference for feature list)
  - `docs/migration-plan.md` — Migration plan for feature/capability listing

  **External References**:
  - keep-a-changelog format: https://keepachangelog.com/en/1.1.0/
  - Good Rust SDK README examples: `octocrab` (GitHub Rust API), `reqwest` docs

  **WHY Each Reference Matters**:
  - `Cargo.toml:39-43` — README must document feature flags accurately
  - `docs/migration-plan.md` — Source of truth for what the SDK supports (API domains, auth mechanisms)

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: README exists and has proper structure
    Tool: Bash (grep)
    Preconditions: README.md written
    Steps:
      1. Run: test -f rust-sdk/README.md
      2. Run: wc -l rust-sdk/README.md
      3. Assert line count between 50 and 250 (comprehensive but concise)
      4. Run: rg "no_run" rust-sdk/README.md
      5. Assert at least 2 matches (code examples use no_run)
      6. Run: rg "ignore" rust-sdk/README.md
      7. Assert 0 matches (no ignore blocks)
    Expected Result: README exists with proper code blocks
    Failure Indicators: File missing, too short, uses ignore, no code examples
    Evidence: .sisyphus/evidence/task-4-readme-structure.txt

  Scenario: CHANGELOG exists and follows format
    Tool: Bash (grep)
    Preconditions: CHANGELOG.md written
    Steps:
      1. Run: test -f rust-sdk/CHANGELOG.md
      2. Run: head -3 rust-sdk/CHANGELOG.md
      3. Assert first line contains "# Changelog"
      4. Run: rg "0.1.0" rust-sdk/CHANGELOG.md
      5. Assert match found
    Expected Result: CHANGELOG follows keep-a-changelog format with v0.1.0 entry
    Failure Indicators: File missing, wrong format, missing version
    Evidence: .sisyphus/evidence/task-4-changelog-structure.txt

  Scenario: README code examples compile
    Tool: Bash (cargo test)
    Preconditions: README.md written with no_run blocks
    Steps:
      1. Run: cargo test --doc 2>&1
      2. Assert exit code 0 (or only doc-test failures unrelated to README)
    Expected Result: README examples compile without errors
    Failure Indicators: Compilation errors in README code blocks
    Evidence: .sisyphus/evidence/task-4-readme-compiles.txt

  Scenario: README covers key sections
    Tool: Bash (grep)
    Preconditions: README.md written
    Steps:
      1. Run: rg -i "features|installation|quick.start|authentication" rust-sdk/README.md
      2. Assert matches found for all 4 section headings
    Expected Result: README has Features, Installation, Quick Start, Authentication sections
    Failure Indicators: Missing sections
    Evidence: .sisyphus/evidence/task-4-readme-sections.txt
  ```

  **Commit**: YES
  - Message: `docs: add README and CHANGELOG`
  - Files: `README.md`, `CHANGELOG.md`
  - Pre-commit: None (markdown files don't affect compilation)

- [ ] 5. Set Up GitHub Actions CI

  **What to do**:
  - Create `.github/workflows/ci.yml` with a single job (minimal first version)
  - **Trigger**: `push` to `main`/`master`, `pull_request` to `main`/`master`
  - **Runner**: `ubuntu-latest`
  - **Rust toolchain**: `stable`
  - **Steps**:
    1. Checkout repository
    2. Install Rust toolchain (stable)
    3. Cache `~/.cargo/registry`, `~/.cargo/git`, `target/`
    4. `cargo check --all-features`
    5. `cargo fmt --check --all`
    6. `cargo clippy --all-features -- -D warnings`
    7. `cargo test --all-features`
    8. `cargo doc --no-deps --all-features`
  - Keep it minimal — single job, single platform, single toolchain
  - Do NOT add matrix, cross-platform, MSRV, or semver-checks in this phase

  **Must NOT do**:
  - Do NOT add cross-platform matrix (linux/windows/macos) — over-engineering for v0.1.0
  - Do NOT add MSRV check — MSRV is implicit from `rust-version = "1.88"` in Cargo.toml
  - Do NOT add `cargo-semver-checks` — not meaningful before first release
  - Do NOT add deployment/publish automation
  - Do NOT add security audit (`cargo audit`)
  - Do NOT add benchmark or coverage steps

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard GitHub Actions setup for a Rust library, well-known pattern
  - **Skills**: []
  - **Skills Evaluated but Omitted**:
    - `brainstorming`: CI setup follows Rust community conventions, no creative decisions

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 5 (solo)
  - **Blocks**: F1-F4
  - **Blocked By**: Task 4 (all code/docs complete before CI)

  **References**:

  **Pattern References**:
  - `.githooks/pre-commit` — Current pre-commit hook (has the exact commands CI should run: fmt, clippy, test)
  - `Cargo.toml:39-43` — Feature flags that CI must test with `--all-features`
  - `Cargo.toml:5` — `rust-version = "1.88"` (MSRV, not enforced in CI yet)

  **External References**:
  - GitHub Actions for Rust: https://docs.github.com/en/actions/automating-builds-and-tests/building-and-testing-rust
  - Standard Rust CI workflow: uses `dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`

  **WHY Each Reference Matters**:
  - `.githooks/pre-commit` — Contains the exact commands and flags CI should mirror (clippy with `-D warnings`, fmt with `--check`)
  - `Cargo.toml:39-43` — CI must test with `--all-features` to cover rustls-tls, native-tls, and stream features

  **Acceptance Criteria**:

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: CI workflow file exists and is valid YAML
    Tool: Bash
    Preconditions: .github/workflows/ci.yml created
    Steps:
      1. Run: test -f rust-sdk/.github/workflows/ci.yml
      2. Run: python3 -c "import yaml; yaml.safe_load(open('rust-sdk/.github/workflows/ci.yml'))" && echo "VALID YAML" || echo "INVALID YAML"
    Expected Result: File exists and contains valid YAML
    Failure Indicators: File missing or YAML parse error
    Evidence: .sisyphus/evidence/task-5-ci-yaml-valid.txt

  Scenario: CI runs all required steps
    Tool: Bash (grep)
    Preconditions: .github/workflows/ci.yml created
    Steps:
      1. Run: rg "cargo fmt --check" rust-sdk/.github/workflows/ci.yml
      2. Run: rg "cargo clippy" rust-sdk/.github/workflows/ci.yml
      3. Run: rg "cargo test" rust-sdk/.github/workflows/ci.yml
      4. Run: rg "cargo doc" rust-sdk/.github/workflows/ci.yml
      5. Run: rg "cargo check" rust-sdk/.github/workflows/ci.yml
      6. Assert all 5 commands found in workflow
    Expected Result: All 5 cargo commands present in CI workflow
    Failure Indicators: Any command missing
    Evidence: .sisyphus/evidence/task-5-ci-steps.txt

  Scenario: CI triggers on push and PR
    Tool: Bash (grep)
    Preconditions: .github/workflows/ci.yml created
    Steps:
      1. Run: rg "push:" rust-sdk/.github/workflows/ci.yml
      2. Run: rg "pull_request:" rust-sdk/.github/workflows/ci.yml
      3. Assert both triggers present
    Expected Result: CI triggers on push and pull_request
    Failure Indicators: Missing trigger
    Evidence: .sisyphus/evidence/task-5-ci-triggers.txt

  Scenario: CI uses all-features flag
    Tool: Bash (grep)
    Preconditions: .github/workflows/ci.yml created
    Steps:
      1. Run: rg "all-features" rust-sdk/.github/workflows/ci.yml
      2. Assert at least one match found
    Expected Result: CI tests all features
    Failure Indicators: --all-features not used
    Evidence: .sisyphus/evidence/task-5-ci-all-features.txt
  ```

  **Commit**: YES
  - Message: `ci: add GitHub Actions workflow`
  - Files: `.github/workflows/ci.yml`
  - Pre-commit: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"`

---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Present consolidated results to user and get explicit "okay" before completing.
>
> **Do NOT auto-proceed after verification. Wait for user's explicit approval before marking work complete.**
> **Never mark F1-F4 as checked before getting user's okay.** Rejection or user feedback -> fix -> re-run -> present again -> wait for okay.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy --all-features -- -D warnings` + `cargo fmt --check`. Review all changed files for: `#[allow(dead_code)]` that should be removed, `#[allow(unused_imports)]` that should be removed, duplicate doc comments. Check that `cargo doc --no-deps` succeeds without warnings.
  Output: `Clippy [PASS/FAIL] | Fmt [PASS/FAIL] | Doc [PASS/FAIL] | Lint Allows [N removed] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Run `cargo publish --dry-run` and verify success. Check CI YAML validity. Verify README renders (check markdown structure). Run `cargo doc --no-deps --open` and spot-check that type pages show descriptive docs, not "X payload type." Verify `cargo metadata` has all required fields (keywords, categories, etc.). Test README code blocks compile with `cargo test --doc`.
  Output: `Publish Dry-Run [PASS/FAIL] | CI YAML [PASS/FAIL] | Doc Quality [N/N good] | README [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual diff (git log/diff). Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination: Task N touching Task M's files. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

| Task | Message | Key Files |
|------|---------|-----------|
| 1 | `docs: fix dual doc comments on type structs` | `src/types/*.rs` (26 files) |
| 2 | `refactor: clean up lint allows and fix warnings` | `src/api/mod.rs`, `src/types/mod.rs`, `src/api/admin.rs`, +7 files |
| 3 | `metadata: add crates.io fields, LICENSE, and crate-level docs` | `Cargo.toml`, `LICENSE`, `src/lib.rs` |
| 4 | `docs: add README and CHANGELOG` | `README.md`, `CHANGELOG.md` |
| 5 | `ci: add GitHub Actions workflow` | `.github/workflows/ci.yml` |

---

## Success Criteria

### Verification Commands
```bash
cargo clippy --all-features -- -D warnings  # Expected: exit 0, zero warnings
cargo fmt --check                           # Expected: exit 0
cargo doc --no-deps                          # Expected: exit 0, no doc warnings
cargo test                                  # Expected: all tests pass
cargo publish --dry-run                      # Expected: success
```

### Final Checklist
- [ ] All "Must Have" present
- [ ] All "Must NOT Have" absent
- [ ] `cargo publish --dry-run` succeeds
- [ ] CI workflow is valid and runs expected steps
- [ ] README.md has valid code examples
- [ ] CHANGELOG.md documents v0.1.0
- [ ] LICENSE file exists
- [ ] Zero "payload type." remnants in type files
