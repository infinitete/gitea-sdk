// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Issue time tracking types: `StopWatch`, `TrackedTime`.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::issue::Issue;

// ── issue_stopwatch.go ───────────────────────────────────────────

/// `StopWatch` represents a running stopwatch of an issue / pr
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Stop Watch payload type.
pub struct StopWatch {
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
    pub seconds: i64,
    pub duration: String,
    #[serde(rename = "issue_index")]
    pub issue_index: i64,
    #[serde(rename = "issue_title")]
    pub issue_title: String,
    #[serde(rename = "repo_owner_name")]
    pub repo_owner_name: String,
    #[serde(rename = "repo_name")]
    pub repo_name: String,
}

// ── issue_tracked_time.go ────────────────────────────────────────

/// `TrackedTime` worked time for an issue / pr
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tracked Time payload type.
pub struct TrackedTime {
    pub id: i64,
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
    /// Time in seconds
    pub time: i64,
    #[serde(default, rename = "user_id")]
    pub user_id: i64,
    #[serde(default, rename = "user_name")]
    pub user_name: String,
    #[serde(default, rename = "issue_id")]
    pub issue_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue: Option<Box<Issue>>,
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
    fn test_stop_watch_round_trip() {
        let original = StopWatch {
            created: test_time(),
            seconds: 3600,
            duration: "1h0m0s".to_string(),
            issue_index: 1,
            issue_title: "Fix bug".to_string(),
            repo_owner_name: "owner".to_string(),
            repo_name: "repo".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: StopWatch = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.seconds, 3600);
        assert_eq!(restored.issue_index, 1);
    }

    #[test]
    fn test_tracked_time_round_trip() {
        let original = TrackedTime {
            id: 1,
            created: test_time(),
            time: 1800,
            user_id: 0,
            user_name: String::new(),
            issue_id: 0,
            issue: None,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: TrackedTime = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.time, 1800);
        assert!(restored.issue.is_none());
    }
}
