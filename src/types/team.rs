// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for organization teams.

use crate::types::enums::{AccessMode, RepoUnitType};
use crate::{Deserialize, Serialize};

use super::organization::Organization;
use super::serde_helpers::null_to_default;

/// Team represents a team in an organization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Team payload type.
pub struct Team {
    pub id: i64,
    pub name: String,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<Organization>,
    pub permission: AccessMode,
    #[serde(rename = "can_create_org_repo")]
    pub can_create_org_repo: bool,
    #[serde(rename = "includes_all_repositories")]
    pub includes_all_repositories: bool,
    #[serde(default, deserialize_with = "null_to_default")]
    pub units: Vec<RepoUnitType>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_team_round_trip() {
        let original = Team {
            id: 1,
            name: "developers".to_string(),
            description: "Dev team".to_string(),
            organization: None,
            permission: AccessMode::Write,
            can_create_org_repo: true,
            includes_all_repositories: false,
            units: vec![RepoUnitType::Code, RepoUnitType::Issues],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Team = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.permission, original.permission);
        assert_eq!(restored.units.len(), 2);
    }

    #[test]
    fn test_team_empty_units() {
        let json = r#"{
            "id": 1,
            "name": "test",
            "description": "",
            "permission": "read",
            "can_create_org_repo": false,
            "includes_all_repositories": false,
            "units": []
        }"#;
        let team: Team = serde_json::from_str(json).unwrap();
        assert!(team.units.is_empty());
    }
}
