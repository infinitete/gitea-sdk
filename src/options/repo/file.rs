// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::types::repository::{CommitDateOptions, Identity};
use crate::{Deserialize, Serialize};

// ── repo_file.go ────────────────────────────────────────────────

/// FileOptions options for all file APIs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for File Option.
pub struct FileOptions {
    pub message: String,
    #[serde(rename = "branch")]
    pub branch_name: String,
    #[serde(rename = "new_branch")]
    pub new_branch_name: String,
    pub author: Identity,
    pub committer: Identity,
    pub dates: CommitDateOptions,
    pub signoff: bool,
}

/// CreateFileOptions options for creating files
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create File Option.
pub struct CreateFileOptions {
    #[serde(flatten)]
    pub file_options: FileOptions,
    /// content must be base64 encoded
    pub content: String,
}

/// DeleteFileOptions options for deleting files
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Delete File Option.
pub struct DeleteFileOptions {
    #[serde(flatten)]
    pub file_options: FileOptions,
    /// sha is the SHA for the file that already exists
    pub sha: String,
}

/// UpdateFileOptions options for updating files
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Update File Option.
pub struct UpdateFileOptions {
    #[serde(flatten)]
    pub file_options: FileOptions,
    /// sha is the SHA for the file that already exists
    pub sha: String,
    /// content must be base64 encoded
    pub content: String,
    #[serde(rename = "from_path")]
    pub from_path: String,
}

// ── repo_file_ext.go ────────────────────────────────────────────

/// GetContentsExtOptions options for getting extended contents
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Get Contents Ext Option.
pub struct GetContentsExtOptions {
    pub r#ref: String,
    pub includes: String,
}
