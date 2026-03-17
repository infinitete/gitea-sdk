// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::label::Label;
use super::milestone::Milestone;
use super::serde_helpers::nullable_rfc3339;
use super::user::User;
use crate::types::enums::{IssueFormElementType, StateType};

// ── issue.go ─────────────────────────────────────────────────────

/// PullRequestMeta PR info if an issue is a PR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestMeta {
    #[serde(rename = "merged")]
    pub has_merged: bool,
    #[serde(
        rename = "merged_at",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub merged: Option<OffsetDateTime>,
}

/// RepositoryMeta basic repository information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryMeta {
    pub id: i64,
    pub name: String,
    pub owner: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
}

/// Issue represents an issue in a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: i64,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "number")]
    pub index: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(default, rename = "original_author")]
    pub original_author: String,
    #[serde(default, rename = "original_author_id")]
    pub original_author_id: i64,
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub ref_field: String,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(default)]
    pub assignees: Vec<User>,
    /// Whether the issue is open or closed
    pub state: StateType,
    #[serde(rename = "is_locked")]
    pub is_locked: bool,
    pub comments: i64,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,
    #[serde(
        rename = "closed_at",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub closed: Option<OffsetDateTime>,
    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pull_request: Option<PullRequestMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<RepositoryMeta>,
}

// ── issue_ext.go ─────────────────────────────────────────────────

/// IssueBlockedBy represents an issue that blocks another issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueBlockedBy {
    pub index: i64,
    pub title: String,
    pub state: String,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

/// IssueMeta represents issue reference for blocking/dependency operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueMeta {
    pub index: i64,
}

// ── issue_stopwatch.go ───────────────────────────────────────────

/// StopWatch represents a running stopwatch of an issue / pr
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopWatch {
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
    pub seconds: i64,
    pub duration: String,
    #[serde(rename = "issue_index")]
    pub issue_index: i64,
    #[serde(rename = "issue_title")]
    pub issue_title: String,
    #[serde(rename = "repo_owner_name")]
    pub repo_owner_name: String,
    #[serde(rename = "repo_name")]
    pub repo_name: String,
}

// ── issue_tracked_time.go ────────────────────────────────────────

/// TrackedTime worked time for an issue / pr
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedTime {
    pub id: i64,
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
    /// Time in seconds
    pub time: i64,
    #[serde(default, rename = "user_id")]
    pub user_id: i64,
    #[serde(default, rename = "user_name")]
    pub user_name: String,
    #[serde(default, rename = "issue_id")]
    pub issue_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<Box<Issue>>,
}

// ── issue_timeline.go ────────────────────────────────────────────

/// TimelineComment represents a timeline comment on an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineComment {
    pub id: i64,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub pr_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(default, rename = "original_author")]
    pub original_author: String,
    #[serde(default, rename = "original_author_id")]
    pub original_author_id: i64,
    #[serde(default)]
    pub body: String,
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated: OffsetDateTime,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub label: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(
        default,
        rename = "old_milestone",
        skip_serializing_if = "Option::is_none"
    )]
    pub old_milestone: Option<Milestone>,
    #[serde(default, rename = "new_title")]
    pub new_title: String,
    #[serde(default, rename = "old_title")]
    pub old_title: String,
}

// ── issue_subscription.go ────────────────────────────────────────

/// WatchInfo represents the subscription state of an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WatchInfo {
    #[serde(default)]
    pub subscribed: bool,
    #[serde(default)]
    pub watching: bool,
    #[serde(default)]
    pub ignored: bool,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "created_at", default, with = "nullable_rfc3339")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(default, rename = "url")]
    pub url: String,
    #[serde(default, rename = "repository_url")]
    pub repository_url: String,
}

// ── issue_template.go ────────────────────────────────────────────

/// IssueTemplate provides metadata and content on an issue template.
/// There are two types of issue templates: .Markdown- and .Form-based.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueTemplate {
    pub name: String,
    pub about: String,
    #[serde(rename = "file_name")]
    pub filename: String,
    pub title: String,
    pub labels: Vec<String>,
    #[serde(default)]
    pub r#ref: String,
    /// If non-nil, this is a form-based template
    #[serde(default)]
    pub form: Vec<IssueFormElement>,
    /// Should only be used when .Form is nil.
    #[serde(default, rename = "content")]
    pub markdown_content: String,
}

/// IssueFormElement describes a part of a IssueTemplate form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueFormElement {
    pub id: String,
    pub r#type: IssueFormElementType,
    pub attributes: IssueFormElementAttributes,
    #[serde(default)]
    pub validations: IssueFormElementValidations,
}

/// IssueFormElementAttributes contains the combined set of attributes available on all element types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueFormElementAttributes {
    /// A brief description of the expected user input
    pub label: String,
    /// Options for dropdown and checkboxes
    #[serde(default)]
    pub options: Vec<String>,
    /// Pre-filled value for markdown, textarea, input
    #[serde(default)]
    pub value: String,
    /// Description for textarea, input, dropdown, checkboxes
    #[serde(default)]
    pub description: String,
    /// Placeholder for textarea, input
    #[serde(default)]
    pub placeholder: String,
    /// Syntax highlighting language for textarea
    #[serde(default, rename = "render")]
    pub syntax_highlighting: String,
    /// Multiple selection for dropdown
    #[serde(default)]
    pub multiple: bool,
}

/// IssueFormElementValidations contains the combined set of validations available on all element types.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IssueFormElementValidations {
    #[serde(default)]
    pub required: bool,
    #[serde(default, rename = "is_number")]
    pub is_number: bool,
    #[serde(default)]
    pub regex: String,
}
