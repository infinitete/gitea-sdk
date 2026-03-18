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
/// Pull Request Meta payload type.
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
/// Repository Meta payload type.
pub struct RepositoryMeta {
    pub id: i64,
    pub name: String,
    pub owner: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
}

/// Issue represents an issue in a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue payload type.
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
/// Issue Blocked By payload type.
pub struct IssueBlockedBy {
    pub index: i64,
    pub title: String,
    pub state: String,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

/// IssueMeta represents issue reference for blocking/dependency operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Meta payload type.
pub struct IssueMeta {
    pub index: i64,
}

// ── issue_stopwatch.go ───────────────────────────────────────────

/// StopWatch represents a running stopwatch of an issue / pr
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Stop Watch payload type.
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
/// Tracked Time payload type.
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
/// Timeline Comment payload type.
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
/// Watch Info payload type.
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
/// Issue Template payload type.
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
/// Issue Form Element payload type.
pub struct IssueFormElement {
    pub id: String,
    pub r#type: IssueFormElementType,
    pub attributes: IssueFormElementAttributes,
    #[serde(default)]
    pub validations: IssueFormElementValidations,
}

/// IssueFormElementAttributes contains the combined set of attributes available on all element types.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Form Element Attributes payload type.
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
/// Issue Form Element Validations payload type.
pub struct IssueFormElementValidations {
    #[serde(default)]
    pub required: bool,
    #[serde(default, rename = "is_number")]
    pub is_number: bool,
    #[serde(default)]
    pub regex: String,
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

    fn test_user() -> User {
        User {
            id: 1,
            user_name: "testuser".to_string(),
            login_name: "".to_string(),
            source_id: 0,
            full_name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            html_url: "https://gitea.example.com/testuser".to_string(),
            language: "en-US".to_string(),
            is_admin: false,
            last_login: None,
            created: None,
            restricted: false,
            is_active: true,
            prohibit_login: false,
            location: "".to_string(),
            website: "".to_string(),
            description: "".to_string(),
            visibility: crate::types::enums::VisibleType::Public,
            follower_count: 0,
            following_count: 0,
            starred_repo_count: 0,
        }
    }

    fn test_issue() -> Issue {
        Issue {
            id: 1,
            url: "https://gitea.example.com/api/v1/repos/test/repo/issues/1".to_string(),
            html_url: "https://gitea.example.com/test/repo/issues/1".to_string(),
            index: 1,
            user: None,
            original_author: String::new(),
            original_author_id: 0,
            title: "Bug fix".to_string(),
            body: "Something broke".to_string(),
            ref_field: String::new(),
            labels: vec![],
            milestone: None,
            assignees: vec![],
            state: StateType::Open,
            is_locked: false,
            comments: 0,
            created: test_time(),
            updated: test_time(),
            closed: None,
            deadline: None,
            pull_request: None,
            repository: None,
        }
    }

    #[test]
    fn test_pull_request_meta_round_trip() {
        let original = PullRequestMeta {
            has_merged: false,
            merged: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PullRequestMeta = serde_json::from_str(&json).unwrap();
        assert!(!restored.has_merged);
        assert!(restored.merged.is_none());
    }

    #[test]
    fn test_pull_request_meta_with_merge() {
        let original = PullRequestMeta {
            has_merged: true,
            merged: Some(test_time()),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PullRequestMeta = serde_json::from_str(&json).unwrap();
        assert!(restored.has_merged);
        assert!(restored.merged.is_some());
    }

    #[test]
    fn test_repository_meta_round_trip() {
        let original = RepositoryMeta {
            id: 1,
            name: "test-repo".to_string(),
            owner: "testowner".to_string(),
            full_name: "testowner/test-repo".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: RepositoryMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.full_name, original.full_name);
    }

    #[test]
    fn test_issue_round_trip_minimal() {
        let original = test_issue();
        let json = serde_json::to_string(&original).unwrap();
        let restored: Issue = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.state, original.state);
        assert!(restored.user.is_none());
        assert!(restored.labels.is_empty());
        assert!(restored.assignees.is_empty());
    }

    #[test]
    fn test_issue_round_trip_with_user() {
        let mut issue = test_issue();
        issue.user = Some(test_user());
        let json = serde_json::to_string(&issue).unwrap();
        let restored: Issue = serde_json::from_str(&json).unwrap();
        assert!(restored.user.is_some());
        assert_eq!(restored.user.unwrap().user_name, "testuser");
    }

    #[test]
    fn test_issue_round_trip_with_closed() {
        let mut issue = test_issue();
        issue.state = StateType::Closed;
        issue.closed = Some(test_time());
        let json = serde_json::to_string(&issue).unwrap();
        let restored: Issue = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.state, StateType::Closed);
        assert!(restored.closed.is_some());
    }

    #[test]
    fn test_issue_blocked_by_round_trip() {
        let original = IssueBlockedBy {
            index: 2,
            title: "Blocking issue".to_string(),
            state: "open".to_string(),
            created_at: test_time(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: IssueBlockedBy = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.index, original.index);
        assert_eq!(restored.title, original.title);
    }

    #[test]
    fn test_issue_meta_round_trip() {
        let original = IssueMeta { index: 1 };
        let json = serde_json::to_string(&original).unwrap();
        let restored: IssueMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.index, 1);
    }

    #[test]
    fn test_stop_watch_round_trip() {
        let original = StopWatch {
            created: test_time(),
            seconds: 3600,
            duration: "1h0m0s".to_string(),
            issue_index: 1,
            issue_title: "Fix bug".to_string(),
            repo_owner_name: "owner".to_string(),
            repo_name: "repo".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: StopWatch = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.seconds, 3600);
        assert_eq!(restored.issue_index, 1);
    }

    #[test]
    fn test_tracked_time_round_trip() {
        let original = TrackedTime {
            id: 1,
            created: test_time(),
            time: 1800,
            user_id: 0,
            user_name: String::new(),
            issue_id: 0,
            issue: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: TrackedTime = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.time, 1800);
        assert!(restored.issue.is_none());
    }

    #[test]
    fn test_timeline_comment_round_trip() {
        let original = TimelineComment {
            id: 1,
            html_url: "https://example.com".to_string(),
            pr_url: String::new(),
            issue_url: "https://example.com".to_string(),
            user: None,
            original_author: String::new(),
            original_author_id: 0,
            body: "comment".to_string(),
            created: test_time(),
            updated: test_time(),
            r#type: "comment".to_string(),
            label: vec![],
            milestone: None,
            old_milestone: None,
            new_title: String::new(),
            old_title: String::new(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: TimelineComment = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert!(restored.user.is_none());
        assert!(restored.label.is_empty());
    }

    #[test]
    fn test_watch_info_round_trip() {
        let original = WatchInfo {
            subscribed: true,
            watching: true,
            ignored: false,
            reason: String::new(),
            created_at: None,
            url: String::new(),
            repository_url: String::new(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: WatchInfo = serde_json::from_str(&json).unwrap();
        assert!(restored.subscribed);
        assert!(restored.watching);
    }

    #[test]
    fn test_issue_template_round_trip() {
        let original = IssueTemplate {
            name: "Bug Report".to_string(),
            about: "File a bug".to_string(),
            filename: "bug_report.md".to_string(),
            title: "Bug: ".to_string(),
            labels: vec!["bug".to_string()],
            r#ref: String::new(),
            form: vec![],
            markdown_content: "Describe the bug...".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: IssueTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.labels.len(), 1);
        assert!(restored.form.is_empty());
    }

    #[test]
    fn test_issue_form_element_round_trip() {
        let original = IssueFormElement {
            id: "title".to_string(),
            r#type: IssueFormElementType::Input,
            attributes: IssueFormElementAttributes {
                label: "Title".to_string(),
                options: vec![],
                value: String::new(),
                description: "Bug title".to_string(),
                placeholder: String::new(),
                syntax_highlighting: String::new(),
                multiple: false,
            },
            validations: IssueFormElementValidations {
                required: true,
                is_number: false,
                regex: String::new(),
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: IssueFormElement = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, "title");
        assert!(restored.validations.required);
    }
}
