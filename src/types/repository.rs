// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::serde_helpers::nullable_rfc3339;
use super::team::Team;
use super::user::User;
use crate::types::enums::{MergeStyle, ProjectsMode};

// ── repo.go ─────────────────────────────────────────────────────

/// Permission represents a set of permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

/// InternalTracker represents settings for internal tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ExternalWiki {
    /// URL of external wiki
    #[serde(rename = "external_wiki_url")]
    pub external_wiki_url: String,
}

/// RepoTransfer represents a pending repository transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct PayloadUser {
    /// Full name of the commit author
    pub name: String,
    pub email: String,
    pub username: String,
}

/// PayloadCommitVerification represents the GPG verification of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayloadCommitVerification {
    pub verified: bool,
    pub reason: String,
    pub signature: String,
    pub payload: String,
}

/// PayloadCommit represents a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub timestamp: OffsetDateTime,
    #[serde(default)]
    pub added: Vec<String>,
    #[serde(default)]
    pub removed: Vec<String>,
    #[serde(default)]
    pub modified: Vec<String>,
}

/// Branch represents a repository branch
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct Identity {
    pub name: String,
    pub email: String,
}

/// CommitMeta contains meta information of a commit in terms of API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitMeta {
    pub url: String,
    pub sha: String,
    #[serde(rename = "created", with = "rfc3339")]
    pub created: OffsetDateTime,
}

/// CommitUser contains information of a user in the context of a commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitUser {
    #[serde(flatten)]
    pub identity: Identity,
    pub date: String,
}

/// RepoCommit contains information of a commit in the context of a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct CommitStats {
    pub total: i32,
    pub additions: i32,
    pub deletions: i32,
}

/// CommitAffectedFiles store information about files affected by the commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitAffectedFiles {
    pub filename: String,
}

/// CommitDateOptions store dates for GIT_AUTHOR_DATE and GIT_COMMITTER_DATE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitDateOptions {
    pub author: OffsetDateTime,
    pub committer: OffsetDateTime,
}

/// Commit contains information generated from a Git commit
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    #[serde(default)]
    pub parents: Vec<CommitMeta>,
    #[serde(default)]
    pub files: Vec<CommitAffectedFiles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<CommitStats>,
}

// ── repo_tag.go ─────────────────────────────────────────────────

/// Tag represents a repository tag
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct AnnotatedTagObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub sha: String,
}

// ── repo_file.go ────────────────────────────────────────────────

/// FileLinksResponse contains the links for a repo's file
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ContentsExtResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir_contents: Option<Vec<ContentsResponse>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_contents: Option<ContentsResponse>,
}

// ── repo_branch_protection.go ───────────────────────────────────

/// BranchProtection represents a branch protection for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct GitObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub sha: String,
    pub url: String,
}

/// Reference represents a Git reference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub ref_: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object: Option<GitObject>,
}

// ── git_blob.go ─────────────────────────────────────────────────

/// GitBlobResponse represents a git blob
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct Compare {
    /// Total number of commits in the comparison
    #[serde(rename = "total_commits")]
    pub total_commits: i32,
    /// List of commits in the comparison
    #[serde(default)]
    pub commits: Vec<Commit>,
}

// ── repo_wiki.go ────────────────────────────────────────────────

/// WikiCommit represents a wiki commit/revision
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct WikiCommitList {
    #[serde(default)]
    pub commits: Vec<WikiCommit>,
    pub count: i64,
}

// ── repo_git_notes.go ───────────────────────────────────────────

/// Note represents a git note
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
}

// ── repo_action_variable.go ─────────────────────────────────────

/// RepoActionVariable represents a action variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoActionVariable {
    #[serde(rename = "owner_id")]
    pub owner_id: i64,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    pub name: String,
    pub data: String,
}
