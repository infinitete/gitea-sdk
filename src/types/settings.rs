// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalUISettings {
    #[serde(rename = "default_theme")]
    pub default_theme: String,
    #[serde(rename = "allowed_reactions", default)]
    pub allowed_reactions: Vec<String>,
    #[serde(rename = "custom_emojis", default)]
    pub custom_emojis: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct GlobalAttachmentSettings {
    pub enabled: bool,
    #[serde(rename = "allowed_types")]
    pub allowed_types: String,
    #[serde(rename = "max_size")]
    pub max_size: i64,
    #[serde(rename = "max_files")]
    pub max_files: i32,
}
