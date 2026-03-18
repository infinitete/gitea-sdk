// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for repository releases and attachments.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::user::User;

/// Attachment represents a generic attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Attachment payload type.
pub struct Attachment {
    pub id: i64,
    pub name: String,
    pub size: i64,
    #[serde(rename = "download_count")]
    pub download_count: i64,
    #[serde(with = "rfc3339", alias = "created_at")]
    pub created: OffsetDateTime,
    pub uuid: String,
    #[serde(rename = "browser_download_url")]
    pub download_url: String,
}

/// Release represents a repository release
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Release payload type.
pub struct Release {
    pub id: i64,
    #[serde(rename = "tag_name")]
    pub tag_name: String,
    #[serde(alias = "target_commitish", default)]
    pub target: String,
    #[serde(alias = "name", default)]
    pub title: String,
    #[serde(alias = "body", default)]
    pub note: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "tarball_url")]
    pub tar_url: String,
    #[serde(rename = "zipball_url")]
    pub zip_url: String,
    #[serde(rename = "draft")]
    pub is_draft: bool,
    pub prerelease: bool,
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(rename = "published_at", with = "rfc3339")]
    pub published_at: OffsetDateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher: Option<User>,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_round_trip() {
        let original = Release {
            id: 1,
            tag_name: "v1.0.0".to_string(),
            target: "main".to_string(),
            title: "v1.0.0".to_string(),
            note: "First release".to_string(),
            url: "https://gitea.example.com/api/v1/repos/test/repo/releases/1".to_string(),
            html_url: "https://gitea.example.com/test/repo/releases/tag/v1.0.0".to_string(),
            tar_url: "https://gitea.example.com/test/repo/archive/v1.0.0.tar.gz".to_string(),
            zip_url: "https://gitea.example.com/test/repo/archive/v1.0.0.zip".to_string(),
            is_draft: false,
            prerelease: false,
            created_at: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            published_at: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            publisher: None,
            attachments: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Release = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.tag_name, original.tag_name);
    }

    #[test]
    fn test_attachment_round_trip() {
        let original = Attachment {
            id: 1,
            name: "binary.zip".to_string(),
            size: 1024,
            download_count: 5,
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
            uuid: "abc123".to_string(),
            download_url: "https://gitea.example.com/attachments/abc123".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Attachment = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
    }
}
