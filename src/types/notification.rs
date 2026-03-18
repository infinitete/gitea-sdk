// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use crate::types::enums::{NotifySubjectState, NotifySubjectType};

/// NotificationSubject contains the notification subject (Issue/Pull/Commit)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Notify Subject payload type.
pub struct NotifySubject {
    pub title: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "latest_comment_url")]
    pub latest_comment_url: String,
    #[serde(rename = "latest_comment_html_url")]
    pub latest_comment_html_url: String,
    #[serde(rename = "type")]
    pub subject_type: NotifySubjectType,
    pub state: NotifySubjectState,
}

/// NotificationThread represents a notification on the API
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Notification Thread payload type.
pub struct NotificationThread {
    pub id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<NotifySubject>,
    pub unread: bool,
    pub pinned: bool,
    #[serde(rename = "updated_at", with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_subject_round_trip() {
        let original = NotifySubject {
            title: "Bug fix".to_string(),
            url: "https://gitea.example.com/api/v1/repos/test/repo/issues/1".to_string(),
            html_url: "https://gitea.example.com/test/repo/issues/1".to_string(),
            latest_comment_url: "".to_string(),
            latest_comment_html_url: "".to_string(),
            subject_type: NotifySubjectType::Issue,
            state: NotifySubjectState::Open,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: NotifySubject = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.title, original.title);
        assert_eq!(restored.subject_type, original.subject_type);
        assert_eq!(restored.state, original.state);
    }

    #[test]
    fn test_notification_thread_deserialize() {
        let json = r#"{
            "id": 1,
            "repository": null,
            "subject": null,
            "unread": true,
            "pinned": false,
            "updated_at": "2024-01-15T10:00:00Z",
            "url": "https://gitea.example.com/notifications/1"
        }"#;
        let thread: NotificationThread = serde_json::from_str(json).unwrap();
        assert_eq!(thread.id, 1);
        assert!(thread.unread);
        assert!(thread.subject.is_none());
    }
}
