// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::internal::request::urlencoding;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::{MergeStyle, ProjectsMode, RepoType, TrustModel};
use crate::types::repository::{ExternalTracker, ExternalWiki, InternalTracker};
use crate::{Deserialize, Serialize};

// ── repo.go ─────────────────────────────────────────────────────

/// ListReposOptions options for listing repositories
#[derive(Debug, Clone, Default)]
/// Options for List Repos Option.
pub struct ListReposOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListReposOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListOrgReposOptions options for a organization's repositories
#[derive(Debug, Clone, Default)]
/// Options for List Org Repos Option.
pub struct ListOrgReposOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgReposOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// SearchRepoOptions options for searching repositories
#[derive(Debug, Clone, Default)]
/// Options for Search Repo Option.
pub struct SearchRepoOptions {
    pub list_options: ListOptions,
    pub keyword: String,
    pub keyword_is_topic: bool,
    pub keyword_in_description: bool,
    pub owner_id: i64,
    pub starred_by_user_id: i64,
    pub is_private: Option<bool>,
    pub is_archived: Option<bool>,
    pub exclude_template: bool,
    pub repo_type: Option<RepoType>,
    pub sort: String,
    pub order: String,
    pub prioritized_by_owner_id: i64,
    pub raw_query: String,
}

impl QueryEncode for SearchRepoOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();

        if !self.keyword.is_empty() {
            out.push_str(&format!("&q={}", urlencoding(&self.keyword)));
        }
        if self.keyword_is_topic {
            out.push_str("&topic=true");
        }
        if self.keyword_in_description {
            out.push_str("&includeDesc=true");
        }
        if self.owner_id > 0 {
            out.push_str(&format!("&uid={}", self.owner_id));
            out.push_str("&exclusive=true");
        }
        if self.starred_by_user_id > 0 {
            out.push_str(&format!("&starredBy={}", self.starred_by_user_id));
        }
        if let Some(is_private) = self.is_private {
            out.push_str(&format!("&is_private={}", is_private));
        }
        if let Some(is_archived) = self.is_archived {
            out.push_str(&format!("&archived={}", is_archived));
        }
        if self.exclude_template {
            out.push_str("&template=false");
        }
        if let Some(ref repo_type) = self.repo_type
            && !repo_type.as_ref().is_empty()
        {
            out.push_str(&format!("&mode={}", repo_type.as_ref()));
        }
        if !self.sort.is_empty() {
            out.push_str(&format!("&sort={}", urlencoding(&self.sort)));
        }
        if self.prioritized_by_owner_id > 0 {
            out.push_str(&format!(
                "&priority_owner_id={}",
                self.prioritized_by_owner_id
            ));
        }
        if !self.order.is_empty() {
            out.push_str(&format!("&order={}", urlencoding(&self.order)));
        }

        out
    }
}

/// CreateRepoOption options when creating repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Repo Option.
pub struct CreateRepoOption {
    pub name: String,
    pub description: String,
    pub private: bool,
    #[serde(rename = "issue_labels")]
    pub issue_labels: String,
    #[serde(rename = "auto_init")]
    pub auto_init: bool,
    pub template: bool,
    pub gitignores: String,
    pub license: String,
    pub readme: String,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "trust_model")]
    pub trust_model: TrustModel,
    #[serde(rename = "object_format_name")]
    pub object_format_name: String,
}

impl CreateRepoOption {
    /// Validate this `CreateRepoOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::Error::Validation("name is empty".to_string()));
        }
        if self.name.len() > 100 {
            return Err(crate::Error::Validation(
                "name has more than 100 chars".to_string(),
            ));
        }
        if self.description.len() > 2048 {
            return Err(crate::Error::Validation(
                "description has more than 2048 chars".to_string(),
            ));
        }
        if self.default_branch.len() > 100 {
            return Err(crate::Error::Validation(
                "default branch name has more than 100 chars".to_string(),
            ));
        }
        if !self.object_format_name.is_empty()
            && self.object_format_name != "sha1"
            && self.object_format_name != "sha256"
        {
            return Err(crate::Error::Validation(
                "object format must be sha1 or sha256".to_string(),
            ));
        }
        Ok(())
    }
}

/// EditRepoOption options when editing a repository's properties
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Repo Option.
pub struct EditRepoOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<bool>,
    #[serde(
        rename = "has_issues",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_issues: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_tracker: Option<InternalTracker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_tracker: Option<ExternalTracker>,
    #[serde(rename = "has_wiki", default, skip_serializing_if = "Option::is_none")]
    pub has_wiki: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_wiki: Option<ExternalWiki>,
    #[serde(
        rename = "default_branch",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch: Option<String>,
    #[serde(
        rename = "has_pull_requests",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_pull_requests: Option<bool>,
    #[serde(
        rename = "has_projects",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_projects: Option<bool>,
    #[serde(
        rename = "has_releases",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_releases: Option<bool>,
    #[serde(
        rename = "has_packages",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_packages: Option<bool>,
    #[serde(
        rename = "has_actions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_actions: Option<bool>,
    #[serde(
        rename = "ignore_whitespace_conflicts",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_whitespace_conflicts: Option<bool>,
    #[serde(
        rename = "allow_fast_forward_only_merge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_fast_forward_only_merge: Option<bool>,
    #[serde(
        rename = "allow_merge_commits",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_merge: Option<bool>,
    #[serde(
        rename = "allow_rebase",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_rebase: Option<bool>,
    #[serde(
        rename = "allow_rebase_explicit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_rebase_merge: Option<bool>,
    #[serde(
        rename = "allow_squash_merge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_squash: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(
        rename = "mirror_interval",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub mirror_interval: Option<String>,
    #[serde(
        rename = "allow_manual_merge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_manual_merge: Option<bool>,
    #[serde(
        rename = "autodetect_manual_merge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub autodetect_manual_merge: Option<bool>,
    #[serde(
        rename = "default_merge_style",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_merge_style: Option<MergeStyle>,
    #[serde(
        rename = "projects_mode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub projects_mode: Option<ProjectsMode>,
    #[serde(
        rename = "default_delete_branch_after_merge",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub default_delete_branch_after_merge: Option<bool>,
}

/// UpdateRepoAvatarOption options for updating repository avatar
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Update Repo Avatar Option.
pub struct UpdateRepoAvatarOption {
    /// base64 encoded image
    pub image: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::TrustModel;

    #[test]
    fn test_create_repo_option_validate_success() {
        let opt = CreateRepoOption {
            name: "test-repo".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_repo_option_validate_empty_name() {
        let opt = CreateRepoOption {
            name: String::new(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_option_validate_whitespace_name() {
        let opt = CreateRepoOption {
            name: "   ".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_option_validate_name_too_long() {
        let opt = CreateRepoOption {
            name: "a".repeat(101),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_option_validate_description_too_long() {
        let opt = CreateRepoOption {
            name: "test".to_string(),
            description: "d".repeat(2049),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_option_validate_invalid_object_format() {
        let opt = CreateRepoOption {
            name: "test".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: "sha512".to_string(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_option_validate_sha256_format() {
        let opt = CreateRepoOption {
            name: "test".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: "sha256".to_string(),
        };
        assert!(opt.validate().is_ok());
    }
}
