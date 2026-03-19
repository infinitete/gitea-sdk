// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Deploy key types (repo_key.go).

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::core::Repository;

// ── repo_key.go ─────────────────────────────────────────────────

/// DeployKey a deploy key
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Deploy Key payload type.
pub struct DeployKey {
    pub id: i64,
    #[serde(rename = "key_id")]
    pub key_id: i64,
    pub key: String,
    pub url: String,
    pub title: String,
    pub fingerprint: String,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(rename = "read_only")]
    pub read_only: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<Box<Repository>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::OffsetDateTime;

    fn test_time() -> OffsetDateTime {
        OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
            time::Time::from_hms(10, 0, 0).unwrap(),
        )
    }

    #[test]
    fn test_deploy_key_round_trip() {
        let original = DeployKey {
            id: 1,
            key_id: 2,
            key: "ssh-rsa AAAA...".to_string(),
            url: "https://example.com/keys/1".to_string(),
            title: "CI key".to_string(),
            fingerprint: "abcd".to_string(),
            created: test_time(),
            read_only: true,
            repository: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: DeployKey = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.title, "CI key");
        assert!(restored.repository.is_none());
    }
}
