// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::comment::Comment;
use super::user::User;

/// Activity represents a user or organization activity
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Activity payload type.
pub struct Activity {
    pub id: i64,
    #[serde(rename = "act_user_id")]
    pub act_user_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub act_user: Option<User>,
    #[serde(rename = "op_type")]
    pub op_type: String,
    pub content: String,
    #[serde(rename = "repo_id")]
    pub repo_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<serde_json::Value>,
    #[serde(rename = "comment_id")]
    pub comment_id: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<Comment>,
    #[serde(rename = "ref_name")]
    pub ref_name: String,
    #[serde(rename = "is_private")]
    pub is_private: bool,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_activity_deserialize() {
        let json = r#"{
            "id": 1,
            "act_user_id": 2,
            "act_user": null,
            "op_type": "create_repo",
            "content": "",
            "repo_id": 3,
            "repo": null,
            "comment_id": 0,
            "comment": null,
            "ref_name": "",
            "is_private": false,
            "user_id": 2,
            "created": "2024-01-15T10:00:00Z"
        }"#;
        let activity: Activity = serde_json::from_str(json).unwrap();
        assert_eq!(activity.id, 1);
        assert_eq!(activity.op_type, "create_repo");
        assert!(activity.act_user.is_none());
    }
}
