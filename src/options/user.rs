// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::AccessTokenScope;
use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Default)]
pub struct ListEmailsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListEmailsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmailOption {
    /// email addresses to add
    pub emails: Vec<String>,
}

impl CreateEmailOption {
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
pub struct DeleteEmailOption {
    /// email addresses to delete
    pub emails: Vec<String>,
}

impl DeleteEmailOption {
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
pub struct ListPublicKeysOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPublicKeysOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct ListFollowersOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListFollowersOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListFollowingOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListFollowingOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ListAccessTokensOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListAccessTokensOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAccessTokenOption {
    pub name: String,
    #[serde(default)]
    pub scopes: Vec<AccessTokenScope>,
}

impl CreateAccessTokenOption {
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.is_empty() {
            return Err(crate::Error::Validation("name is required".to_string()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
pub struct ListUserBlocksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListUserBlocksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Default)]
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
        } else {
            out.push_str(&format!("page={}", defaulted.page.unwrap()));
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
pub struct ListGPGKeysOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListGPGKeysOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGPGKeyOption {
    /// An armored GPG key to add
    #[serde(rename = "armored_public_key")]
    pub armored_key: String,
    /// An optional armored signature for the GPG key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl CreateGPGKeyOption {
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
pub struct VerifyGPGKeyOption {
    #[serde(rename = "key_id")]
    pub key_id: String,
    #[serde(rename = "armored_signature")]
    pub signature: String,
}

impl VerifyGPGKeyOption {
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
pub struct UpdateUserAvatarOption {
    /// base64 encoded image
    pub image: String,
}

impl UpdateUserAvatarOption {
    pub fn validate(&self) -> crate::Result<()> {
        if self.image.is_empty() {
            return Err(crate::Error::Validation("image is required".to_string()));
        }
        Ok(())
    }
}
