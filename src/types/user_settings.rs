// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for user account settings.

use crate::{Deserialize, Serialize};

/// UserSettings represents user settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// User Settings payload type.
pub struct UserSettings {
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub website: String,
    pub description: String,
    pub location: String,
    pub language: String,
    pub theme: String,
    #[serde(rename = "diff_view_style")]
    pub diff_view_style: String,
    /// Privacy
    #[serde(rename = "hide_email")]
    pub hide_email: bool,
    #[serde(rename = "hide_activity")]
    pub hide_activity: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_settings_round_trip() {
        let original = UserSettings {
            full_name: "John Doe".to_string(),
            website: "https://example.com".to_string(),
            description: "A developer".to_string(),
            location: "World".to_string(),
            language: "en-US".to_string(),
            theme: "gitea-dark".to_string(),
            diff_view_style: "unified".to_string(),
            hide_email: false,
            hide_activity: true,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: UserSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.full_name, original.full_name);
        assert_eq!(restored.theme, original.theme);
        assert_eq!(restored.hide_email, original.hide_email);
    }
}
