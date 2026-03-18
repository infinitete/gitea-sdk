// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for organization API endpoints.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::{AccessMode, RepoUnitType, VisibleType};
use crate::{Deserialize, Serialize};

// ── org.go ──────────────────────────────────────────────────────────────

/// ListOrgsOptions options for listing organizations
#[derive(Debug, Clone, Default)]
/// Options for List Orgs Option.
pub struct ListOrgsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreateOrgOption options for creating an organization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Org Option.
pub struct CreateOrgOption {
    #[serde(rename = "username")]
    pub name: String,
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibleType>,
    #[serde(
        rename = "repo_admin_change_team_access",
        skip_serializing_if = "Option::is_none"
    )]
    pub repo_admin_change_team_access: Option<bool>,
}

impl CreateOrgOption {
    /// Validate this `CreateOrgOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::Validation("org name is required".to_string()));
        }
        if let Some(vis) = &self.visibility
            && !matches!(
                vis,
                VisibleType::Public | VisibleType::Limited | VisibleType::Private
            )
        {
            return Err(crate::Error::Validation(
                "invalid visibility option".to_string(),
            ));
        }
        Ok(())
    }
}

/// EditOrgOption options for editing an organization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Edit Org Option.
pub struct EditOrgOption {
    #[serde(rename = "full_name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibleType>,
    #[serde(
        rename = "repo_admin_change_team_access",
        skip_serializing_if = "Option::is_none"
    )]
    pub repo_admin_change_team_access: Option<bool>,
}

impl EditOrgOption {
    /// Validate this `EditOrgOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if let Some(vis) = &self.visibility
            && !matches!(
                vis,
                VisibleType::Public | VisibleType::Limited | VisibleType::Private
            )
        {
            return Err(crate::Error::Validation(
                "invalid visibility option".to_string(),
            ));
        }
        Ok(())
    }
}

// ── org_team.go ─────────────────────────────────────────────────────────

/// ListTeamsOptions options for listing teams
#[derive(Debug, Clone, Default)]
/// Options for List Teams Option.
pub struct ListTeamsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListTeamsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// SearchTeamsOptions options for searching teams
#[derive(Debug, Clone, Default)]
/// Options for Search Teams Option.
pub struct SearchTeamsOptions {
    pub list_options: ListOptions,
    pub query: String,
    pub include_description: bool,
}

impl QueryEncode for SearchTeamsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        out.push_str(&format!(
            "&q={}&include_desc={}",
            percent_encode(&self.query),
            self.include_description
        ));
        out
    }
}

/// CreateTeamOption options for creating a team
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Team Option.
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
    /// Validate this `CreateTeamOption` payload.
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

/// EditTeamOption options for editing a team
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Edit Team Option.
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
    /// Validate this `EditTeamOption` payload.
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

/// ListTeamMembersOptions options for listing team's members
#[derive(Debug, Clone, Default)]
/// Options for List Team Members Option.
pub struct ListTeamMembersOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListTeamMembersOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListTeamRepositoriesOptions options for listing team's repositories
#[derive(Debug, Clone, Default)]
/// Options for List Team Repositories Option.
pub struct ListTeamRepositoriesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListTeamRepositoriesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── org_member.go ───────────────────────────────────────────────────────

/// ListOrgMembershipOption list OrgMembership options
#[derive(Debug, Clone, Default)]
/// Options for List Org Membership Option.
pub struct ListOrgMembershipOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgMembershipOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── org_label.go ────────────────────────────────────────────────────────

/// ListOrgLabelsOptions options for listing organization labels
#[derive(Debug, Clone, Default)]
/// Options for List Org Labels Option.
pub struct ListOrgLabelsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgLabelsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreateOrgLabelOption options for creating an organization label
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Org Label Option.
pub struct CreateOrgLabelOption {
    /// Name of the label
    pub name: String,
    /// Color of the label in hex format without #
    pub color: String,
    /// Description of the label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this is an exclusive label
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
}

impl CreateOrgLabelOption {
    /// Validate this `CreateOrgLabelOption` payload.
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

/// EditOrgLabelOption options for editing an organization label
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Options for Edit Org Label Option.
pub struct EditOrgLabelOption {
    /// New name of the label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// New color of the label in hex format without #
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// New description of the label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this is an exclusive label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive: Option<bool>,
}

// ── org_action.go ───────────────────────────────────────────────────────

/// ListOrgActionSecretOption list OrgActionSecret options
#[derive(Debug, Clone, Default)]
/// Options for List Org Action Secret Option.
pub struct ListOrgActionSecretOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgActionSecretOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListOrgActionVariableOption lists OrgActionVariable options
#[derive(Debug, Clone, Default)]
/// Options for List Org Action Variable Option.
pub struct ListOrgActionVariableOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgActionVariableOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// OrgActionVariable represents an organization action variable
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Org Action Variable.
pub struct OrgActionVariable {
    #[serde(rename = "owner_id")]
    pub owner_id: i64,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    pub name: String,
    pub data: String,
    pub description: String,
}

/// CreateOrgActionVariableOption options for creating an org action variable
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Org Action Variable Option.
pub struct CreateOrgActionVariableOption {
    /// Name is the name of the variable
    pub name: String,
    /// Value is the value of the variable
    pub value: String,
    /// Description is the description of the variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateOrgActionVariableOption {
    /// Validate this `CreateOrgActionVariableOption` payload.
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

/// UpdateOrgActionVariableOption options for updating an org action variable
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Update Org Action Variable Option.
pub struct UpdateOrgActionVariableOption {
    /// Value is the new value of the variable
    pub value: String,
    /// Description is the new description of the variable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl UpdateOrgActionVariableOption {
    /// Validate this `UpdateOrgActionVariableOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.value.is_empty() {
            return Err(crate::Error::Validation("value required".to_string()));
        }
        Ok(())
    }
}

/// CreateSecretOption options for creating a secret
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Secret Option.
pub struct CreateSecretOption {
    /// Name is the name of the secret
    pub name: String,
    /// Data is the data of the secret
    pub data: String,
    /// Description is the description of the secret
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl CreateSecretOption {
    /// Validate this `CreateSecretOption` payload.
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

// ── org_block.go ────────────────────────────────────────────────────────

/// ListOrgBlocksOptions options for listing organization blocks
#[derive(Debug, Clone, Default)]
/// Options for List Org Blocks Option.
pub struct ListOrgBlocksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgBlocksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── org_social.go ───────────────────────────────────────────────────────

/// RenameOrgOption options for renaming an organization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Rename Org Option.
pub struct RenameOrgOption {
    #[serde(rename = "new_name")]
    pub new_name: String,
}

/// ListOrgActivityFeedsOptions options for listing organization activity feeds
#[derive(Debug, Clone, Default)]
/// Options for List Org Activity Feeds Option.
pub struct ListOrgActivityFeedsOptions {
    pub list_options: ListOptions,
    pub date: Option<String>,
}

impl QueryEncode for ListOrgActivityFeedsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(date) = &self.date {
            out.push_str(&format!("&date={}", percent_encode(date)));
        }
        out
    }
}

/// ListTeamActivityFeedsOptions options for listing team activity feeds
#[derive(Debug, Clone, Default)]
/// Options for List Team Activity Feeds Option.
pub struct ListTeamActivityFeedsOptions {
    pub list_options: ListOptions,
    pub date: Option<String>,
}

impl QueryEncode for ListTeamActivityFeedsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(date) = &self.date {
            out.push_str(&format!("&date={}", percent_encode(date)));
        }
        out
    }
}

// ── helpers ─────────────────────────────────────────────────────────────

fn percent_encode(s: &str) -> String {
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
    utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_org_option_validate_success() {
        let opt = CreateOrgOption {
            name: "myorg".to_string(),
            ..Default::default()
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_org_option_validate_empty_name() {
        let opt = CreateOrgOption {
            name: String::new(),
            ..Default::default()
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_org_option_validate_invalid_visibility() {
        let opt = CreateOrgOption {
            name: "myorg".to_string(),
            visibility: Some(VisibleType::Unknown),
            ..Default::default()
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_org_option_validate_success() {
        let opt = EditOrgOption {
            visibility: Some(VisibleType::Public),
            ..Default::default()
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_org_option_validate_invalid_visibility() {
        let opt = EditOrgOption {
            visibility: Some(VisibleType::Unknown),
            ..Default::default()
        };
        assert!(opt.validate().is_err());
    }

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
