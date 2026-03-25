// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Branch-related types (`repo_branch.go`, `repo_branch_protection.go`).

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::serde_helpers::null_to_default;

// ── repo_branch.go ──────────────────────────────────────────────

/// `PayloadUser` represents the author or committer of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload User payload type.
pub struct PayloadUser {
    /// Full name of the commit author
    pub name: String,
    pub email: String,
    pub username: String,
}

/// `PayloadCommitVerification` represents the GPG verification of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload Commit Verification payload type.
pub struct PayloadCommitVerification {
    pub verified: bool,
    pub reason: String,
    pub signature: String,
    pub payload: String,
}

/// `PayloadCommit` represents a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload Commit payload type.
pub struct PayloadCommit {
    /// sha1 hash of the commit
    pub id: String,
    pub message: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<PayloadUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<PayloadUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<PayloadCommitVerification>,
    #[serde(with = "rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(default, deserialize_with = "null_to_default")]
    pub added: Vec<String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub removed: Vec<String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub modified: Vec<String>,
}

/// Branch represents a repository branch
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Branch payload type.
pub struct Branch {
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<PayloadCommit>,
    pub protected: bool,
    #[serde(rename = "required_approvals")]
    pub required_approvals: i64,
    #[serde(rename = "enable_status_check")]
    pub enable_status_check: bool,
    #[serde(
        rename = "status_check_contexts",
        default,
        deserialize_with = "null_to_default"
    )]
    pub status_check_contexts: Vec<String>,
    #[serde(rename = "user_can_push")]
    pub user_can_push: bool,
    #[serde(rename = "user_can_merge")]
    pub user_can_merge: bool,
    #[serde(rename = "effective_branch_protection_name")]
    pub effective_branch_protection_name: String,
}

// ── repo_branch_protection.go ───────────────────────────────────

/// `BranchProtection` represents a branch protection for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Branch Protection payload type.
pub struct BranchProtection {
    #[serde(rename = "branch_name")]
    pub branch_name: String,
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    #[serde(rename = "enable_push")]
    pub enable_push: bool,
    #[serde(rename = "enable_push_whitelist")]
    pub enable_push_whitelist: bool,
    #[serde(
        rename = "push_whitelist_usernames",
        default,
        deserialize_with = "null_to_default"
    )]
    pub push_whitelist_usernames: Vec<String>,
    #[serde(
        rename = "push_whitelist_teams",
        default,
        deserialize_with = "null_to_default"
    )]
    pub push_whitelist_teams: Vec<String>,
    #[serde(rename = "push_whitelist_deploy_keys")]
    pub push_whitelist_deploy_keys: bool,
    #[serde(rename = "enable_merge_whitelist")]
    pub enable_merge_whitelist: bool,
    #[serde(
        rename = "merge_whitelist_usernames",
        default,
        deserialize_with = "null_to_default"
    )]
    pub merge_whitelist_usernames: Vec<String>,
    #[serde(
        rename = "merge_whitelist_teams",
        default,
        deserialize_with = "null_to_default"
    )]
    pub merge_whitelist_teams: Vec<String>,
    #[serde(rename = "enable_status_check")]
    pub enable_status_check: bool,
    #[serde(
        rename = "status_check_contexts",
        default,
        deserialize_with = "null_to_default"
    )]
    pub status_check_contexts: Vec<String>,
    #[serde(rename = "required_approvals")]
    pub required_approvals: i64,
    #[serde(rename = "enable_approvals_whitelist")]
    pub enable_approvals_whitelist: bool,
    #[serde(
        rename = "approvals_whitelist_username",
        default,
        deserialize_with = "null_to_default"
    )]
    pub approvals_whitelist_usernames: Vec<String>,
    #[serde(
        rename = "approvals_whitelist_teams",
        default,
        deserialize_with = "null_to_default"
    )]
    pub approvals_whitelist_teams: Vec<String>,
    #[serde(rename = "block_on_rejected_reviews")]
    pub block_on_rejected_reviews: bool,
    #[serde(rename = "block_on_official_review_requests")]
    pub block_on_official_review_requests: bool,
    #[serde(rename = "block_on_outdated_branch")]
    pub block_on_outdated_branch: bool,
    #[serde(rename = "dismiss_stale_approvals")]
    pub dismiss_stale_approvals: bool,
    #[serde(rename = "require_signed_commits")]
    pub require_signed_commits: bool,
    #[serde(rename = "protected_file_patterns")]
    pub protected_file_patterns: String,
    #[serde(rename = "unprotected_file_patterns")]
    pub unprotected_file_patterns: String,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    fn test_time() -> OffsetDateTime {
        OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
            time::Time::from_hms(10, 0, 0).unwrap(),
        )
    }

    #[test]
    fn test_branch_round_trip() {
        let original = Branch {
            name: "main".to_string(),
            commit: None,
            protected: false,
            required_approvals: 0,
            enable_status_check: false,
            status_check_contexts: vec![],
            user_can_push: true,
            user_can_merge: true,
            effective_branch_protection_name: String::new(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Branch = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, "main");
        assert!(restored.status_check_contexts.is_empty());
    }

    #[test]
    fn test_payload_user_round_trip() {
        let original = PayloadUser {
            name: "Test".to_string(),
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PayloadUser = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.username, "testuser");
    }

    #[test]
    fn test_branch_protection_round_trip() {
        let original = BranchProtection {
            branch_name: "main".to_string(),
            rule_name: "main".to_string(),
            enable_push: false,
            enable_push_whitelist: false,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: false,
            enable_merge_whitelist: false,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: false,
            status_check_contexts: vec![],
            required_approvals: 1,
            enable_approvals_whitelist: false,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: false,
            block_on_official_review_requests: false,
            block_on_outdated_branch: false,
            dismiss_stale_approvals: false,
            require_signed_commits: false,
            protected_file_patterns: String::new(),
            unprotected_file_patterns: String::new(),
            created: test_time(),
            updated: test_time(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: BranchProtection = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.branch_name, "main");
        assert_eq!(restored.required_approvals, 1);
    }
}
