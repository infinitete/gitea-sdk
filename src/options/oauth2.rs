// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for OAuth2 application API endpoints.

use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create OAuth2 Option.
pub struct CreateOauth2Option {
    pub name: String,
    #[serde(rename = "confidential_client", default)]
    pub confidential_client: bool,
    #[serde(
        rename = "redirect_uris",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub redirect_uris: Vec<String>,
}

#[derive(Debug, Clone, Default)]
/// Options for List OAuth2 Option.
pub struct ListOauth2Option {
    pub list_options: crate::pagination::ListOptions,
}

impl crate::pagination::QueryEncode for ListOauth2Option {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

impl CreateOauth2Option {
    /// Validate this `CreateOauth2Option` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::Error::Validation("name is required".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_oauth2_option_validate_success() {
        let opt = CreateOauth2Option {
            name: "my-app".to_string(),
            confidential_client: false,
            redirect_uris: Vec::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_oauth2_option_validate_empty_name() {
        let opt = CreateOauth2Option {
            name: String::new(),
            confidential_client: false,
            redirect_uris: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }
}
