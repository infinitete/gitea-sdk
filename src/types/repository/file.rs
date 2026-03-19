// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! File-related types (repo_file.go, repo_file_ext.go).

use crate::{Deserialize, Serialize};

use crate::types::repository::branch::PayloadCommitVerification;
use crate::types::repository::commit::{CommitMeta, CommitUser};
use crate::types::serde_helpers::null_to_default;

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
    #[serde(default, deserialize_with = "null_to_default")]
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
