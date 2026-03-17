// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::{HookType, VisibleType};
use crate::{Deserialize, Serialize};

// ── admin_user.go ───────────────────────────────────────────────────

/// Options for listing admin users
#[derive(Debug, Clone, Default)]
pub struct AdminListUsersOptions {
    pub list_options: ListOptions,
    pub source_id: i64,
    pub login_name: String,
    pub query: String,
    pub sort: String,
    pub order: String,
    pub visibility: String,
    pub is_active: Option<bool>,
    pub is_admin: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_2fa_enabled: Option<bool>,
    pub is_prohibit_login: Option<bool>,
}

impl QueryEncode for AdminListUsersOptions {
    fn query_encode(&self) -> String {
        let mut parts = vec![self.list_options.query_encode()];
        if self.source_id > 0 {
            parts.push(format!("source_id={}", self.source_id));
        }
        if !self.login_name.is_empty() {
            parts.push(format!("login_name={}", self.login_name));
        }
        if !self.query.is_empty() {
            parts.push(format!("q={}", self.query));
        }
        if !self.sort.is_empty() {
            parts.push(format!("sort={}", self.sort));
        }
        if !self.order.is_empty() {
            parts.push(format!("order={}", self.order));
        }
        if !self.visibility.is_empty() {
            parts.push(format!("visibility={}", self.visibility));
        }
        if let Some(v) = self.is_active {
            parts.push(format!("is_active={v}"));
        }
        if let Some(v) = self.is_admin {
            parts.push(format!("is_admin={v}"));
        }
        if let Some(v) = self.is_restricted {
            parts.push(format!("is_restricted={v}"));
        }
        if let Some(v) = self.is_2fa_enabled {
            parts.push(format!("is_2fa_enabled={v}"));
        }
        if let Some(v) = self.is_prohibit_login {
            parts.push(format!("is_prohibit_login={v}"));
        }
        parts.join("&")
    }
}

/// Options for creating a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateUserOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_name: Option<String>,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    pub email: String,
    pub password: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "must_change_password"
    )]
    pub must_change_password: Option<bool>,
    #[serde(default, rename = "send_notify")]
    pub send_notify: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibleType>,
}

impl CreateUserOption {
    pub fn validate(&self) -> crate::Result<()> {
        if self.username.is_empty() {
            return Err(crate::Error::Validation("username is empty".to_string()));
        }
        if self.password.is_empty() {
            return Err(crate::Error::Validation("password is empty".to_string()));
        }
        if self.email.is_empty() {
            return Err(crate::Error::Validation("email is empty".to_string()));
        }
        Ok(())
    }
}

/// Options for editing a user
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditUserOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "must_change_password"
    )]
    pub must_change_password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "allow_git_hook")]
    pub allow_git_hook: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "allow_import_local")]
    pub allow_import_local: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "max_repo_creation")]
    pub max_repo_creation: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "prohibit_login")]
    pub prohibit_login: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename = "allow_create_organization"
    )]
    pub allow_create_organization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<VisibleType>,
}

/// Options for renaming a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameUserOption {
    #[serde(rename = "new_username")]
    pub new_username: String,
}

// ── admin_repo.go ───────────────────────────────────────────────────

/// Options for listing unadopted repositories
#[derive(Debug, Clone, Default)]
pub struct ListUnadoptedReposOptions {
    pub list_options: ListOptions,
    pub pattern: String,
}

impl QueryEncode for ListUnadoptedReposOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if !self.pattern.is_empty() {
            out.push_str(&format!("&pattern={}", self.pattern));
        }
        out
    }
}

// ── admin_org.go ────────────────────────────────────────────────────

/// Options for listing admin organizations
#[derive(Debug, Clone, Default)]
pub struct AdminListOrgsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for AdminListOrgsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── admin_cron.go ───────────────────────────────────────────────────

/// Options for listing cron tasks
#[derive(Debug, Clone, Default)]
pub struct ListCronTasksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListCronTasksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── admin_hooks.go ──────────────────────────────────────────────────

/// Options for listing admin hooks
#[derive(Debug, Clone, Default)]
pub struct ListAdminHooksOptions {
    pub list_options: ListOptions,
    pub hook_type: String,
}

impl QueryEncode for ListAdminHooksOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if !self.hook_type.is_empty() {
            out.push_str(&format!("&type={}", self.hook_type));
        }
        out
    }
}

/// Options for creating a hook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHookOption {
    #[serde(rename = "type")]
    pub hook_type: HookType,
    pub config: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub events: Vec<String>,
    #[serde(default, rename = "branch_filter")]
    pub branch_filter: String,
    #[serde(default)]
    pub active: bool,
    #[serde(default, rename = "authorization_header")]
    pub authorization_header: String,
}

impl CreateHookOption {
    pub fn validate(&self) -> crate::Result<()> {
        // HookType::Unknown means empty, which is invalid
        if matches!(self.hook_type, HookType::Unknown) {
            return Err(crate::Error::Validation(
                "hook type is required".to_string(),
            ));
        }
        Ok(())
    }
}

/// Options for editing a hook
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditHookOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<std::collections::HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(default, rename = "branch_filter")]
    pub branch_filter: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, rename = "authorization_header")]
    pub authorization_header: String,
}

// ── admin_email.go ──────────────────────────────────────────────────

/// Options for listing admin emails
#[derive(Debug, Clone, Default)]
pub struct ListAdminEmailsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListAdminEmailsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// Options for searching admin emails
#[derive(Debug, Clone, Default)]
pub struct SearchAdminEmailsOptions {
    pub list_options: ListOptions,
    pub query: String,
}

impl QueryEncode for SearchAdminEmailsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if !self.query.is_empty() {
            out.push_str(&format!("&q={}", self.query));
        }
        out
    }
}

// ── admin_badges.go ─────────────────────────────────────────────────

/// Options for adding user badges
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBadgeOption {
    #[serde(rename = "badge_slugs")]
    pub badge_slugs: Vec<String>,
}
