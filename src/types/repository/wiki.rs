// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Wiki-related types (`repo_wiki.go`).

use crate::{Deserialize, Serialize};

use crate::types::repository::commit::CommitUser;
use crate::types::serde_helpers::null_to_default;

// ── repo_wiki.go ────────────────────────────────────────────────

/// `WikiCommit` represents a wiki commit/revision
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

/// `WikiPage` represents a wiki page
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

/// `WikiPageMetaData` represents metadata for a wiki page (without content)
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

/// `WikiCommitList` represents a list of wiki commits
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Wiki Commit List payload type.
pub struct WikiCommitList {
    #[serde(default, deserialize_with = "null_to_default")]
    pub commits: Vec<WikiCommit>,
    pub count: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
