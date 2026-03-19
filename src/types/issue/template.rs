// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Issue template types: IssueTemplate, IssueFormElement, IssueFormElementAttributes, IssueFormElementValidations.

use crate::{Deserialize, Serialize};

use crate::types::enums::IssueFormElementType;
use crate::types::serde_helpers::null_to_default;

// ── issue_template.go ────────────────────────────────────────────

/// IssueTemplate provides metadata and content on an issue template.
/// There are two types of issue templates: .Markdown- and .Form-based.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Template payload type.
pub struct IssueTemplate {
    pub name: String,
    pub about: String,
    #[serde(rename = "file_name")]
    pub filename: String,
    pub title: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub labels: Vec<String>,
    #[serde(default)]
    pub r#ref: String,
    /// If non-nil, this is a form-based template
    #[serde(default, deserialize_with = "null_to_default")]
    pub form: Vec<IssueFormElement>,
    /// Should only be used when .Form is nil.
    #[serde(default, rename = "content")]
    pub markdown_content: String,
}

/// IssueFormElement describes a part of a IssueTemplate form
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Form Element payload type.
pub struct IssueFormElement {
    pub id: String,
    pub r#type: IssueFormElementType,
    pub attributes: IssueFormElementAttributes,
    #[serde(default)]
    pub validations: IssueFormElementValidations,
}

/// IssueFormElementAttributes contains the combined set of attributes available on all element types.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Issue Form Element Attributes payload type.
pub struct IssueFormElementAttributes {
    /// A brief description of the expected user input
    pub label: String,
    /// Options for dropdown and checkboxes
    #[serde(default, deserialize_with = "null_to_default")]
    pub options: Vec<String>,
    /// Pre-filled value for markdown, textarea, input
    #[serde(default)]
    pub value: String,
    /// Description for textarea, input, dropdown, checkboxes
    #[serde(default)]
    pub description: String,
    /// Placeholder for textarea, input
    #[serde(default)]
    pub placeholder: String,
    /// Syntax highlighting language for textarea
    #[serde(default, rename = "render")]
    pub syntax_highlighting: String,
    /// Multiple selection for dropdown
    #[serde(default)]
    pub multiple: bool,
}

/// IssueFormElementValidations contains the combined set of validations available on all element types.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Issue Form Element Validations payload type.
pub struct IssueFormElementValidations {
    #[serde(default)]
    pub required: bool,
    #[serde(default, rename = "is_number")]
    pub is_number: bool,
    #[serde(default)]
    pub regex: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_issue_template_round_trip() {
        let original = IssueTemplate {
            name: "Bug Report".to_string(),
            about: "File a bug".to_string(),
            filename: "bug_report.md".to_string(),
            title: "Bug: ".to_string(),
            labels: vec!["bug".to_string()],
            r#ref: String::new(),
            form: vec![],
            markdown_content: "Describe the bug...".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: IssueTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.labels.len(), 1);
        assert!(restored.form.is_empty());
    }

    #[test]
    fn test_issue_form_element_round_trip() {
        let original = IssueFormElement {
            id: "title".to_string(),
            r#type: IssueFormElementType::Input,
            attributes: IssueFormElementAttributes {
                label: "Title".to_string(),
                options: vec![],
                value: String::new(),
                description: "Bug title".to_string(),
                placeholder: String::new(),
                syntax_highlighting: String::new(),
                multiple: false,
            },
            validations: IssueFormElementValidations {
                required: true,
                is_number: false,
                regex: String::new(),
            },
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: IssueFormElement = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, "title");
        assert!(restored.validations.required);
    }
}
