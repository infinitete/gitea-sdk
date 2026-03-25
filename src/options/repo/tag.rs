// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── repo_tag.go ─────────────────────────────────────────────────

/// `ListRepoTagsOptions` options for listing a repository's tags
#[derive(Debug, Clone, Default)]
/// Options for List Repo Tags Option.
pub struct ListRepoTagsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoTagsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// `ListRepoTagProtectionsOptions` options for listing tag protections
#[derive(Debug, Clone, Default)]
/// Options for List Repo Tag Protections Option.
pub struct ListRepoTagProtectionsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListRepoTagProtectionsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// `CreateTagOption` options when creating a tag
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Tag Option.
pub struct CreateTagOption {
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    pub message: String,
    pub target: String,
}

impl CreateTagOption {
    /// Validate this `CreateTagOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.tag_name.is_empty() {
            return Err(crate::Error::Validation("TagName is required".to_string()));
        }
        Ok(())
    }
}

// ── repo_tag_protection.go ──────────────────────────────────────

/// `CreateTagProtectionOption` options for creating a tag protection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Tag Protection Option.
pub struct CreateTagProtectionOption {
    #[serde(rename = "name_pattern")]
    pub name_pattern: String,
    #[serde(default)]
    pub whitelist_usernames: Vec<String>,
    #[serde(default)]
    pub whitelist_teams: Vec<String>,
}

/// `EditTagProtectionOption` options for editing a tag protection
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Tag Protection Option.
pub struct EditTagProtectionOption {
    #[serde(
        rename = "name_pattern",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub name_pattern: Option<String>,
    #[serde(default)]
    pub whitelist_usernames: Vec<String>,
    #[serde(default)]
    pub whitelist_teams: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tag_option_validate_success() {
        let opt = CreateTagOption {
            tag_name: "v1.0".to_string(),
            message: String::new(),
            target: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_tag_option_validate_empty_tag_name() {
        let opt = CreateTagOption {
            tag_name: String::new(),
            message: String::new(),
            target: String::new(),
        };
        assert!(opt.validate().is_err());
    }
}
