// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::label::Label;
use super::milestone::Milestone;
use super::serde_helpers::nullable_rfc3339;
use super::team::Team;
use super::user::User;
use crate::types::enums::{ReviewStateType, StateType};

// ── pull.go ─────────────────────────────────────────────────────

/// PRBranchInfo information about a branch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRBranchInfo {
    #[serde(rename = "label")]
    pub name: String,
    #[serde(rename = "ref")]
    pub ref_field: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<PRBranchInfoRepo>>,
}

/// PRBranchInfoRepo repository info embedded in PR branch info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PRBranchInfoRepo {
    pub id: i64,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
}

/// PullRequest represents a pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: i64,
    pub url: String,
    #[serde(rename = "number")]
    pub index: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<Box<User>>,
    pub title: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Box<Milestone>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<User>>,
    #[serde(default)]
    pub assignees: Vec<User>,
    #[serde(default)]
    pub requested_reviewers: Vec<User>,
    #[serde(default)]
    pub requested_reviewers_teams: Vec<Team>,
    pub state: StateType,
    #[serde(default)]
    pub draft: bool,
    #[serde(rename = "is_locked")]
    pub is_locked: bool,
    pub comments: i64,
    #[serde(default, rename = "review_comments")]
    pub review_comments: i64,

    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "diff_url")]
    pub diff_url: String,
    #[serde(rename = "patch_url")]
    pub patch_url: String,

    #[serde(default)]
    pub mergeable: bool,
    #[serde(rename = "merged")]
    pub has_merged: bool,
    #[serde(
        rename = "merged_at",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub merged: Option<OffsetDateTime>,
    #[serde(
        rename = "merge_commit_sha",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub merged_commit_id: Option<String>,
    #[serde(rename = "merged_by", default, skip_serializing_if = "Option::is_none")]
    pub merged_by: Option<Box<User>>,
    #[serde(rename = "allow_maintainer_edit")]
    pub allow_maintainer_edit: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<Box<PRBranchInfo>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head: Option<Box<PRBranchInfo>>,
    #[serde(default, rename = "merge_base")]
    pub merge_base: String,

    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
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

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletions: Option<i64>,
    #[serde(
        default,
        rename = "changed_files",
        skip_serializing_if = "Option::is_none"
    )]
    pub changed_files: Option<i64>,
    #[serde(default, rename = "pin_order")]
    pub pin_order: i32,
}

/// ChangedFile is a changed file in a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangedFile {
    pub filename: String,
    #[serde(default, rename = "previous_filename")]
    pub previous_filename: String,
    pub status: String,
    pub additions: i64,
    pub deletions: i64,
    pub changes: i64,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "raw_url")]
    pub raw_url: String,
}

// ── pull_review.go ───────────────────────────────────────────────

/// PullReview represents a pull request review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullReview {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<Box<User>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_team: Option<Box<Team>>,
    pub state: ReviewStateType,
    #[serde(default)]
    pub body: String,
    #[serde(rename = "commit_id")]
    pub commit_id: String,
    /// Stale indicates if the pull has changed since the review
    #[serde(default)]
    pub stale: bool,
    /// Official indicates if the review counts towards the required approval limit
    #[serde(default)]
    pub official: bool,
    #[serde(default)]
    pub dismissed: bool,
    #[serde(default, rename = "comments_count")]
    pub code_comments_count: i64,
    #[serde(rename = "submitted_at", with = "rfc3339")]
    pub submitted: OffsetDateTime,

    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub html_pull_url: String,
}

/// PullReviewComment represents a comment on a pull request review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullReviewComment {
    pub id: i64,
    #[serde(default)]
    pub body: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer: Option<Box<User>>,
    #[serde(rename = "pull_request_review_id")]
    pub review_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Box<User>>,

    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,

    pub path: String,
    #[serde(rename = "commit_id")]
    pub commit_id: String,
    #[serde(rename = "original_commit_id")]
    pub orig_commit_id: String,
    #[serde(default)]
    pub diff_hunk: String,
    #[serde(rename = "position")]
    pub line_num: u64,
    #[serde(rename = "original_position")]
    pub old_line_num: u64,

    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub html_pull_url: String,
}
