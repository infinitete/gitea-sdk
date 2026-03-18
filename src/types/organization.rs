// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};

/// Organization represents an organization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Organization payload type.
pub struct Organization {
    pub id: i64,
    pub name: String,
    #[serde(rename = "username")]
    pub user_name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub email: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    pub description: String,
    pub website: String,
    pub location: String,
    pub visibility: String,
    #[serde(rename = "repo_admin_change_team_access")]
    pub repo_admin_change_team_access: bool,
}

/// OrgPermissions represents the permissions for a user in an organization
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Org Permissions payload type.
pub struct OrgPermissions {
    #[serde(rename = "can_create_repository")]
    pub can_create_repository: bool,
    #[serde(rename = "can_read")]
    pub can_read: bool,
    #[serde(rename = "can_write")]
    pub can_write: bool,
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    #[serde(rename = "is_owner")]
    pub is_owner: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_organization_round_trip() {
        let original = Organization {
            id: 1,
            name: "myorg".to_string(),
            user_name: "myorg".to_string(),
            full_name: "My Organization".to_string(),
            email: "org@example.com".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            description: "An organization".to_string(),
            website: "https://example.com".to_string(),
            location: "World".to_string(),
            visibility: "public".to_string(),
            repo_admin_change_team_access: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Organization = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
    }

    #[test]
    fn test_org_permissions_round_trip() {
        let original = OrgPermissions {
            can_create_repository: true,
            can_read: true,
            can_write: false,
            is_admin: false,
            is_owner: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: OrgPermissions = serde_json::from_str(&json).unwrap();
        assert_eq!(
            restored.can_create_repository,
            original.can_create_repository
        );
        assert_eq!(restored.is_owner, original.is_owner);
    }
}
