// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Issue timeline and subscription types: `TimelineComment`, `WatchInfo`.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::types::label::Label;
use crate::types::milestone::Milestone;
use crate::types::serde_helpers::{null_to_default, nullable_rfc3339};
use crate::types::user::User;

// ── issue_timeline.go ────────────────────────────────────────────

/// `TimelineComment` represents a timeline comment on an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Timeline Comment payload type.
pub struct TimelineComment {
    pub id: i64,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub pr_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(default, rename = "original_author")]
    pub original_author: String,
    #[serde(default, rename = "original_author_id")]
    pub original_author_id: i64,
    #[serde(default)]
    pub body: String,
    #[serde(default, with = "nullable_rfc3339")]
    pub created: Option<OffsetDateTime>,
    #[serde(default, with = "nullable_rfc3339")]
    pub updated: Option<OffsetDateTime>,
    #[serde(default)]
    pub r#type: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub label: Vec<Label>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<Milestone>,
    #[serde(
        default,
        rename = "old_milestone",
        skip_serializing_if = "Option::is_none"
    )]
    pub old_milestone: Option<Milestone>,
    #[serde(default, rename = "new_title")]
    pub new_title: String,
    #[serde(default, rename = "old_title")]
    pub old_title: String,
}

// ── issue_subscription.go ────────────────────────────────────────

/// `WatchInfo` represents the subscription state of an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Watch Info payload type.
pub struct WatchInfo {
    #[serde(default)]
    pub subscribed: bool,
    #[serde(default)]
    pub watching: bool,
    #[serde(default)]
    pub ignored: bool,
    #[serde(default, deserialize_with = "null_to_default")]
    pub reason: String,
    #[serde(rename = "created_at", default, with = "nullable_rfc3339")]
    pub created_at: Option<OffsetDateTime>,
    #[serde(default, rename = "url", deserialize_with = "null_to_default")]
    pub url: String,
    #[serde(
        default,
        rename = "repository_url",
        deserialize_with = "null_to_default"
    )]
    pub repository_url: String,
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
    fn test_timeline_comment_round_trip() {
        let original = TimelineComment {
            id: 1,
            html_url: "https://example.com".to_string(),
            pr_url: String::new(),
            issue_url: "https://example.com".to_string(),
            user: None,
            original_author: String::new(),
            original_author_id: 0,
            body: "comment".to_string(),
            created: Some(test_time()),
            updated: Some(test_time()),
            r#type: "comment".to_string(),
            label: vec![],
            milestone: None,
            old_milestone: None,
            new_title: String::new(),
            old_title: String::new(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: TimelineComment = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert!(restored.user.is_none());
        assert!(restored.label.is_empty());
    }

    #[test]
    fn test_watch_info_round_trip() {
        let original = WatchInfo {
            subscribed: true,
            watching: true,
            ignored: false,
            reason: String::new(),
            created_at: None,
            url: String::new(),
            repository_url: String::new(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: WatchInfo = serde_json::from_str(&json).unwrap();
        assert!(restored.subscribed);
        assert!(restored.watching);
    }
}
