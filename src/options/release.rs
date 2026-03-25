// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for release and attachment API endpoints.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── ListReleasesOptions (release.go) ─────────────────────────────────

/// `ListReleasesOptions` options for listing repository's releases
#[derive(Debug, Clone, Default)]
/// Options for List Releases Option.
pub struct ListReleasesOptions {
    pub list_options: ListOptions,
    pub is_draft: Option<bool>,
    pub is_pre_release: Option<bool>,
}

impl QueryEncode for ListReleasesOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(draft) = self.is_draft {
            out.push_str(&format!("&draft={draft}"));
        }
        if let Some(pre) = self.is_pre_release {
            out.push_str(&format!("&pre-release={pre}"));
        }
        out
    }
}

// ── CreateReleaseOption (release.go) ────────────────────────────────

/// `CreateReleaseOption` options when creating a release
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Release Option.
pub struct CreateReleaseOption {
    /// `tag_name`
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    /// `target_commitish`
    #[serde(rename = "target_commitish", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// body
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// draft
    #[serde(default)]
    pub is_draft: bool,
    /// prerelease
    #[serde(default)]
    pub is_prerelease: bool,
}

impl CreateReleaseOption {
    /// Validate the `CreateReleaseOption` struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.tag_name.trim().is_empty() {
            return Err(crate::Error::Validation("tag_name is required".to_string()));
        }
        if let Some(ref title) = self.title
            && title.trim().is_empty()
        {
            return Err(crate::Error::Validation("title is empty".to_string()));
        }
        Ok(())
    }
}

// ── EditReleaseOption (release.go) ──────────────────────────────────

/// `EditReleaseOption` options when editing a release
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Edit Release Option.
pub struct EditReleaseOption {
    /// `tag_name`
    #[serde(rename = "tag_name", skip_serializing_if = "Option::is_none")]
    pub tag_name: Option<String>,
    /// `target_commitish`
    #[serde(rename = "target_commitish", skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// name
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// body
    #[serde(rename = "body", skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// draft
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_draft: Option<bool>,
    /// prerelease
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_prerelease: Option<bool>,
}

// ── ListReleaseAttachmentsOptions (attachment.go) ───────────────────

/// `ListReleaseAttachmentsOptions` options for listing release's attachments
#[derive(Debug, Clone, Default)]
/// Options for List Release Attachments Option.
pub struct ListReleaseAttachmentsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListReleaseAttachmentsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── EditAttachmentOption (attachment.go) ─────────────────────────────

/// `EditAttachmentOptions` options for editing attachments
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Attachment Option.
pub struct EditAttachmentOption {
    /// name
    pub name: String,
}

impl EditAttachmentOption {
    /// Validate the `EditAttachmentOption` struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::Error::Validation("name is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_release_option_validate_success() {
        let opt = CreateReleaseOption {
            tag_name: "v1.0.0".to_string(),
            target: None,
            title: None,
            note: None,
            is_draft: false,
            is_prerelease: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_release_option_validate_empty_tag_name() {
        let opt = CreateReleaseOption {
            tag_name: String::new(),
            target: None,
            title: None,
            note: None,
            is_draft: false,
            is_prerelease: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_release_option_validate_empty_title() {
        let opt = CreateReleaseOption {
            tag_name: "v1.0.0".to_string(),
            target: None,
            title: Some("   ".to_string()),
            note: None,
            is_draft: false,
            is_prerelease: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_attachment_option_validate_success() {
        let opt = EditAttachmentOption {
            name: "file.zip".to_string(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_attachment_option_validate_empty_name() {
        let opt = EditAttachmentOption {
            name: String::new(),
        };
        assert!(opt.validate().is_err());
    }
}
