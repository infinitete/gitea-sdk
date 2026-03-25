// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode, push_query_segment};
use crate::types::enums::VisibleType;
use crate::{Deserialize, Serialize};

use super::percent_encode;

// ── org.go ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ListOrgsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

// ── org_social.go ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameOrgOption {
    #[serde(rename = "new_name")]
    pub new_name: String,
}

#[derive(Debug, Clone, Default)]
pub struct ListOrgActivityFeedsOptions {
    pub list_options: ListOptions,
    pub date: Option<String>,
}

impl QueryEncode for ListOrgActivityFeedsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(date) = &self.date {
            push_query_segment(&mut out, &format!("date={}", percent_encode(date)));
        }
        out
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListTeamActivityFeedsOptions {
    pub list_options: ListOptions,
    pub date: Option<String>,
}

impl QueryEncode for ListTeamActivityFeedsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(date) = &self.date {
            push_query_segment(&mut out, &format!("date={}", percent_encode(date)));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::VisibleType;

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
}
