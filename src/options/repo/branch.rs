// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── repo_branch.go ──────────────────────────────────────────────

/// `ListRepoBranchesOptions` options for listing a repository's branches
#[derive(Debug, Clone, Default)]
/// Options for List Repo Branches Option.
pub struct ListRepoBranchesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoBranchesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// `CreateBranchOption` options when creating a branch in a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Branch Option.
pub struct CreateBranchOption {
    #[serde(rename = "new_branch_name")]
    pub branch_name: String,
    #[serde(rename = "old_branch_name")]
    pub old_branch_name: String,
}

impl CreateBranchOption {
    /// Validate this `CreateBranchOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.branch_name.is_empty() {
            return Err(crate::Error::Validation("BranchName is empty".to_string()));
        }
        if self.branch_name.len() > 100 {
            return Err(crate::Error::Validation("BranchName too long".to_string()));
        }
        if self.old_branch_name.len() > 100 {
            return Err(crate::Error::Validation(
                "OldBranchName too long".to_string(),
            ));
        }
        Ok(())
    }
}

/// `UpdateRepoBranchOption` options when renaming a branch
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Update Repo Branch Option.
pub struct UpdateRepoBranchOption {
    pub name: String,
}

impl UpdateRepoBranchOption {
    /// Validate this `UpdateRepoBranchOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::Validation("empty Name field".to_string()));
        }
        Ok(())
    }
}

// ── repo_branch_protection.go ───────────────────────────────────

/// `ListBranchProtectionsOptions` list branch protection options
#[derive(Debug, Clone, Default)]
/// Options for List Branch Protections Option.
pub struct ListBranchProtectionsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListBranchProtectionsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// `CreateBranchProtectionOption` options for creating a branch protection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Branch Protection Option.
pub struct CreateBranchProtectionOption {
    #[serde(rename = "branch_name")]
    pub branch_name: String,
    #[serde(rename = "rule_name")]
    pub rule_name: String,
    #[serde(rename = "enable_push")]
    pub enable_push: bool,
    #[serde(rename = "enable_push_whitelist")]
    pub enable_push_whitelist: bool,
    #[serde(rename = "push_whitelist_usernames", default)]
    pub push_whitelist_usernames: Vec<String>,
    #[serde(rename = "push_whitelist_teams", default)]
    pub push_whitelist_teams: Vec<String>,
    #[serde(rename = "push_whitelist_deploy_keys")]
    pub push_whitelist_deploy_keys: bool,
    #[serde(rename = "enable_merge_whitelist")]
    pub enable_merge_whitelist: bool,
    #[serde(rename = "merge_whitelist_usernames", default)]
    pub merge_whitelist_usernames: Vec<String>,
    #[serde(rename = "merge_whitelist_teams", default)]
    pub merge_whitelist_teams: Vec<String>,
    #[serde(rename = "enable_status_check")]
    pub enable_status_check: bool,
    #[serde(rename = "status_check_contexts", default)]
    pub status_check_contexts: Vec<String>,
    #[serde(rename = "required_approvals")]
    pub required_approvals: i64,
    #[serde(rename = "enable_approvals_whitelist")]
    pub enable_approvals_whitelist: bool,
    #[serde(rename = "approvals_whitelist_username", default)]
    pub approvals_whitelist_usernames: Vec<String>,
    #[serde(rename = "approvals_whitelist_teams", default)]
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
}

/// `EditBranchProtectionOption` options for editing a branch protection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Branch Protection Option.
pub struct EditBranchProtectionOption {
    #[serde(
        rename = "enable_push",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_push: Option<bool>,
    #[serde(
        rename = "enable_push_whitelist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_push_whitelist: Option<bool>,
    #[serde(rename = "push_whitelist_usernames", default)]
    pub push_whitelist_usernames: Vec<String>,
    #[serde(rename = "push_whitelist_teams", default)]
    pub push_whitelist_teams: Vec<String>,
    #[serde(
        rename = "push_whitelist_deploy_keys",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub push_whitelist_deploy_keys: Option<bool>,
    #[serde(
        rename = "enable_merge_whitelist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_merge_whitelist: Option<bool>,
    #[serde(rename = "merge_whitelist_usernames", default)]
    pub merge_whitelist_usernames: Vec<String>,
    #[serde(rename = "merge_whitelist_teams", default)]
    pub merge_whitelist_teams: Vec<String>,
    #[serde(
        rename = "enable_status_check",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_status_check: Option<bool>,
    #[serde(rename = "status_check_contexts", default)]
    pub status_check_contexts: Vec<String>,
    #[serde(
        rename = "required_approvals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub required_approvals: Option<i64>,
    #[serde(
        rename = "enable_approvals_whitelist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_approvals_whitelist: Option<bool>,
    #[serde(rename = "approvals_whitelist_username", default)]
    pub approvals_whitelist_usernames: Vec<String>,
    #[serde(rename = "approvals_whitelist_teams", default)]
    pub approvals_whitelist_teams: Vec<String>,
    #[serde(
        rename = "block_on_rejected_reviews",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_on_rejected_reviews: Option<bool>,
    #[serde(
        rename = "block_on_official_review_requests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_on_official_review_requests: Option<bool>,
    #[serde(
        rename = "block_on_outdated_branch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub block_on_outdated_branch: Option<bool>,
    #[serde(
        rename = "dismiss_stale_approvals",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dismiss_stale_approvals: Option<bool>,
    #[serde(
        rename = "require_signed_commits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_signed_commits: Option<bool>,
    #[serde(
        rename = "protected_file_patterns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub protected_file_patterns: Option<String>,
    #[serde(
        rename = "unprotected_file_patterns",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub unprotected_file_patterns: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_repo_branch_option_validate_success() {
        let opt = UpdateRepoBranchOption {
            name: "main".to_string(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_update_repo_branch_option_validate_empty_name() {
        let opt = UpdateRepoBranchOption {
            name: String::new(),
        };
        assert!(opt.validate().is_err());
    }
}
