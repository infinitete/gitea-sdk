
## Task 12: Pull Requests API Module

### Patterns confirmed
- `time::serde::rfc3339` MUST be imported BEFORE `time::OffsetDateTime` (cargo fmt requirement)
- `Option::deserialize()` requires `use serde::Deserialize;` inside the module scope
- `StateType` does not implement `Default` — must provide manual `impl Default` with `StateType::All`
- For `get_status_code` pattern: merge returns 200=true, 405=merge conflict (return false), other non-2xx=error
- For `is_merged` pattern: 204=true, 404=false
- PR diff/patch uses `get_response` for raw bytes, NOT `get_parsed_response`
- `User` type referenced inside `api/` module must use `crate::types::User` (not just `User`) when defined in a separate sub-module
- Clippy warns on `is_some_and` chains — use `self.body.as_ref().is_some_and(...)` instead of multi-line

### Go SDK mapping quirks
- `PullReview.ReviewerTeam` is `*Team` in Go (not `*Organization` as task description suggested)
- `MergePullRequestOption.Do` maps to JSON `"Do"` (Go field name), not `"style"`
- `PullRequestDiffOptions` doesn't embed `ListOptions` in Go — it has its own `QueryEncode`
- Go SDK's `PRBranchInfo.Repo` is `*Repository` but we only need minimal fields (id, name, full_name, owner)
- `CreatePullReviewComment.OldLineNum` and `NewLineNum` are `int64` in Go (not `uint64`)
- `CreatePullReviewOptions.state` is `Option<ReviewStateType>` in Rust (Go validates but doesn't require)
- `EditPullRequestOption` fields are all `Option<T>` in Rust — Go sends zero values for unset fields
