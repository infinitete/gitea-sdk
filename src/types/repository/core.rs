// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Core repository types (repo.go, repo_collaborator.go, repo_mirror.go,
//! repo_action_variable.go, git_hook.go).

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::enums::{MergeStyle, ProjectsMode};
use crate::types::serde_helpers::null_to_default;
use crate::types::team::Team;
use crate::types::user::User;

// ── repo.go ─────────────────────────────────────────────────────

/// Permission represents a set of permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Permission payload type.
pub struct Permission {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

/// InternalTracker represents settings for internal tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Internal Tracker payload type.
pub struct InternalTracker {
    /// Enable time tracking (Built-in issue tracker)
    #[serde(rename = "enable_time_tracker")]
    pub enable_time_tracker: bool,
    /// Let only contributors track time (Built-in issue tracker)
    #[serde(rename = "allow_only_contributors_to_track_time")]
    pub allow_only_contributors_to_track_time: bool,
    /// Enable dependencies for issues and pull requests (Built-in issue tracker)
    #[serde(rename = "enable_issue_dependencies")]
    pub enable_issue_dependencies: bool,
}

/// ExternalTracker represents settings for external tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
/// External Tracker payload type.
pub struct ExternalTracker {
    /// URL of external issue tracker
    #[serde(rename = "external_tracker_url")]
    pub external_tracker_url: String,
    /// External Issue Tracker URL Format. Use the placeholders {user}, {repo} and {index}
    #[serde(rename = "external_tracker_format")]
    pub external_tracker_format: String,
    /// External Issue Tracker Number Format, either `numeric` or `alphanumeric`
    #[serde(rename = "external_tracker_style")]
    pub external_tracker_style: String,
}

/// ExternalWiki represents setting for external wiki
#[derive(Debug, Clone, Serialize, Deserialize)]
/// External Wiki payload type.
pub struct ExternalWiki {
    /// URL of external wiki
    #[serde(rename = "external_wiki_url")]
    pub external_wiki_url: String,
}

/// RepoTransfer represents a pending repository transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Repo Transfer payload type.
pub struct RepoTransfer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doer: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<User>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub teams: Vec<Team>,
}

/// Repository represents a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Repository payload type.
pub struct Repository {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub description: String,
    pub empty: bool,
    pub private: bool,
    pub fork: bool,
    pub template: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<Repository>>,
    pub mirror: bool,
    pub size: i32,
    pub language: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub url: String,
    pub link: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "original_url")]
    pub original_url: String,
    pub website: String,
    #[serde(rename = "stars_count")]
    pub stars: i32,
    #[serde(rename = "forks_count")]
    pub forks: i32,
    #[serde(rename = "watchers_count")]
    pub watchers: i32,
    #[serde(rename = "open_issues_count")]
    pub open_issues: i32,
    #[serde(rename = "open_pr_counter")]
    pub open_pulls: i32,
    #[serde(rename = "release_counter")]
    pub releases: i32,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    pub archived: bool,
    #[serde(rename = "archived_at", with = "rfc3339")]
    pub archived_at: OffsetDateTime,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permission>,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_code")]
    pub has_code: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_tracker: Option<InternalTracker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_tracker: Option<ExternalTracker>,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_wiki: Option<ExternalWiki>,
    #[serde(rename = "has_pull_requests")]
    pub has_pull_requests: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_releases: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_packages: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_actions: Option<bool>,
    #[serde(rename = "ignore_whitespace_conflicts")]
    pub ignore_whitespace_conflicts: bool,
    #[serde(rename = "allow_fast_forward_only_merge")]
    pub allow_fast_forward_only_merge: bool,
    #[serde(rename = "allow_merge_commits")]
    pub allow_merge: bool,
    #[serde(rename = "allow_rebase")]
    pub allow_rebase: bool,
    #[serde(rename = "allow_rebase_explicit")]
    pub allow_rebase_merge: bool,
    #[serde(rename = "allow_rebase_update")]
    pub allow_rebase_update: bool,
    #[serde(rename = "allow_squash_merge")]
    pub allow_squash: bool,
    #[serde(rename = "default_allow_maintainer_edit")]
    pub default_allow_maintainer_edit: bool,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub internal: bool,
    #[serde(rename = "mirror_interval")]
    pub mirror_interval: String,
    #[serde(
        rename = "mirror_updated",
        default,
        with = "crate::types::serde_helpers::nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_updated: Option<OffsetDateTime>,
    #[serde(rename = "default_merge_style")]
    pub default_merge_style: MergeStyle,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub projects_mode: Option<ProjectsMode>,
    #[serde(rename = "default_delete_branch_after_merge")]
    pub default_delete_branch_after_merge: bool,
    #[serde(rename = "object_format_name")]
    pub object_format_name: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub topics: Vec<String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub licenses: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_transfer: Option<RepoTransfer>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::VisibleType;
    use crate::types::user::User;

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
            language: "".to_string(),
            is_admin: false,
            last_login: None,
            created: None,
            restricted: false,
            is_active: true,
            prohibit_login: false,
            location: "".to_string(),
            website: "".to_string(),
            description: "".to_string(),
            visibility: VisibleType::Public,
            follower_count: 0,
            following_count: 0,
            starred_repo_count: 0,
        }
    }

    fn test_repository() -> Repository {
        Repository {
            id: 1,
            owner: None,
            name: "test-repo".to_string(),
            full_name: "owner/test-repo".to_string(),
            description: "A test repo".to_string(),
            empty: false,
            private: false,
            fork: false,
            template: false,
            parent: None,
            mirror: false,
            size: 1024,
            language: "Rust".to_string(),
            languages_url: "https://example.com/languages".to_string(),
            html_url: "https://example.com/owner/test-repo".to_string(),
            url: "https://example.com/api/v1/repos/owner/test-repo".to_string(),
            link: "https://example.com/owner/test-repo".to_string(),
            ssh_url: "git@example.com:owner/test-repo.git".to_string(),
            clone_url: "https://example.com/owner/test-repo.git".to_string(),
            original_url: String::new(),
            website: String::new(),
            stars: 10,
            forks: 2,
            watchers: 5,
            open_issues: 3,
            open_pulls: 1,
            releases: 4,
            default_branch: "main".to_string(),
            archived: false,
            archived_at: test_time(),
            created: test_time(),
            updated: test_time(),
            permissions: None,
            has_issues: true,
            has_code: true,
            internal_tracker: None,
            external_tracker: None,
            has_wiki: true,
            external_wiki: None,
            has_pull_requests: true,
            has_projects: true,
            has_releases: None,
            has_packages: None,
            has_actions: None,
            ignore_whitespace_conflicts: false,
            allow_fast_forward_only_merge: false,
            allow_merge: true,
            allow_rebase: true,
            allow_rebase_merge: true,
            allow_rebase_update: false,
            allow_squash: true,
            default_allow_maintainer_edit: false,
            avatar_url: String::new(),
            internal: false,
            mirror_interval: String::new(),
            mirror_updated: None,
            default_merge_style: MergeStyle::Merge,
            projects_mode: None,
            default_delete_branch_after_merge: false,
            object_format_name: "sha1".to_string(),
            topics: vec![],
            licenses: vec![],
            repo_transfer: None,
        }
    }

    #[test]
    fn test_permission_round_trip() {
        let original = Permission {
            admin: false,
            push: true,
            pull: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Permission = serde_json::from_str(&json).unwrap();
        assert!(restored.pull);
        assert!(!restored.admin);
    }

    #[test]
    fn test_internal_tracker_round_trip() {
        let original = InternalTracker {
            enable_time_tracker: true,
            allow_only_contributors_to_track_time: false,
            enable_issue_dependencies: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: InternalTracker = serde_json::from_str(&json).unwrap();
        assert!(restored.enable_time_tracker);
        assert!(restored.enable_issue_dependencies);
    }

    #[test]
    fn test_external_tracker_round_trip() {
        let original = ExternalTracker {
            external_tracker_url: "https://tracker.example.com".to_string(),
            external_tracker_format: "https://tracker.example.com/{index}".to_string(),
            external_tracker_style: "numeric".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ExternalTracker = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.external_tracker_style, "numeric");
    }

    #[test]
    fn test_external_wiki_round_trip() {
        let original = ExternalWiki {
            external_wiki_url: "https://wiki.example.com".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ExternalWiki = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.external_wiki_url, original.external_wiki_url);
    }

    #[test]
    fn test_repo_transfer_round_trip() {
        let original = RepoTransfer {
            doer: None,
            recipient: None,
            teams: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: RepoTransfer = serde_json::from_str(&json).unwrap();
        assert!(restored.teams.is_empty());
        assert!(restored.doer.is_none());
    }

    #[test]
    fn test_repository_round_trip() {
        let original = test_repository();
        let json = serde_json::to_string(&original).unwrap();
        let restored: Repository = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.full_name, original.full_name);
        assert!(!restored.private);
        assert!(restored.topics.is_empty());
        assert!(restored.licenses.is_empty());
        assert!(restored.repo_transfer.is_none());
    }

    #[test]
    fn test_repository_with_owner() {
        let mut repo = test_repository();
        repo.owner = Some(test_user());
        let json = serde_json::to_string(&repo).unwrap();
        let restored: Repository = serde_json::from_str(&json).unwrap();
        assert!(restored.owner.is_some());
        assert_eq!(restored.owner.unwrap().user_name, "testuser");
    }

    #[test]
    fn test_repository_with_permissions() {
        let mut repo = test_repository();
        repo.permissions = Some(Permission {
            admin: false,
            push: true,
            pull: true,
        });
        let json = serde_json::to_string(&repo).unwrap();
        let restored: Repository = serde_json::from_str(&json).unwrap();
        assert!(restored.permissions.is_some());
        assert!(restored.permissions.unwrap().push);
    }

    #[test]
    fn test_repository_with_topics() {
        let mut repo = test_repository();
        repo.topics = vec!["rust".to_string(), "gitea".to_string()];
        repo.licenses = vec!["MIT".to_string()];
        let json = serde_json::to_string(&repo).unwrap();
        let restored: Repository = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.topics.len(), 2);
        assert_eq!(restored.licenses.len(), 1);
    }
}
