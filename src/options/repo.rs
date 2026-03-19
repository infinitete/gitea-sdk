// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for repository API endpoints.

use crate::internal::request::urlencoding;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::{
    AccessMode, GitServiceType, MergeStyle, ProjectsMode, RepoType, TrustModel,
};
use crate::types::repository::{
    CommitDateOptions, ExternalTracker, ExternalWiki, Identity, InternalTracker,
};
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

// ── repo_branch.go ──────────────────────────────────────────────

/// ListRepoBranchesOptions options for listing a repository's branches
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

/// CreateBranchOption options when creating a branch in a repository
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

/// UpdateRepoBranchOption options when renaming a branch
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

// ── repo_tag.go ─────────────────────────────────────────────────

/// ListRepoTagsOptions options for listing a repository's tags
#[derive(Debug, Clone, Default)]
/// Options for List Repo Tags Option.
pub struct ListRepoTagsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoTagsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListRepoTagProtectionsOptions options for listing tag protections
#[derive(Debug, Clone, Default)]
/// Options for List Repo Tag Protections Option.
pub struct ListRepoTagProtectionsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoTagProtectionsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreateTagOption options when creating a tag
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Tag Option.
pub struct CreateTagOption {
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    pub message: String,
    pub target: String,
}

impl CreateTagOption {
    /// Validate this `CreateTagOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.tag_name.is_empty() {
            return Err(crate::Error::Validation("TagName is required".to_string()));
        }
        Ok(())
    }
}

// ── repo_collaborator.go ────────────────────────────────────────

/// ListCollaboratorsOptions options for listing a repository's collaborators
#[derive(Debug, Clone, Default)]
/// Options for List Collaborators Option.
pub struct ListCollaboratorsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListCollaboratorsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// AddCollaboratorOption options when adding a user as a collaborator
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Add Collaborator Option.
pub struct AddCollaboratorOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<AccessMode>,
}

impl AddCollaboratorOption {
    /// Validate this `AddCollaboratorOption` payload.
    pub fn validate(&mut self) -> crate::Result<()> {
        if let Some(ref perm) = self.permission {
            match perm {
                AccessMode::Owner => {
                    self.permission = Some(AccessMode::Admin);
                    return Ok(());
                }
                AccessMode::None => {
                    self.permission = None;
                    return Ok(());
                }
                AccessMode::Read | AccessMode::Write | AccessMode::Admin => {}
                _ => {
                    return Err(crate::Error::Validation(
                        "permission mode invalid".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

// ── repo_commit.go ──────────────────────────────────────────────

/// ListCommitOptions list commit options
#[derive(Debug, Clone, Default)]
/// Options for List Commit Option.
pub struct ListCommitOptions {
    pub list_options: ListOptions,
    pub sha: String,
    pub path: String,
    pub stat: bool,
    pub verification: bool,
    pub files: bool,
    pub not: String,
}

impl QueryEncode for ListCommitOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();

        if !self.sha.is_empty() {
            out.push_str(&format!("&sha={}", urlencoding(&self.sha)));
        }
        if !self.path.is_empty() {
            out.push_str(&format!("&path={}", urlencoding(&self.path)));
        }
        out.push_str(&format!("&stat={}", self.stat));
        out.push_str(&format!("&verification={}", self.verification));
        out.push_str(&format!("&files={}", self.files));
        if !self.not.is_empty() {
            out.push_str(&format!("&not={}", urlencoding(&self.not)));
        }

        out
    }
}

// ── repo_file.go ────────────────────────────────────────────────

/// FileOptions options for all file APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for File Option.
pub struct FileOptions {
    pub message: String,
    #[serde(rename = "branch")]
    pub branch_name: String,
    #[serde(rename = "new_branch")]
    pub new_branch_name: String,
    pub author: Identity,
    pub committer: Identity,
    pub dates: CommitDateOptions,
    pub signoff: bool,
}

/// CreateFileOptions options for creating files
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create File Option.
pub struct CreateFileOptions {
    #[serde(flatten)]
    pub file_options: FileOptions,
    /// content must be base64 encoded
    pub content: String,
}

/// DeleteFileOptions options for deleting files
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Delete File Option.
pub struct DeleteFileOptions {
    #[serde(flatten)]
    pub file_options: FileOptions,
    /// sha is the SHA for the file that already exists
    pub sha: String,
}

/// UpdateFileOptions options for updating files
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Update File Option.
pub struct UpdateFileOptions {
    #[serde(flatten)]
    pub file_options: FileOptions,
    /// sha is the SHA for the file that already exists
    pub sha: String,
    /// content must be base64 encoded
    pub content: String,
    #[serde(rename = "from_path")]
    pub from_path: String,
}

// ── repo_file_ext.go ────────────────────────────────────────────

/// GetContentsExtOptions options for getting extended contents
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Get Contents Ext Option.
pub struct GetContentsExtOptions {
    pub r#ref: String,
    pub includes: String,
}

// ── repo_migrate.go ─────────────────────────────────────────────

/// MigrateRepoOption options for migrating a repository from an external service
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Migrate Repo Option.
pub struct MigrateRepoOption {
    #[serde(rename = "repo_name")]
    pub repo_name: String,
    #[serde(rename = "repo_owner")]
    pub repo_owner: String,
    /// deprecated use RepoOwner
    pub uid: i64,
    #[serde(rename = "clone_addr")]
    pub clone_addr: String,
    pub service: GitServiceType,
    #[serde(rename = "auth_username")]
    pub auth_username: String,
    #[serde(rename = "auth_password")]
    pub auth_password: String,
    #[serde(rename = "auth_token")]
    pub auth_token: String,
    pub mirror: bool,
    pub private: bool,
    pub description: String,
    pub wiki: bool,
    pub milestones: bool,
    pub labels: bool,
    pub issues: bool,
    #[serde(rename = "pull_requests")]
    pub pull_requests: bool,
    pub releases: bool,
    #[serde(rename = "mirror_interval")]
    pub mirror_interval: String,
    pub lfs: bool,
    #[serde(rename = "lfs_endpoint")]
    pub lfs_endpoint: String,
}

impl MigrateRepoOption {
    /// Validate this `MigrateRepoOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.clone_addr.is_empty() {
            return Err(crate::Error::Validation("clone addr required".to_string()));
        }
        if self.repo_name.is_empty() {
            return Err(crate::Error::Validation("repo name required".to_string()));
        } else if self.repo_name.len() > 100 {
            return Err(crate::Error::Validation("repo name too long".to_string()));
        }
        if self.description.len() > 2048 {
            return Err(crate::Error::Validation("description too long".to_string()));
        }
        match self.service {
            GitServiceType::Github => {
                if self.auth_token.is_empty() {
                    return Err(crate::Error::Validation(
                        "github requires token authentication".to_string(),
                    ));
                }
            }
            GitServiceType::Gitlab | GitServiceType::Gitea => {
                if self.auth_token.is_empty() {
                    return Err(crate::Error::Validation(format!(
                        "{} requires token authentication",
                        self.service
                    )));
                }
            }
            GitServiceType::Gogs => {
                if self.auth_token.is_empty() {
                    return Err(crate::Error::Validation(
                        "gogs requires token authentication".to_string(),
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// ── repo_label.go ───────────────────────────────────────────────

/// ListLabelsOptions options for listing repository's labels
#[derive(Debug, Clone, Default)]
/// Options for List Labels Option.
pub struct ListLabelsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListLabelsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreateLabelOption options for creating a label
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Label Option.
pub struct CreateLabelOption {
    pub name: String,
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
}

impl CreateLabelOption {
    /// Validate this `CreateLabelOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        let color = self.color.trim_start_matches('#');
        if color.len() != 6 || !color.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(crate::Error::Validation("invalid color format".to_string()));
        }
        if self.name.trim().is_empty() {
            return Err(crate::Error::Validation(
                "empty name not allowed".to_string(),
            ));
        }
        Ok(())
    }
}

/// EditLabelOption options for editing a label
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Label Option.
pub struct EditLabelOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    #[serde(
        rename = "is_archived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_archived: Option<bool>,
}

impl EditLabelOption {
    /// Validate this `EditLabelOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if let Some(ref color) = self.color {
            let color = color.trim_start_matches('#');
            if color.len() != 6 || !color.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(crate::Error::Validation("invalid color format".to_string()));
            }
        }
        if let Some(ref name) = self.name
            && name.trim().is_empty()
        {
            return Err(crate::Error::Validation(
                "empty name not allowed".to_string(),
            ));
        }
        Ok(())
    }
}

// ── repo_stars.go ───────────────────────────────────────────────

/// ListStargazersOptions options for listing a repository's stargazers
#[derive(Debug, Clone, Default)]
/// Options for List Stargazers Option.
pub struct ListStargazersOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListStargazersOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── repo_key.go ─────────────────────────────────────────────────

/// ListDeployKeysOptions options for listing a repository's deploy keys
#[derive(Debug, Clone, Default)]
/// Options for List Deploy Keys Option.
pub struct ListDeployKeysOptions {
    pub list_options: ListOptions,
    pub key_id: i64,
    pub fingerprint: String,
}

impl QueryEncode for ListDeployKeysOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();

        if self.key_id > 0 {
            out.push_str(&format!("&key_id={}", self.key_id));
        }
        if !self.fingerprint.is_empty() {
            out.push_str(&format!("&fingerprint={}", urlencoding(&self.fingerprint)));
        }

        out
    }
}

/// CreateKeyOption options when creating a deploy key
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Key Option.
pub struct CreateKeyOption {
    pub title: String,
    /// An armored SSH key to add
    pub key: String,
    /// Describe if the key has only read access or read/write
    #[serde(rename = "read_only")]
    pub read_only: bool,
}

// ── repo_mirror.go ──────────────────────────────────────────────

/// CreatePushMirrorOption options for creating a push mirror
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Push Mirror Option.
pub struct CreatePushMirrorOption {
    pub interval: String,
    #[serde(rename = "remote_address")]
    pub remote_address: String,
    #[serde(rename = "remote_password")]
    pub remote_password: String,
    #[serde(rename = "remote_username")]
    pub remote_username: String,
    #[serde(rename = "sync_on_commit")]
    pub sync_on_commit: bool,
}

/// ListPushMirrorOptions options for listing push mirrors
#[derive(Debug, Clone, Default)]
/// Options for List Push Mirror Option.
pub struct ListPushMirrorOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPushMirrorOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── repo_topics.go ──────────────────────────────────────────────

/// ListRepoTopicsOptions options for listing repo's topics
#[derive(Debug, Clone, Default)]
/// Options for List Repo Topics Option.
pub struct ListRepoTopicsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoTopicsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── repo_transfer.go ────────────────────────────────────────────

/// TransferRepoOption options when transfer a repository's ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Transfer Repo Option.
pub struct TransferRepoOption {
    #[serde(rename = "new_owner")]
    pub new_owner: String,
    #[serde(rename = "team_ids")]
    pub team_ids: Option<Vec<i64>>,
}

// ── repo_template.go ────────────────────────────────────────────

/// CreateRepoFromTemplateOption options when creating repository using a template
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Repo From Template Option.
pub struct CreateRepoFromTemplateOption {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub private: bool,
    #[serde(rename = "git_content")]
    pub git_content: bool,
    pub topics: bool,
    #[serde(rename = "git_hooks")]
    pub git_hooks: bool,
    pub webhooks: bool,
    pub avatar: bool,
    pub labels: bool,
}

impl CreateRepoFromTemplateOption {
    /// Validate this `CreateRepoFromTemplateOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.owner.is_empty() {
            return Err(crate::Error::Validation(
                "field Owner is required".to_string(),
            ));
        }
        if self.name.is_empty() {
            return Err(crate::Error::Validation(
                "field Name is required".to_string(),
            ));
        }
        Ok(())
    }
}

// ── repo_tree.go ────────────────────────────────────────────────

/// ListTreeOptions options for listing repository tree
#[derive(Debug, Clone, Default)]
/// Options for List Tree Option.
pub struct ListTreeOptions {
    pub list_options: ListOptions,
    /// Ref can be branch/tag/commit. required
    pub r#ref: String,
    /// Recursive if true will return the tree in a recursive fashion
    pub recursive: bool,
}

impl QueryEncode for ListTreeOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();

        if self.recursive {
            out.push_str("&recursive=1");
        }

        out
    }
}

// ── repo_wiki.go ────────────────────────────────────────────────

/// CreateWikiPageOptions options for creating or editing a wiki page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Wiki Page Option.
pub struct CreateWikiPageOptions {
    pub title: String,
    #[serde(rename = "content_base64")]
    pub content_base64: String,
    pub message: String,
}

/// ListWikiPagesOptions options for listing wiki pages
#[derive(Debug, Clone, Default)]
/// Options for List Wiki Pages Option.
pub struct ListWikiPagesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListWikiPagesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListWikiPageRevisionsOptions options for listing wiki page revisions
#[derive(Debug, Clone, Default)]
/// Options for List Wiki Page Revisions Option.
pub struct ListWikiPageRevisionsOptions {
    pub page: i32,
}

// ── repo_branch_protection.go ───────────────────────────────────

/// ListBranchProtectionsOptions list branch protection options
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

/// CreateBranchProtectionOption options for creating a branch protection
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

/// EditBranchProtectionOption options for editing a branch protection
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

// ── repo_tag_protection.go ──────────────────────────────────────

/// CreateTagProtectionOption options for creating a tag protection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Tag Protection Option.
pub struct CreateTagProtectionOption {
    #[serde(rename = "name_pattern")]
    pub name_pattern: String,
    #[serde(default)]
    pub whitelist_usernames: Vec<String>,
    #[serde(default)]
    pub whitelist_teams: Vec<String>,
}

/// EditTagProtectionOption options for editing a tag protection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Tag Protection Option.
pub struct EditTagProtectionOption {
    #[serde(
        rename = "name_pattern",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_pattern: Option<String>,
    #[serde(default)]
    pub whitelist_usernames: Vec<String>,
    #[serde(default)]
    pub whitelist_teams: Vec<String>,
}

// ── fork.go ─────────────────────────────────────────────────────

/// ListForksOptions options for listing repository's forks
#[derive(Debug, Clone, Default)]
/// Options for List Forks Option.
pub struct ListForksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListForksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreateForkOption options for creating a fork
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Fork Option.
pub struct CreateForkOption {
    /// organization name, if forking into an organization
    pub organization: Option<String>,
    /// name of the forked repository
    pub name: Option<String>,
}

// ── git_hook.go ─────────────────────────────────────────────────

/// ListRepoGitHooksOptions options for listing repository's githooks
#[derive(Debug, Clone, Default)]
/// Options for List Repo Git Hooks Option.
pub struct ListRepoGitHooksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoGitHooksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// EditGitHookOption options when modifying one Git hook
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Git Hook Option.
pub struct EditGitHookOption {
    pub content: String,
}

// ── repo_git_notes.go ───────────────────────────────────────────

/// GetRepoNoteOptions options for getting a note
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Get Repo Note Option.
pub struct GetRepoNoteOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_create_tag_option_validate_success() {
        let opt = CreateTagOption {
            tag_name: "v1.0".to_string(),
            message: String::new(),
            target: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_tag_option_validate_empty_tag_name() {
        let opt = CreateTagOption {
            tag_name: String::new(),
            message: String::new(),
            target: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_add_collaborator_option_validate_read() {
        let mut opt = AddCollaboratorOption {
            permission: Some(AccessMode::Read),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_add_collaborator_option_validate_none() {
        let mut opt = AddCollaboratorOption { permission: None };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_label_option_validate_success() {
        let opt = CreateLabelOption {
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_label_option_validate_invalid_color() {
        let opt = CreateLabelOption {
            name: "bug".to_string(),
            color: "red".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_label_option_validate_empty_name() {
        let opt = CreateLabelOption {
            name: String::new(),
            color: "ff0000".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_label_option_validate_color_with_hash() {
        let opt = CreateLabelOption {
            name: "bug".to_string(),
            color: "#00ff00".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_label_option_validate_success() {
        let opt = EditLabelOption {
            name: Some("new-name".to_string()),
            color: Some("abcdef".to_string()),
            description: None,
            exclusive: None,
            is_archived: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_label_option_validate_invalid_color() {
        let opt = EditLabelOption {
            name: None,
            color: Some("zzz".to_string()),
            description: None,
            exclusive: None,
            is_archived: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_label_option_validate_empty_name() {
        let opt = EditLabelOption {
            name: Some("   ".to_string()),
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_success_git() {
        let opt = MigrateRepoOption {
            clone_addr: "https://example.com/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_migrate_repo_option_validate_empty_clone_addr() {
        let opt = MigrateRepoOption {
            clone_addr: String::new(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_empty_repo_name() {
        let opt = MigrateRepoOption {
            clone_addr: "https://example.com/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: String::new(),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_name_too_long() {
        let opt = MigrateRepoOption {
            clone_addr: "https://example.com/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "a".repeat(101),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_github_no_token() {
        let opt = MigrateRepoOption {
            clone_addr: "https://github.com/user/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Github,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_github_with_token() {
        let opt = MigrateRepoOption {
            clone_addr: "https://github.com/user/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Github,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: "token123".to_string(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_repo_from_template_option_validate_success() {
        let opt = CreateRepoFromTemplateOption {
            owner: "myorg".to_string(),
            name: "my-repo".to_string(),
            description: String::new(),
            private: false,
            git_content: false,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_repo_from_template_option_validate_empty_owner() {
        let opt = CreateRepoFromTemplateOption {
            owner: String::new(),
            name: "my-repo".to_string(),
            description: String::new(),
            private: false,
            git_content: false,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_from_template_option_validate_empty_name() {
        let opt = CreateRepoFromTemplateOption {
            owner: "myorg".to_string(),
            name: String::new(),
            description: String::new(),
            private: false,
            git_content: false,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        assert!(opt.validate().is_err());
    }
}
