// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Core issue types: Issue, `PullRequestMeta`, `RepositoryMeta`, `IssueBlockedBy`, `IssueMeta`.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::enums::StateType;
use crate::types::label::Label;
use crate::types::milestone::Milestone;
use crate::types::serde_helpers::{null_to_default, nullable_rfc3339};
use crate::types::user::User;

// ── issue.go ─────────────────────────────────────────────────────

/// `PullRequestMeta` PR info if an issue is a PR
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

/// `RepositoryMeta` basic repository information
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
    #[serde(default, deserialize_with = "null_to_default")]
    pub labels: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(default, deserialize_with = "null_to_default")]
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

/// `IssueBlockedBy` represents an issue that blocks another issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Blocked By payload type.
pub struct IssueBlockedBy {
    pub index: i64,
    pub title: String,
    pub state: String,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

/// `IssueMeta` represents issue reference for blocking/dependency operations
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Meta payload type.
pub struct IssueMeta {
    pub index: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::StateType;

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
    fn test_issue_deserialize_null_assignees() {
        let json = r#"{
            "id":5,
            "url":"http://example.com/api/v1/repos/test/repo/issues/1",
            "html_url":"http://example.com/test/repo/issues/1",
            "number":1,
            "user":null,
            "original_author":"",
            "original_author_id":0,
            "title":"debug issue",
            "body":"debug body",
            "ref":"",
            "labels":[],
            "milestone":null,
            "assignees":null,
            "state":"open",
            "is_locked":false,
            "comments":0,
            "created_at":"2026-03-18T11:48:01+08:00",
            "updated_at":"2026-03-18T11:48:01+08:00",
            "closed_at":null,
            "due_date":null,
            "pull_request":null,
            "repository":{
                "id":29,
                "name":"debug-issue-json",
                "owner":"gitea_tester",
                "full_name":"gitea_tester/debug-issue-json"
            }
        }"#;

        let restored: Issue = serde_json::from_str(json).unwrap();
        assert_eq!(restored.index, 1);
        assert!(restored.assignees.is_empty());
        assert!(restored.repository.is_some());
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
}
