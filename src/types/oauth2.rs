// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for `OAuth2` applications.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::serde_helpers::null_to_default;

/// Oauth2 represents an Oauth2 Application
#[derive(Clone, Serialize, Deserialize)]
/// `OAuth2` payload type.
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

impl std::fmt::Debug for Oauth2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Oauth2")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("client_id", &self.client_id)
            .field(
                "client_secret",
                &if self.client_secret.is_empty() {
                    ""
                } else {
                    "***"
                },
            )
            .field("redirect_uris", &self.redirect_uris)
            .field("confidential_client", &self.confidential_client)
            .field("created", &self.created)
            .finish()
    }
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

    #[test]
    fn test_oauth2_debug_redacts_secret() {
        let app = Oauth2 {
            id: 1,
            name: "My App".to_string(),
            client_id: "abc123".to_string(),
            client_secret: "supersecret".to_string(),
            redirect_uris: vec![],
            confidential_client: false,
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
        };
        let debug = format!("{:?}", app);
        assert!(
            !debug.contains("supersecret"),
            "client_secret must be redacted in Debug output"
        );
        assert!(
            debug.contains("***"),
            "Debug output should contain redaction marker"
        );
    }
}
