// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::serde_helpers::{null_to_default, nullable_rfc3339};
use super::team::Team;
use super::user::User;
use crate::types::enums::{MergeStyle, ProjectsMode};

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
    #[serde(default)]
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
        with = "nullable_rfc3339",
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
    #[serde(default)]
    pub topics: Vec<String>,
    #[serde(default)]
    pub licenses: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo_transfer: Option<RepoTransfer>,
}

// ── repo_branch.go ──────────────────────────────────────────────

/// PayloadUser represents the author or committer of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload User payload type.
pub struct PayloadUser {
    /// Full name of the commit author
    pub name: String,
    pub email: String,
    pub username: String,
}

/// PayloadCommitVerification represents the GPG verification of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Payload Commit Verification payload type.
pub struct PayloadCommitVerification {
    pub verified: bool,
    pub reason: String,
    pub signature: String,
    pub payload: String,
}

/// PayloadCommit represents a commit
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
    #[serde(rename = "status_check_contexts", default)]
    pub status_check_contexts: Vec<String>,
    #[serde(rename = "user_can_push")]
    pub user_can_push: bool,
    #[serde(rename = "user_can_merge")]
    pub user_can_merge: bool,
    #[serde(rename = "effective_branch_protection_name")]
    pub effective_branch_protection_name: String,
}

// ── repo_commit.go ──────────────────────────────────────────────

/// Identity for a person's identity like an author or committer
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Identity payload type.
pub struct Identity {
    pub name: String,
    pub email: String,
}

/// CommitMeta contains meta information of a commit in terms of API
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commit Meta payload type.
pub struct CommitMeta {
    pub url: String,
    pub sha: String,
    #[serde(rename = "created", with = "rfc3339")]
    pub created: OffsetDateTime,
}

/// CommitUser contains information of a user in the context of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commit User payload type.
pub struct CommitUser {
    #[serde(flatten)]
    pub identity: Identity,
    pub date: String,
}

/// RepoCommit contains information of a commit in the context of a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Repo Commit payload type.
pub struct RepoCommit {
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<CommitUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<CommitUser>,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<CommitMeta>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<PayloadCommitVerification>,
}

/// CommitStats contains stats from a Git commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commit Stats payload type.
pub struct CommitStats {
    pub total: i32,
    pub additions: i32,
    pub deletions: i32,
}

/// CommitAffectedFiles store information about files affected by the commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commit Affected Files payload type.
pub struct CommitAffectedFiles {
    pub filename: String,
}

/// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commit Date Options payload type.
pub struct CommitDateOptions {
    #[serde(with = "rfc3339")]
    pub author: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub committer: OffsetDateTime,
}

/// Commit contains information generated from a Git commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Commit payload type.
pub struct Commit {
    #[serde(flatten)]
    pub commit_meta: CommitMeta,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<RepoCommit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<User>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub parents: Vec<CommitMeta>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub files: Vec<CommitAffectedFiles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<CommitStats>,
}

// ── repo_tag.go ─────────────────────────────────────────────────

/// Tag represents a repository tag
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tag payload type.
pub struct Tag {
    pub name: String,
    pub message: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<CommitMeta>,
    #[serde(rename = "zipball_url")]
    pub zipball_url: String,
    #[serde(rename = "tarball_url")]
    pub tarball_url: String,
}

/// AnnotatedTag represents an annotated tag
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Annotated Tag payload type.
pub struct AnnotatedTag {
    pub tag: String,
    pub sha: String,
    pub url: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tagger: Option<CommitUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<AnnotatedTagObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<PayloadCommitVerification>,
}

/// AnnotatedTagObject contains meta information of the tag object
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Annotated Tag Object payload type.
pub struct AnnotatedTagObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub sha: String,
}

// ── repo_file.go ────────────────────────────────────────────────

/// FileLinksResponse contains the links for a repo's file
#[derive(Debug, Clone, Serialize, Deserialize)]
/// File Links Response payload type.
pub struct FileLinksResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

/// ContentsResponse contains information about a repo's entry's metadata and content
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contents Response payload type.
pub struct ContentsResponse {
    pub name: String,
    pub path: String,
    pub sha: String,
    /// `type` will be `file`, `dir`, `symlink`, or `submodule`
    #[serde(rename = "type")]
    pub type_: String,
    pub size: i64,
    /// `encoding` is populated when `type` is `file`, otherwise null
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// `content` is populated when `type` is `file`, otherwise null
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// `target` is populated when `type` is `symlink`, otherwise null
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub git_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    /// `submodule_git_url` is populated when `type` is `submodule`, otherwise null
    #[serde(
        rename = "submodule_git_url",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub submodule_git_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub links: Option<FileLinksResponse>,
    #[serde(rename = "last_commit_sha")]
    pub last_commit_sha: String,
}

/// FileCommitResponse contains information generated from a Git commit for a repo's file
#[derive(Debug, Clone, Serialize, Deserialize)]
/// File Commit Response payload type.
pub struct FileCommitResponse {
    #[serde(flatten)]
    pub commit_meta: CommitMeta,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<CommitUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committer: Option<CommitUser>,
    #[serde(default)]
    pub parents: Vec<CommitMeta>,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tree: Option<CommitMeta>,
}

/// FileResponse contains information about a repo's file
#[derive(Debug, Clone, Serialize, Deserialize)]
/// File Response payload type.
pub struct FileResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentsResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<FileCommitResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification: Option<PayloadCommitVerification>,
}

// ── repo_file_ext.go ────────────────────────────────────────────

/// ContentsExtResponse contains extended information about a repo's contents
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Contents Ext Response payload type.
pub struct ContentsExtResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir_contents: Option<Vec<ContentsResponse>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_contents: Option<ContentsResponse>,
}

// ── repo_branch_protection.go ───────────────────────────────────

/// BranchProtection represents a branch protection for a repository
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
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,
}

// ── repo_tag_protection.go ──────────────────────────────────────

/// TagProtection represents a tag protection for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tag Protection payload type.
pub struct TagProtection {
    pub id: i64,
    #[serde(rename = "name_pattern")]
    pub name_pattern: String,
    #[serde(default)]
    pub whitelist_usernames: Vec<String>,
    #[serde(default)]
    pub whitelist_teams: Vec<String>,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,
}

// ── repo_key.go ─────────────────────────────────────────────────

/// DeployKey a deploy key
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deploy Key payload type.
pub struct DeployKey {
    pub id: i64,
    #[serde(rename = "key_id")]
    pub key_id: i64,
    pub key: String,
    pub url: String,
    pub title: String,
    pub fingerprint: String,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "read_only")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<Repository>>,
}

// ── git_hook.go ─────────────────────────────────────────────────

/// GitHook represents a Git repository hook
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Git Hook payload type.
pub struct GitHook {
    pub name: String,
    #[serde(rename = "is_active")]
    pub is_active: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

// ── repo_collaborator.go ────────────────────────────────────────

/// CollaboratorPermissionResult result type for CollaboratorPermission
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Collaborator Permission Result payload type.
pub struct CollaboratorPermissionResult {
    pub permission: crate::types::enums::AccessMode,
    #[serde(rename = "role_name")]
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

// ── repo_mirror.go ──────────────────────────────────────────────

/// PushMirrorResponse returns a git push mirror
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Push Mirror Response payload type.
pub struct PushMirrorResponse {
    pub created: String,
    pub interval: String,
    #[serde(rename = "last_error")]
    pub last_error: String,
    #[serde(rename = "last_update")]
    pub last_update: String,
    #[serde(rename = "remote_address")]
    pub remote_address: String,
    #[serde(rename = "remote_name")]
    pub remote_name: String,
    #[serde(rename = "repo_name")]
    pub repo_name: String,
    #[serde(rename = "sync_on_commit")]
    pub sync_on_commit: bool,
}

// ── repo_tree.go ────────────────────────────────────────────────

/// GitEntry represents a git tree entry
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Git Entry payload type.
pub struct GitEntry {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub size: i64,
    pub sha: String,
    pub url: String,
}

/// GitTreeResponse returns a git tree
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Git Tree Response payload type.
pub struct GitTreeResponse {
    pub sha: String,
    pub url: String,
    #[serde(default)]
    pub tree: Vec<GitEntry>,
    pub truncated: bool,
    pub page: i32,
    #[serde(rename = "total_count")]
    pub total_count: i32,
}

// ── repo_refs.go ────────────────────────────────────────────────

/// GitObject represents a Git object
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Git Object payload type.
pub struct GitObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub sha: String,
    pub url: String,
}

/// Reference represents a Git reference
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reference payload type.
pub struct Reference {
    #[serde(rename = "ref")]
    pub ref_: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<GitObject>,
}

// ── git_blob.go ─────────────────────────────────────────────────

/// GitBlobResponse represents a git blob
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Git Blob Response payload type.
pub struct GitBlobResponse {
    pub content: String,
    pub encoding: String,
    pub url: String,
    pub sha: String,
    pub size: i64,
}

// ── repo_compare.go ─────────────────────────────────────────────

/// Compare represents a comparison between two commits
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Compare payload type.
pub struct Compare {
    /// Total number of commits in the comparison
    #[serde(rename = "total_commits")]
    pub total_commits: i32,
    /// List of commits in the comparison
    #[serde(default, deserialize_with = "null_to_default")]
    pub commits: Vec<Commit>,
}

// ── repo_wiki.go ────────────────────────────────────────────────

/// WikiCommit represents a wiki commit/revision
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wiki Commit payload type.
pub struct WikiCommit {
    pub sha: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<CommitUser>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commiter: Option<CommitUser>,
}

/// WikiPage represents a wiki page
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wiki Page payload type.
pub struct WikiPage {
    pub title: String,
    #[serde(rename = "content_base64")]
    pub content_base64: String,
    #[serde(rename = "commit_count")]
    pub commit_count: i64,
    pub sidebar: String,
    pub footer: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "sub_url")]
    pub sub_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_commit: Option<WikiCommit>,
}

/// WikiPageMetaData represents metadata for a wiki page (without content)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wiki Page Meta Data payload type.
pub struct WikiPageMetaData {
    pub title: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "sub_url")]
    pub sub_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_commit: Option<WikiCommit>,
}

/// WikiCommitList represents a list of wiki commits
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wiki Commit List payload type.
pub struct WikiCommitList {
    #[serde(default)]
    pub commits: Vec<WikiCommit>,
    pub count: i64,
}

// ── repo_git_notes.go ───────────────────────────────────────────

/// Note represents a git note
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Note payload type.
pub struct Note {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
}

// ── repo_action_variable.go ─────────────────────────────────────

/// RepoActionVariable represents a action variable
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Repo Action Variable payload type.
pub struct RepoActionVariable {
    #[serde(rename = "owner_id")]
    pub owner_id: i64,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    pub name: String,
    pub data: String,
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
            visibility: crate::types::enums::VisibleType::Public,
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
    fn test_commit_meta_round_trip() {
        let original = CommitMeta {
            url: "https://example.com/commit/abc".to_string(),
            sha: "abc123".to_string(),
            created: test_time(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: CommitMeta = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.sha, "abc123");
    }

    #[test]
    fn test_tag_round_trip() {
        let original = Tag {
            name: "v1.0".to_string(),
            message: "Release v1.0".to_string(),
            id: "abc123".to_string(),
            commit: None,
            zipball_url: "https://example.com/archive/zip".to_string(),
            tarball_url: "https://example.com/archive/tar".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Tag = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, "v1.0");
        assert!(restored.commit.is_none());
    }

    #[test]
    fn test_contents_response_round_trip() {
        let original = ContentsResponse {
            name: "README.md".to_string(),
            path: "README.md".to_string(),
            sha: "def456".to_string(),
            type_: "file".to_string(),
            size: 100,
            encoding: None,
            content: None,
            target: None,
            url: None,
            html_url: None,
            git_url: None,
            download_url: None,
            submodule_git_url: None,
            links: None,
            last_commit_sha: "abc123".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ContentsResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, "README.md");
        assert!(restored.content.is_none());
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

    #[test]
    fn test_deploy_key_round_trip() {
        let original = DeployKey {
            id: 1,
            key_id: 2,
            key: "ssh-rsa AAAA...".to_string(),
            url: "https://example.com/keys/1".to_string(),
            title: "CI key".to_string(),
            fingerprint: "abcd".to_string(),
            created: test_time(),
            read_only: true,
            repository: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: DeployKey = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.title, "CI key");
        assert!(restored.repository.is_none());
    }

    #[test]
    fn test_git_hook_round_trip() {
        let original = GitHook {
            name: "pre-receive".to_string(),
            is_active: true,
            content: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GitHook = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, "pre-receive");
        assert!(restored.content.is_none());
    }

    #[test]
    fn test_git_tree_response_round_trip() {
        let original = GitTreeResponse {
            sha: "abc123".to_string(),
            url: "https://example.com/tree".to_string(),
            tree: vec![],
            truncated: false,
            page: 1,
            total_count: 0,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GitTreeResponse = serde_json::from_str(&json).unwrap();
        assert!(restored.tree.is_empty());
    }

    #[test]
    fn test_git_entry_round_trip() {
        let original = GitEntry {
            path: "src/main.rs".to_string(),
            mode: "100644".to_string(),
            type_: "blob".to_string(),
            size: 1024,
            sha: "abc123".to_string(),
            url: "https://example.com/blob".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GitEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.path, "src/main.rs");
    }

    #[test]
    fn test_reference_round_trip() {
        let original = Reference {
            ref_: "refs/heads/main".to_string(),
            url: "https://example.com/ref".to_string(),
            object: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Reference = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.ref_, "refs/heads/main");
        assert!(restored.object.is_none());
    }

    #[test]
    fn test_git_blob_response_round_trip() {
        let original = GitBlobResponse {
            content: "aGVsbG8=".to_string(),
            encoding: "base64".to_string(),
            url: "https://example.com/blob".to_string(),
            sha: "abc123".to_string(),
            size: 5,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GitBlobResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.encoding, "base64");
    }

    #[test]
    fn test_wiki_page_round_trip() {
        let original = WikiPage {
            title: "Home".to_string(),
            content_base64: "SGVsbG8=".to_string(),
            commit_count: 1,
            sidebar: String::new(),
            footer: String::new(),
            html_url: "https://example.com/wiki/Home".to_string(),
            sub_url: "wiki/Home".to_string(),
            last_commit: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: WikiPage = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.title, "Home");
        assert!(restored.last_commit.is_none());
    }

    #[test]
    fn test_collaborator_permission_result_round_trip() {
        let original = CollaboratorPermissionResult {
            permission: crate::types::enums::AccessMode::Write,
            role: "write".to_string(),
            user: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: CollaboratorPermissionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.permission, crate::types::enums::AccessMode::Write);
        assert!(restored.user.is_none());
    }

    #[test]
    fn test_push_mirror_response_round_trip() {
        let original = PushMirrorResponse {
            created: "2024-01-01".to_string(),
            interval: "8h".to_string(),
            last_error: String::new(),
            last_update: "2024-01-15".to_string(),
            remote_address: "https://mirror.example.com".to_string(),
            remote_name: "origin".to_string(),
            repo_name: "test-repo".to_string(),
            sync_on_commit: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PushMirrorResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.interval, "8h");
    }

    #[test]
    fn test_compare_round_trip() {
        let original = Compare {
            total_commits: 2,
            commits: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Compare = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.total_commits, 2);
        assert!(restored.commits.is_empty());
    }
}
