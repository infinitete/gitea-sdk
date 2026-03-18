// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for webhooks.

use crate::{Deserialize, Serialize};
use std::collections::HashMap;
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::enums::HookType;

use super::serde_helpers::null_to_default;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Hook payload type.
pub struct Hook {
    pub id: i64,
    #[serde(rename = "type")]
    pub hook_type: HookType,
    #[serde(skip)]
    pub url: String,
    #[serde(rename = "branch_filter")]
    pub branch_filter: String,
    #[serde(default)]
    pub config: HashMap<String, String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub events: Vec<String>,
    #[serde(rename = "authorization_header")]
    pub authorization_header: String,
    pub active: bool,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_deserialize() {
        let json = r#"{
            "id": 1,
            "type": "slack",
            "branch_filter": "",
            "config": {"url": "https://example.com/hook"},
            "events": ["push"],
            "authorization_header": "",
            "active": true,
            "updated_at": "2024-01-15T10:00:00Z",
            "created_at": "2024-01-15T10:00:00Z"
        }"#;
        let hook: Hook = serde_json::from_str(json).unwrap();
        assert_eq!(hook.id, 1);
        assert_eq!(hook.hook_type, HookType::Slack);
        assert!(hook.url.is_empty());
        assert_eq!(hook.events.len(), 1);
        assert!(hook.active);
    }

    #[test]
    fn test_hook_url_skipped() {
        let hook = Hook {
            id: 1,
            hook_type: HookType::Gitea,
            url: "https://example.com/hook".to_string(),
            branch_filter: String::new(),
            config: HashMap::new(),
            events: Vec::new(),
            authorization_header: String::new(),
            active: true,
            updated_at: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            created_at: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
        };
        let json = serde_json::to_string(&hook).unwrap();
        assert!(!json.contains("url"));
    }
}
