// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for pull requests, reviews, and changed files.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::label::Label;
use super::milestone::Milestone;
use super::serde_helpers::{null_to_default, nullable_rfc3339};
use super::team::Team;
use super::user::User;
use crate::types::enums::{ReviewStateType, StateType};

// ── pull.go ─────────────────────────────────────────────────────

/// `PRBranchInfo` information about a branch
#[derive(Debug, Clone, Serialize, Deserialize)]
/// `PRBranch` Info payload type.
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

/// `PRBranchInfoRepo` repository info embedded in PR branch info
#[derive(Debug, Clone, Serialize, Deserialize)]
/// `PRBranch` Info Repo payload type.
pub struct PRBranchInfoRepo {
    pub id: i64,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
}

/// `PullRequest` represents a pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pull Request payload type.
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
    #[serde(default, deserialize_with = "null_to_default")]
    pub labels: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Box<Milestone>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<Box<User>>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub assignees: Vec<User>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub requested_reviewers: Vec<User>,
    #[serde(default, deserialize_with = "null_to_default")]
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

/// `ChangedFile` is a changed file in a diff
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Changed File payload type.
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

/// `PullReview` represents a pull request review
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pull Review payload type.
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

/// `PullReviewComment` represents a comment on a pull request review
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pull Review Comment payload type.
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
    pub line_num: i64,
    #[serde(rename = "original_position")]
    pub old_line_num: i64,

    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub html_pull_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_time() -> OffsetDateTime {
        OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
            time::Time::from_hms(10, 0, 0).unwrap(),
        )
    }

    fn test_pr_branch_info() -> PRBranchInfo {
        PRBranchInfo {
            name: "owner:main".to_string(),
            ref_field: "main".to_string(),
            sha: "abc123".to_string(),
            repo_id: 1,
            repository: None,
        }
    }

    fn test_pull_request() -> PullRequest {
        PullRequest {
            id: 1,
            url: "https://gitea.example.com/api/v1/repos/test/repo/pulls/1".to_string(),
            index: 1,
            poster: None,
            title: "Fix bug".to_string(),
            body: "This fixes the bug.".to_string(),
            labels: vec![],
            milestone: None,
            assignee: None,
            assignees: vec![],
            requested_reviewers: vec![],
            requested_reviewers_teams: vec![],
            state: StateType::Open,
            draft: false,
            is_locked: false,
            comments: 2,
            review_comments: 1,
            html_url: "https://gitea.example.com/test/repo/pulls/1".to_string(),
            diff_url: "https://gitea.example.com/test/repo/pulls/1.diff".to_string(),
            patch_url: "https://gitea.example.com/test/repo/pulls/1.patch".to_string(),
            mergeable: true,
            has_merged: false,
            merged: None,
            merged_commit_id: None,
            merged_by: None,
            allow_maintainer_edit: false,
            base: None,
            head: None,
            merge_base: String::new(),
            deadline: None,
            created: test_time(),
            updated: test_time(),
            closed: None,
            additions: None,
            deletions: None,
            changed_files: None,
            pin_order: 0,
        }
    }

    #[test]
    fn test_pr_branch_info_round_trip() {
        let original = test_pr_branch_info();
        let json = serde_json::to_string(&original).unwrap();
        let restored: PRBranchInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.sha, original.sha);
        assert!(restored.repository.is_none());
    }

    #[test]
    fn test_pull_request_round_trip_minimal() {
        let original = test_pull_request();
        let json = serde_json::to_string(&original).unwrap();
        let restored: PullRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.title, original.title);
        assert!(restored.poster.is_none());
        assert!(restored.labels.is_empty());
        assert!(restored.assignees.is_empty());
        assert!(restored.requested_reviewers.is_empty());
        assert!(restored.additions.is_none());
    }

    #[test]
    fn test_pull_request_round_trip_with_branches() {
        let mut pr = test_pull_request();
        pr.base = Some(Box::new(test_pr_branch_info()));
        pr.head = Some(Box::new(test_pr_branch_info()));
        let json = serde_json::to_string(&pr).unwrap();
        let restored: PullRequest = serde_json::from_str(&json).unwrap();
        assert!(restored.base.is_some());
        assert!(restored.head.is_some());
    }

    #[test]
    fn test_pull_request_round_trip_merged() {
        let mut pr = test_pull_request();
        pr.state = StateType::Closed;
        pr.has_merged = true;
        pr.merged = Some(test_time());
        let json = serde_json::to_string(&pr).unwrap();
        let restored: PullRequest = serde_json::from_str(&json).unwrap();
        assert!(restored.has_merged);
        assert!(restored.merged.is_some());
    }

    #[test]
    fn test_changed_file_round_trip() {
        let original = ChangedFile {
            filename: "src/main.rs".to_string(),
            previous_filename: String::new(),
            status: "added".to_string(),
            additions: 10,
            deletions: 2,
            changes: 12,
            html_url: "https://example.com/file".to_string(),
            contents_url: "https://example.com/file/raw".to_string(),
            raw_url: "https://example.com/file/raw".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ChangedFile = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.filename, original.filename);
        assert_eq!(restored.additions, 10);
    }

    #[test]
    fn test_pull_review_round_trip() {
        let original = PullReview {
            id: 1,
            reviewer: None,
            reviewer_team: None,
            state: ReviewStateType::Approved,
            body: "Looks good".to_string(),
            commit_id: "abc123".to_string(),
            stale: false,
            official: false,
            dismissed: false,
            code_comments_count: 0,
            submitted: test_time(),
            html_url: "https://example.com/reviews/1".to_string(),
            html_pull_url: "https://example.com/pulls/1".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PullReview = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.state, ReviewStateType::Approved);
        assert!(restored.reviewer.is_none());
    }

    #[test]
    fn test_pull_review_comment_round_trip() {
        let original = PullReviewComment {
            id: 1,
            body: "nit: use snake_case".to_string(),
            reviewer: None,
            review_id: 10,
            resolver: None,
            created: test_time(),
            updated: test_time(),
            path: "src/main.rs".to_string(),
            commit_id: "abc123".to_string(),
            orig_commit_id: "abc123".to_string(),
            diff_hunk: String::new(),
            line_num: 42,
            old_line_num: 42,
            html_url: "https://example.com/comments/1".to_string(),
            html_pull_url: "https://example.com/pulls/1".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PullReviewComment = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.line_num, 42);
        assert!(restored.reviewer.is_none());
    }
}
