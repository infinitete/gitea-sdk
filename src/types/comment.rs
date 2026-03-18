// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for comments on issues and commits.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::user::User;

/// Comment represents a comment on a commit or issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Comment payload type.
pub struct Comment {
    pub id: i64,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "pull_request_url")]
    pub pr_url: String,
    #[serde(rename = "issue_url")]
    pub issue_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none", alias = "user")]
    pub poster: Option<User>,
    #[serde(rename = "original_author")]
    pub original_author: String,
    #[serde(rename = "original_author_id")]
    pub original_author_id: i64,
    pub body: String,
    #[serde(with = "rfc3339", alias = "created_at")]
    pub created: OffsetDateTime,
    #[serde(with = "rfc3339", alias = "updated_at")]
    pub updated: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_round_trip() {
        let original = Comment {
            id: 1,
            html_url: "https://gitea.example.com/test/repo/issues/1#issuecomment-1".to_string(),
            pr_url: "".to_string(),
            issue_url: "https://gitea.example.com/api/v1/repos/test/repo/issues/1".to_string(),
            poster: None,
            original_author: "".to_string(),
            original_author_id: 0,
            body: "Nice work!".to_string(),
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            updated: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 30, 0).unwrap(),
            ),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Comment = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.body, original.body);
    }

    #[test]
    fn test_comment_deserialize_live_field_names() {
        let json = r#"{
            "id": 2,
            "html_url": "https://gitea.example.com/o/r/issues/1#issuecomment-2",
            "pull_request_url": "",
            "issue_url": "https://gitea.example.com/api/v1/repos/o/r/issues/1",
            "user": null,
            "original_author": "",
            "original_author_id": 0,
            "body": "live comment",
            "assets": [],
            "created_at": "2026-03-18T12:54:50+08:00",
            "updated_at": "2026-03-18T12:54:50+08:00"
        }"#;
        let comment: Comment = serde_json::from_str(json).unwrap();
        assert_eq!(comment.id, 2);
        assert_eq!(comment.body, "live comment");
    }
}
