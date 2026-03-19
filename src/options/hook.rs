// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for webhook API endpoints.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

use crate::types::enums::HookType;

#[derive(Debug, Clone, Default)]
/// Options for List Hooks Option.
pub struct ListHooksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListHooksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Hook Option.
pub struct CreateHookOption {
    #[serde(rename = "type")]
    pub hook_type: HookType,
    #[serde(default)]
    pub config: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(rename = "branch_filter", skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub active: bool,
    #[serde(
        rename = "authorization_header",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_header: Option<String>,
}

fn is_false(b: &bool) -> bool {
    !b
}

impl CreateHookOption {
    /// Validate this `CreateHookOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if std::mem::discriminant(&self.hook_type) == std::mem::discriminant(&HookType::Unknown) {
            return Err(crate::Error::Validation("hook type needed".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Edit Hook Option.
pub struct EditHookOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "branch_filter", skip_serializing_if = "Option::is_none")]
    pub branch_filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        rename = "authorization_header",
        skip_serializing_if = "Option::is_none"
    )]
    pub authorization_header: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hook_option_validate_success() {
        let opt = CreateHookOption {
            hook_type: HookType::Gitea,
            config: std::collections::HashMap::new(),
            events: Vec::new(),
            branch_filter: None,
            active: false,
            authorization_header: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_hook_option_validate_unknown_type() {
        let opt = CreateHookOption {
            hook_type: HookType::Unknown,
            config: std::collections::HashMap::new(),
            events: Vec::new(),
            branch_filter: None,
            active: false,
            authorization_header: None,
        };
        assert!(opt.validate().is_err());
    }
}
