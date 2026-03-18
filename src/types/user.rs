// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::serde_helpers::nullable_rfc3339;
use crate::types::enums::AccessTokenScope;

/// AccessToken represents an API access token
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Access Token payload type.
pub struct AccessToken {
    pub id: i64,
    pub name: String,
    #[serde(rename = "sha1")]
    pub token: String,
    #[serde(rename = "token_last_eight")]
    pub token_last_eight: String,
    #[serde(default)]
    pub scopes: Vec<AccessTokenScope>,
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated: Option<OffsetDateTime>,
}

/// UserHeatmapData represents the data needed to render a user's contribution heatmap.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// User Heatmap Data payload type.
pub struct UserHeatmapData {
    pub timestamp: i64,
    pub contributions: i64,
}

/// Email represents an email address belonging to a user
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Email payload type.
pub struct Email {
    pub email: String,
    pub verified: bool,
    pub primary: bool,
    #[serde(default, rename = "user_id", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// PublicKey represents a user key to push code to repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Public Key payload type.
pub struct PublicKey {
    pub id: i64,
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<OffsetDateTime>,
    #[serde(
        default,
        rename = "last_used_at",
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated: Option<OffsetDateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<User>,
    #[serde(default, rename = "read_only", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(default, rename = "key_type", skip_serializing_if = "Option::is_none")]
    pub key_type: Option<String>,
}

/// GPGKeyEmail represents an email attached to a GPGKey
#[derive(Debug, Clone, Serialize, Deserialize)]
/// GPGKey Email payload type.
pub struct GPGKeyEmail {
    pub email: String,
    pub verified: bool,
}

/// GPGKey represents a user GPG key to sign commit and tag in repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// GPGKey payload type.
pub struct GPGKey {
    pub id: i64,
    #[serde(rename = "primary_key_id")]
    pub primary_key_id: String,
    #[serde(rename = "key_id")]
    pub key_id: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(default)]
    pub emails: Vec<GPGKeyEmail>,
    #[serde(default)]
    pub subs_key: Vec<GPGKey>,
    #[serde(rename = "can_sign")]
    pub can_sign: bool,
    #[serde(rename = "can_encrypt_comms")]
    pub can_encrypt_comms: bool,
    #[serde(rename = "can_encrypt_storage")]
    pub can_encrypt_storage: bool,
    #[serde(rename = "can_certify")]
    pub can_certify: bool,
    pub verified: bool,
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<OffsetDateTime>,
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub expires: Option<OffsetDateTime>,
}

/// User represents a user
#[derive(Debug, Clone, Serialize, Deserialize)]
/// User payload type.
pub struct User {
    /// the user's id
    pub id: i64,
    /// the user's username
    #[serde(rename = "login")]
    pub user_name: String,
    /// The login_name of non local users (e.g. LDAP / OAuth / SMTP)
    #[serde(rename = "login_name")]
    pub login_name: String,
    /// The ID of the Authentication Source for non local users
    #[serde(rename = "source_id")]
    pub source_id: i64,
    /// the user's full name
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub email: String,
    /// URL to the user's avatar
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    /// URL to the user's profile
    #[serde(rename = "html_url")]
    pub html_url: String,
    /// User locale
    pub language: String,
    /// Is the user an administrator
    #[serde(rename = "is_admin")]
    pub is_admin: bool,
    /// Date and Time of last login
    #[serde(
        rename = "last_login",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_login: Option<OffsetDateTime>,
    /// Date and Time of user creation
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<OffsetDateTime>,
    /// Is user restricted
    pub restricted: bool,
    /// Is user active
    #[serde(rename = "active")]
    pub is_active: bool,
    /// Is user login prohibited
    #[serde(rename = "prohibit_login")]
    pub prohibit_login: bool,
    /// the user's location
    pub location: String,
    /// the user's website
    pub website: String,
    /// the user's description
    pub description: String,
    /// User visibility level option
    pub visibility: crate::types::enums::VisibleType,
    /// user counts
    #[serde(rename = "followers_count")]
    pub follower_count: i32,
    #[serde(rename = "following_count")]
    pub following_count: i32,
    #[serde(rename = "starred_repos_count")]
    pub starred_repo_count: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_round_trip() {
        let original = User {
            id: 1,
            user_name: "testuser".to_string(),
            login_name: "".to_string(),
            source_id: 0,
            full_name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            avatar_url: "https://example.com/avatar.png".to_string(),
            html_url: "https://gitea.example.com/testuser".to_string(),
            language: "en-US".to_string(),
            is_admin: false,
            last_login: None,
            created: None,
            restricted: false,
            is_active: true,
            prohibit_login: false,
            location: "".to_string(),
            website: "".to_string(),
            description: "".to_string(),
            visibility: crate::types::enums::VisibleType::Public,
            follower_count: 10,
            following_count: 5,
            starred_repo_count: 3,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: User = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.user_name, original.user_name);
        assert_eq!(restored.email, original.email);
    }

    #[test]
    fn test_email_round_trip() {
        let original = Email {
            email: "test@example.com".to_string(),
            verified: true,
            primary: true,
            user_id: Some(1),
            username: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Email = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.email, original.email);
        assert_eq!(restored.verified, original.verified);
    }

    #[test]
    fn test_user_heatmap_data_round_trip() {
        let original = UserHeatmapData {
            timestamp: 1_710_460_800,
            contributions: 12,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: UserHeatmapData = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.timestamp, original.timestamp);
        assert_eq!(restored.contributions, original.contributions);
    }

    #[test]
    fn test_gpg_key_email_round_trip() {
        let original = GPGKeyEmail {
            email: "test@example.com".to_string(),
            verified: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GPGKeyEmail = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.email, original.email);
    }
}
