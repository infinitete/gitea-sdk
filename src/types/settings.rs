// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for global Gitea instance settings (UI, repo, API, attachments).

use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Global UISettings payload type.
pub struct GlobalUISettings {
    #[serde(rename = "default_theme")]
    pub default_theme: String,
    #[serde(rename = "allowed_reactions", default)]
    pub allowed_reactions: Vec<String>,
    #[serde(rename = "custom_emojis", default)]
    pub custom_emojis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Global Repo Settings payload type.
pub struct GlobalRepoSettings {
    #[serde(rename = "mirrors_disabled")]
    pub mirrors_disabled: bool,
    #[serde(rename = "http_git_disabled")]
    pub http_git_disabled: bool,
    #[serde(rename = "migrations_disabled")]
    pub migrations_disabled: bool,
    #[serde(rename = "stars_disabled")]
    pub stars_disabled: bool,
    #[serde(rename = "time_tracking_disabled")]
    pub time_tracking_disabled: bool,
    #[serde(rename = "lfs_disabled")]
    pub lfs_disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Global APISettings payload type.
pub struct GlobalAPISettings {
    #[serde(rename = "max_response_items")]
    pub max_response_items: i32,
    #[serde(rename = "default_paging_num")]
    pub default_paging_num: i32,
    #[serde(rename = "default_git_trees_per_page")]
    pub default_git_trees_per_page: i32,
    #[serde(rename = "default_max_blob_size")]
    pub default_max_blob_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Global Attachment Settings payload type.
pub struct GlobalAttachmentSettings {
    pub enabled: bool,
    #[serde(rename = "allowed_types")]
    pub allowed_types: String,
    #[serde(rename = "max_size")]
    pub max_size: i64,
    #[serde(rename = "max_files")]
    pub max_files: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_ui_settings_round_trip() {
        let original = GlobalUISettings {
            default_theme: "gitea-auto".to_string(),
            allowed_reactions: vec!["+1".to_string(), "-1".to_string(), "laugh".to_string()],
            custom_emojis: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GlobalUISettings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.default_theme, "gitea-auto");
        assert_eq!(restored.allowed_reactions.len(), 3);
        assert!(restored.custom_emojis.is_empty());
    }

    #[test]
    fn test_global_repo_settings_round_trip() {
        let original = GlobalRepoSettings {
            mirrors_disabled: false,
            http_git_disabled: false,
            migrations_disabled: false,
            stars_disabled: false,
            time_tracking_disabled: false,
            lfs_disabled: false,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GlobalRepoSettings = serde_json::from_str(&json).unwrap();
        assert!(!restored.mirrors_disabled);
        assert!(!restored.http_git_disabled);
    }

    #[test]
    fn test_global_api_settings_round_trip() {
        let original = GlobalAPISettings {
            max_response_items: 50,
            default_paging_num: 30,
            default_git_trees_per_page: 1000,
            default_max_blob_size: 10485760,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GlobalAPISettings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.max_response_items, 50);
        assert_eq!(restored.default_paging_num, 30);
    }

    #[test]
    fn test_global_attachment_settings_round_trip() {
        let original = GlobalAttachmentSettings {
            enabled: true,
            allowed_types: ".png,.jpg".to_string(),
            max_size: 4194304,
            max_files: 5,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GlobalAttachmentSettings = serde_json::from_str(&json).unwrap();
        assert!(restored.enabled);
        assert_eq!(restored.max_files, 5);
    }
}
