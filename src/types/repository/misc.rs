// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Miscellaneous repository types (repo_collaborator.go, repo_mirror.go,
//! git_hook.go, repo_action_variable.go).

use crate::{Deserialize, Serialize};

use crate::types::user::User;

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
}
