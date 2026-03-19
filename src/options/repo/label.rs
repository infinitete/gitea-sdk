// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── repo_label.go ───────────────────────────────────────────────

/// ListLabelsOptions options for listing repository's labels
#[derive(Debug, Clone, Default)]
/// Options for List Labels Option.
pub struct ListLabelsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListLabelsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreateLabelOption options for creating a label
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Label Option.
pub struct CreateLabelOption {
    pub name: String,
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
}

impl CreateLabelOption {
    /// Validate this `CreateLabelOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        let color = self.color.trim_start_matches('#');
        if color.len() != 6 || !color.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(crate::Error::Validation("invalid color format".to_string()));
        }
        if self.name.trim().is_empty() {
            return Err(crate::Error::Validation(
                "empty name not allowed".to_string(),
            ));
        }
        Ok(())
    }
}

/// EditLabelOption options for editing a label
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Label Option.
pub struct EditLabelOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
    #[serde(
        rename = "is_archived",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_archived: Option<bool>,
}

impl EditLabelOption {
    /// Validate this `EditLabelOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if let Some(ref color) = self.color {
            let color = color.trim_start_matches('#');
            if color.len() != 6 || !color.chars().all(|c| c.is_ascii_hexdigit()) {
                return Err(crate::Error::Validation("invalid color format".to_string()));
            }
        }
        if let Some(ref name) = self.name
            && name.trim().is_empty()
        {
            return Err(crate::Error::Validation(
                "empty name not allowed".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_label_option_validate_success() {
        let opt = CreateLabelOption {
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_label_option_validate_invalid_color() {
        let opt = CreateLabelOption {
            name: "bug".to_string(),
            color: "red".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_label_option_validate_empty_name() {
        let opt = CreateLabelOption {
            name: String::new(),
            color: "ff0000".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_label_option_validate_color_with_hash() {
        let opt = CreateLabelOption {
            name: "bug".to_string(),
            color: "#00ff00".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_label_option_validate_success() {
        let opt = EditLabelOption {
            name: Some("new-name".to_string()),
            color: Some("abcdef".to_string()),
            description: None,
            exclusive: None,
            is_archived: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_label_option_validate_invalid_color() {
        let opt = EditLabelOption {
            name: None,
            color: Some("zzz".to_string()),
            description: None,
            exclusive: None,
            is_archived: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_label_option_validate_empty_name() {
        let opt = EditLabelOption {
            name: Some("   ".to_string()),
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
        };
        assert!(opt.validate().is_err());
    }
}
