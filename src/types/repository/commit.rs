// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Commit-related types (repo_commit.go, repo_compare.go).

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::repository::branch::PayloadCommitVerification;
use crate::types::serde_helpers::null_to_default;
use crate::types::user::User;

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

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    fn test_time() -> OffsetDateTime {
        OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
            time::Time::from_hms(10, 0, 0).unwrap(),
        )
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
