// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for repository and organization secrets.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

/// Secret represents a repository or organization secret
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Secret payload type.
pub struct Secret {
    /// the secret's name
    pub name: String,
    /// the secret's data
    #[serde(default)]
    pub data: String,
    /// the secret's description
    #[serde(default)]
    pub description: String,
    /// Date and Time of secret creation
    #[serde(default, with = "rfc3339::option")]
    pub created: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secret_round_trip() {
        let original = Secret {
            name: "MY_SECRET".to_string(),
            data: "supersecretvalue".to_string(),
            description: "A secret value".to_string(),
            created: Some(OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            )),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Secret = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.data, original.data);
    }
}
