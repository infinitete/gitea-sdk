// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Tag-related types (`repo_tag.go`, `repo_tag_protection.go`).

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::repository::branch::PayloadCommitVerification;
use crate::types::repository::commit::{CommitMeta, CommitUser};
use crate::types::serde_helpers::null_to_default;

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

/// `AnnotatedTag` represents an annotated tag
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

/// `AnnotatedTagObject` contains meta information of the tag object
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Annotated Tag Object payload type.
pub struct AnnotatedTagObject {
    #[serde(rename = "type")]
    pub type_: String,
    pub url: String,
    pub sha: String,
}

// ── repo_tag_protection.go ──────────────────────────────────────

/// `TagProtection` represents a tag protection for a repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tag Protection payload type.
pub struct TagProtection {
    pub id: i64,
    #[serde(rename = "name_pattern")]
    pub name_pattern: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub whitelist_usernames: Vec<String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub whitelist_teams: Vec<String>,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
