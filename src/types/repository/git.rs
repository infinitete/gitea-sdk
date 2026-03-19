// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Git-related types (repo_tree.go, repo_refs.go, git_blob.go, repo_git_notes.go).

use crate::{Deserialize, Serialize};

use crate::types::repository::commit::Commit;
use crate::types::serde_helpers::null_to_default;

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
    #[serde(default, deserialize_with = "null_to_default")]
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

// ── repo_git_notes.go ───────────────────────────────────────────

/// Note represents a git note
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Note payload type.
pub struct Note {
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<Commit>,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
