// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for user API endpoints.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::AccessTokenScope;
use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
/// Options for List Emails Option.
pub struct ListEmailsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListEmailsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Email Option.
pub struct CreateEmailOption {
    /// email addresses to add
    pub emails: Vec<String>,
}

impl CreateEmailOption {
    /// Validate this `CreateEmailOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.emails.is_empty() {
            return Err(crate::Error::Validation(
                "at least one email is required".to_string(),
            ));
        }
        for email in &self.emails {
            if email.is_empty() {
                return Err(crate::Error::Validation(
                    "email addresses must not be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Delete Email Option.
pub struct DeleteEmailOption {
    /// email addresses to delete
    pub emails: Vec<String>,
}

impl DeleteEmailOption {
    /// Validate this `DeleteEmailOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.emails.is_empty() {
            return Err(crate::Error::Validation(
                "at least one email is required".to_string(),
            ));
        }
        for email in &self.emails {
            if email.is_empty() {
                return Err(crate::Error::Validation(
                    "email addresses must not be empty".to_string(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
/// Options for List Public Keys Option.
pub struct ListPublicKeysOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPublicKeysOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Key Option.
pub struct CreateKeyOption {
    /// Title of the key to add
    pub title: String,
    /// An armored SSH key to add
    pub key: String,
    /// Describe if the key has only read access or read/write
    #[serde(default)]
    pub read_only: bool,
}

impl CreateKeyOption {
    /// Validate this `CreateKeyOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.key.is_empty() {
            return Err(crate::Error::Validation("key is required".to_string()));
        }
        if self.title.is_empty() {
            return Err(crate::Error::Validation("title is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
/// Options for List Followers Option.
pub struct ListFollowersOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListFollowersOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
/// Options for List Following Option.
pub struct ListFollowingOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListFollowingOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
/// Options for List Access Tokens Option.
pub struct ListAccessTokensOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListAccessTokensOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Access Token Option.
pub struct CreateAccessTokenOption {
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<AccessTokenScope>,
}

impl CreateAccessTokenOption {
    /// Validate this `CreateAccessTokenOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::Validation("name is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for User Settings Option.
pub struct UserSettingsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "diff_view_style")]
    pub diff_view_style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hide_email")]
    pub hide_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hide_activity")]
    pub hide_activity: Option<bool>,
}

#[derive(Debug, Clone, Default)]
/// Options for List User Blocks Option.
pub struct ListUserBlocksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListUserBlocksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
/// Options for Search Users Option.
pub struct SearchUsersOption {
    pub list_options: ListOptions,
    pub key_word: String,
    pub uid: i64,
}

impl QueryEncode for SearchUsersOption {
    fn query_encode(&self) -> String {
        let mut out = String::new();
        let defaulted = self.list_options.with_defaults();
        if defaulted.page == Some(0) {
            out.push_str("page=0&limit=0");
        } else if let Some(page) = defaulted.page {
            out.push_str(&format!("page={page}"));
            if let Some(size) = defaulted.page_size {
                out.push_str(&format!("&limit={size}"));
            }
        }
        if !self.key_word.is_empty() {
            out.push_str(&format!("&q={}", urlencoding(self.key_word.as_bytes())));
        }
        if self.uid > 0 {
            out.push_str(&format!("&uid={}", self.uid));
        }
        out
    }
}

fn urlencoding(bytes: &[u8]) -> String {
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
    utf8_percent_encode(std::str::from_utf8(bytes).unwrap_or(""), NON_ALPHANUMERIC).to_string()
}

#[derive(Debug, Clone, Default)]
/// Options for List User Activity Feeds Option.
pub struct ListUserActivityFeedsOptions {
    pub list_options: ListOptions,
    pub only_performed_by: bool,
    pub date: String,
}

impl QueryEncode for ListUserActivityFeedsOptions {
    fn query_encode(&self) -> String {
        let mut query = self.list_options.query_encode();
        if self.only_performed_by {
            query.push_str("&only-performed-by=true");
        }
        if !self.date.is_empty() {
            query.push_str("&date=");
            query.push_str(&urlencoding(self.date.as_bytes()));
        }
        query
    }
}

#[derive(Debug, Clone, Default)]
/// Options for List GPGKeys Option.
pub struct ListGPGKeysOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListGPGKeysOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create GPGKey Option.
pub struct CreateGPGKeyOption {
    /// An armored GPG key to add
    #[serde(rename = "armored_public_key")]
    pub armored_key: String,
    /// An optional armored signature for the GPG key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl CreateGPGKeyOption {
    /// Validate this `CreateGPGKeyOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.armored_key.is_empty() {
            return Err(crate::Error::Validation(
                "armored_public_key is required".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Verify GPGKey Option.
pub struct VerifyGPGKeyOption {
    #[serde(rename = "key_id")]
    pub key_id: String,
    #[serde(rename = "armored_signature")]
    pub signature: String,
}

impl VerifyGPGKeyOption {
    /// Validate this `VerifyGPGKeyOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.key_id.is_empty() {
            return Err(crate::Error::Validation("key_id is required".to_string()));
        }
        if self.signature.is_empty() {
            return Err(crate::Error::Validation(
                "armored_signature is required".to_string(),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Update User Avatar Option.
pub struct UpdateUserAvatarOption {
    /// base64 encoded image
    pub image: String,
}

impl UpdateUserAvatarOption {
    /// Validate this `UpdateUserAvatarOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.image.is_empty() {
            return Err(crate::Error::Validation("image is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_email_option_validate_success() {
        let opt = CreateEmailOption {
            emails: vec!["user@example.com".to_string()],
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_email_option_validate_empty_list() {
        let opt = CreateEmailOption { emails: vec![] };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_email_option_validate_empty_email() {
        let opt = CreateEmailOption {
            emails: vec![String::new()],
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_delete_email_option_validate_success() {
        let opt = DeleteEmailOption {
            emails: vec!["user@example.com".to_string()],
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_delete_email_option_validate_empty_list() {
        let opt = DeleteEmailOption { emails: vec![] };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_delete_email_option_validate_empty_email() {
        let opt = DeleteEmailOption {
            emails: vec![String::new()],
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_key_option_validate_success() {
        let opt = CreateKeyOption {
            title: "my-key".to_string(),
            key: "ssh-rsa AAAAB3...".to_string(),
            read_only: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_key_option_validate_empty_key() {
        let opt = CreateKeyOption {
            title: "my-key".to_string(),
            key: String::new(),
            read_only: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_key_option_validate_empty_title() {
        let opt = CreateKeyOption {
            title: String::new(),
            key: "ssh-rsa AAAAB3...".to_string(),
            read_only: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_access_token_option_validate_success() {
        let opt = CreateAccessTokenOption {
            name: "my-token".to_string(),
            scopes: Vec::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_access_token_option_validate_empty_name() {
        let opt = CreateAccessTokenOption {
            name: String::new(),
            scopes: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_gpg_key_option_validate_success() {
        let opt = CreateGPGKeyOption {
            armored_key: "-----BEGIN PGP PUBLIC KEY BLOCK-----".to_string(),
            signature: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_gpg_key_option_validate_empty_key() {
        let opt = CreateGPGKeyOption {
            armored_key: String::new(),
            signature: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_verify_gpg_key_option_validate_success() {
        let opt = VerifyGPGKeyOption {
            key_id: "ABCDEF".to_string(),
            signature: "-----BEGIN PGP SIGNATURE-----".to_string(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_verify_gpg_key_option_validate_empty_key_id() {
        let opt = VerifyGPGKeyOption {
            key_id: String::new(),
            signature: "sig".to_string(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_verify_gpg_key_option_validate_empty_signature() {
        let opt = VerifyGPGKeyOption {
            key_id: "ABCDEF".to_string(),
            signature: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_update_user_avatar_option_validate_success() {
        let opt = UpdateUserAvatarOption {
            image: "base64image".to_string(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_update_user_avatar_option_validate_empty_image() {
        let opt = UpdateUserAvatarOption {
            image: String::new(),
        };
        assert!(opt.validate().is_err());
    }
}
