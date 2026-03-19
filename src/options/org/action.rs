// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── org_action.go ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ListOrgActionSecretOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgActionSecretOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListOrgActionVariableOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgActionVariableOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrgActionVariable {
    #[serde(rename = "owner_id")]
    pub owner_id: i64,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    pub name: String,
    pub data: String,
    pub description: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateOrgActionVariableOption {
    pub name: String,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateOrgActionVariableOption {
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::Validation("name required".to_string()));
        }
        if self.name.len() > 30 {
            return Err(crate::Error::Validation("name too long".to_string()));
        }
        if self.value.is_empty() {
            return Err(crate::Error::Validation("value required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateOrgActionVariableOption {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateOrgActionVariableOption {
    pub fn validate(&self) -> crate::Result<()> {
        if self.value.is_empty() {
            return Err(crate::Error::Validation("value required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateSecretOption {
    pub name: String,
    pub data: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateSecretOption {
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::Validation("name required".to_string()));
        }
        if self.name.len() > 30 {
            return Err(crate::Error::Validation("name too long".to_string()));
        }
        if self.data.is_empty() {
            return Err(crate::Error::Validation("data required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_org_action_variable_option_validate_success() {
        let opt = CreateOrgActionVariableOption {
            name: "VAR".to_string(),
            value: "value".to_string(),
            description: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_org_action_variable_option_validate_empty_name() {
        let opt = CreateOrgActionVariableOption {
            name: String::new(),
            value: "value".to_string(),
            description: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_org_action_variable_option_validate_name_too_long() {
        let opt = CreateOrgActionVariableOption {
            name: "a".repeat(31),
            value: "value".to_string(),
            description: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_org_action_variable_option_validate_empty_value() {
        let opt = CreateOrgActionVariableOption {
            name: "VAR".to_string(),
            value: String::new(),
            description: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_update_org_action_variable_option_validate_success() {
        let opt = UpdateOrgActionVariableOption {
            value: "new-value".to_string(),
            description: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_update_org_action_variable_option_validate_empty_value() {
        let opt = UpdateOrgActionVariableOption {
            value: String::new(),
            description: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_secret_option_validate_success() {
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_secret_option_validate_empty_name() {
        let opt = CreateSecretOption {
            name: String::new(),
            data: "secret-data".to_string(),
            description: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_secret_option_validate_empty_data() {
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: String::new(),
            description: None,
        };
        assert!(opt.validate().is_err());
    }
}
