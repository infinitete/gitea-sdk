// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::enums::StatusState;

use super::user::User;

/// Status holds a single Status of a single Commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Status payload type.
pub struct Status {
    pub id: i64,
    #[serde(alias = "status")]
    pub state: StatusState,
    #[serde(rename = "target_url")]
    pub target_url: String,
    pub description: String,
    pub url: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<User>,
    #[serde(with = "rfc3339", alias = "created_at")]
    pub created: OffsetDateTime,
    #[serde(with = "rfc3339", alias = "updated_at")]
    pub updated: OffsetDateTime,
}

/// CombinedStatus holds the combined state of several statuses for a single commit
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Combined Status payload type.
pub struct CombinedStatus {
    pub state: StatusState,
    pub sha: String,
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(default)]
    pub statuses: Vec<Status>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    #[serde(rename = "commit_url")]
    pub commit_url: String,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_round_trip() {
        let original = Status {
            id: 1,
            state: StatusState::Success,
            target_url: "https://ci.example.com/build/1".to_string(),
            description: "Build passed".to_string(),
            url: "https://gitea.example.com/api/v1/repos/test/repo/statuses/abc123".to_string(),
            context: "ci/build".to_string(),
            creator: None,
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            updated: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Status = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.state, original.state);
    }

    #[test]
    fn test_combined_status_deserialize() {
        let json = r#"{
            "state": "success",
            "sha": "abc123def456",
            "total_count": 2,
            "statuses": [],
            "commit_url": "https://gitea.example.com/api/v1/repos/test/repo/git/commits/abc123def456",
            "url": "https://gitea.example.com/api/v1/repos/test/repo/commits/abc123def456/status"
        }"#;
        let combined: CombinedStatus = serde_json::from_str(json).unwrap();
        assert_eq!(combined.state, StatusState::Success);
        assert_eq!(combined.sha, "abc123def456");
        assert_eq!(combined.total_count, 2);
        assert!(combined.statuses.is_empty());
    }

    #[test]
    fn test_status_deserialize_live_field_names() {
        let json = r#"{
            "id": 1,
            "status": "success",
            "target_url": "https://ci.example.com/build/1",
            "description": "Build passed",
            "url": "https://gitea.example.com/api/v1/repos/test/repo/statuses/abc123",
            "context": "ci/build",
            "creator": null,
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T10:00:00Z"
        }"#;
        let status: Status = serde_json::from_str(json).unwrap();
        assert_eq!(status.state, StatusState::Success);
        assert_eq!(status.context, "ci/build");
    }
}
