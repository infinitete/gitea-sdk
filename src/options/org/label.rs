// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── org_label.go ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ListOrgLabelsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgLabelsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateOrgLabelOption {
    pub name: String,
    pub color: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
}

impl CreateOrgLabelOption {
    pub fn validate(&self) -> crate::Result<()> {
        let color = self.color.strip_prefix('#').unwrap_or(&self.color);
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditOrgLabelOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_org_label_option_validate_success() {
        let opt = CreateOrgLabelOption {
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: None,
            exclusive: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_org_label_option_validate_invalid_color() {
        let opt = CreateOrgLabelOption {
            name: "bug".to_string(),
            color: "red".to_string(),
            description: None,
            exclusive: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_org_label_option_validate_empty_name() {
        let opt = CreateOrgLabelOption {
            name: String::new(),
            color: "ff0000".to_string(),
            description: None,
            exclusive: None,
        };
        assert!(opt.validate().is_err());
    }
}
