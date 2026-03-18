// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for OAuth2 applications.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::serde_helpers::null_to_default;

/// Oauth2 represents an Oauth2 Application
#[derive(Debug, Clone, Serialize, Deserialize)]
/// OAuth2 payload type.
pub struct Oauth2 {
    pub id: i64,
    pub name: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "client_secret")]
    pub client_secret: String,
    #[serde(
        rename = "redirect_uris",
        default,
        deserialize_with = "null_to_default"
    )]
    pub redirect_uris: Vec<String>,
    #[serde(rename = "confidential_client")]
    pub confidential_client: bool,
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth2_round_trip() {
        let original = Oauth2 {
            id: 1,
            name: "My App".to_string(),
            client_id: "abc123".to_string(),
            client_secret: "secret456".to_string(),
            redirect_uris: vec!["https://example.com/callback".to_string()],
            confidential_client: true,
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Oauth2 = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.redirect_uris.len(), 1);
    }
}
