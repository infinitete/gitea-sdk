// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for milestones.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::serde_helpers::nullable_rfc3339;
use crate::types::enums::StateType;

/// Milestone represents a collection of issues on one repository
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Milestone payload type.
pub struct Milestone {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub state: StateType,
    #[serde(rename = "open_issues")]
    pub open_issues: i32,
    #[serde(rename = "closed_issues")]
    pub closed_issues: i32,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created: OffsetDateTime,
    #[serde(
        rename = "updated_at",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub updated: Option<OffsetDateTime>,
    #[serde(
        rename = "closed_at",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub closed: Option<OffsetDateTime>,
    #[serde(
        rename = "due_on",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_milestone_round_trip() {
        let original = Milestone {
            id: 1,
            title: "v1.0".to_string(),
            description: "First release".to_string(),
            state: StateType::Open,
            open_issues: 5,
            closed_issues: 2,
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            updated: None,
            closed: None,
            deadline: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Milestone = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.state, original.state);
        assert!(restored.updated.is_none());
    }

    #[test]
    fn test_milestone_with_deadline() {
        let json = r#"{
            "id": 2,
            "title": "v2.0",
            "description": "",
            "state": "open",
            "open_issues": 0,
            "closed_issues": 0,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-02-01T00:00:00Z",
            "closed_at": null,
            "due_on": "2024-06-01T00:00:00Z"
        }"#;
        let milestone: Milestone = serde_json::from_str(json).unwrap();
        assert_eq!(milestone.title, "v2.0");
        assert!(milestone.updated.is_some());
        assert!(milestone.deadline.is_some());
    }
}
