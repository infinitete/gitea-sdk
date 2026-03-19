// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::internal::request::urlencoding;
use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

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
