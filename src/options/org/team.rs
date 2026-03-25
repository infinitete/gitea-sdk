// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode, push_query_segment};
use crate::types::enums::{AccessMode, RepoUnitType};
use crate::{Deserialize, Serialize};

use super::percent_encode;

// ── org_team.go ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ListTeamsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListTeamsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchTeamsOptions {
    pub list_options: ListOptions,
    pub query: String,
    pub include_description: bool,
}

impl QueryEncode for SearchTeamsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        push_query_segment(
            &mut out,
            &format!(
                "q={}&include_desc={}",
                percent_encode(&self.query),
                self.include_description
            ),
        );
        out
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTeamOption {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<AccessMode>,
    #[serde(
        rename = "can_create_org_repo",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_org_repo: Option<bool>,
    #[serde(
        rename = "includes_all_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub includes_all_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub units: Vec<RepoUnitType>,
}

impl CreateTeamOption {
    pub fn validate(&self) -> crate::Result<()> {
        let perm = self.permission.unwrap_or(AccessMode::Read);
        match perm {
            AccessMode::Read | AccessMode::Write | AccessMode::Admin => {}
            _ => {
                return Err(crate::Error::Validation(
                    "permission mode invalid".to_string(),
                ));
            }
        }
        if self.name.is_empty() {
            return Err(crate::Error::Validation("name required".to_string()));
        }
        if self.name.len() > 255 {
            return Err(crate::Error::Validation("name too long".to_string()));
        }
        if let Some(desc) = &self.description
            && desc.len() > 255
        {
            return Err(crate::Error::Validation("description too long".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditTeamOption {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<AccessMode>,
    #[serde(
        rename = "can_create_org_repo",
        skip_serializing_if = "Option::is_none"
    )]
    pub can_create_org_repo: Option<bool>,
    #[serde(
        rename = "includes_all_repositories",
        skip_serializing_if = "Option::is_none"
    )]
    pub includes_all_repositories: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub units: Vec<RepoUnitType>,
}

impl EditTeamOption {
    pub fn validate(&self) -> crate::Result<()> {
        let perm = self.permission.unwrap_or(AccessMode::Read);
        match perm {
            AccessMode::Read | AccessMode::Write | AccessMode::Admin => {}
            _ => {
                return Err(crate::Error::Validation(
                    "permission mode invalid".to_string(),
                ));
            }
        }
        if self.name.is_empty() {
            return Err(crate::Error::Validation("name required".to_string()));
        }
        if self.name.len() > 30 {
            return Err(crate::Error::Validation("name too long".to_string()));
        }
        if let Some(desc) = &self.description
            && desc.len() > 255
        {
            return Err(crate::Error::Validation("description too long".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListTeamMembersOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListTeamMembersOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListTeamRepositoriesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListTeamRepositoriesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_team_option_validate_success() {
        let opt = CreateTeamOption {
            name: "core".to_string(),
            description: Some("Core team".to_string()),
            permission: Some(AccessMode::Read),
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_team_option_validate_empty_name() {
        let opt = CreateTeamOption {
            name: String::new(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_team_option_validate_name_too_long() {
        let opt = CreateTeamOption {
            name: "a".repeat(256),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_team_option_validate_description_too_long() {
        let opt = CreateTeamOption {
            name: "core".to_string(),
            description: Some("d".repeat(256)),
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_team_option_validate_invalid_permission() {
        let opt = CreateTeamOption {
            name: "core".to_string(),
            permission: Some(AccessMode::Owner),
            description: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_team_option_validate_success() {
        let opt = EditTeamOption {
            name: "core".to_string(),
            description: Some("Core team".to_string()),
            permission: Some(AccessMode::Read),
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_team_option_validate_empty_name() {
        let opt = EditTeamOption {
            name: String::new(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_team_option_validate_name_too_long() {
        let opt = EditTeamOption {
            name: "a".repeat(31),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }
}
