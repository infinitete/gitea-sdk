// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::user::UserSettingsOptions;
use crate::types::UserSettings;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_settings.go ───────────────────────────────────────────────

    /// `GetUserSettings` returns user settings
    pub async fn get_settings(&self) -> crate::Result<(UserSettings, Response)> {
        self.client()
            .get_parsed_response(reqwest::Method::GET, "/user/settings", None, None::<&str>)
            .await
    }

    /// `UpdateUserSettings` returns user settings
    pub async fn update_settings(
        &self,
        opt: UserSettingsOptions,
    ) -> crate::Result<(UserSettings, Response)> {
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                "/user/settings",
                Some(&json_header()),
                Some(body),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::create_test_client;

    // ── get_settings ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_settings_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "full_name": "Test User",
            "website": "",
            "description": "",
            "location": "",
            "language": "en-US",
            "theme": "gitea-dark",
            "diff_view_style": "unified",
            "hide_email": false,
            "hide_activity": false
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (settings, resp) = client.users().get_settings().await.unwrap();
        assert_eq!(settings.full_name, "Test User");
        assert_eq!(settings.theme, "gitea-dark");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_settings_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_settings().await;
        assert!(result.is_err());
    }

    // ── update_settings ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_settings_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "full_name": "Updated Name",
            "website": "",
            "description": "",
            "location": "",
            "language": "en-US",
            "theme": "gitea-light",
            "diff_view_style": "unified",
            "hide_email": true,
            "hide_activity": false
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = UserSettingsOptions {
            full_name: Some("Updated Name".to_string()),
            hide_email: Some(true),
            ..Default::default()
        };
        let (settings, resp) = client.users().update_settings(opt).await.unwrap();
        assert_eq!(settings.full_name, "Updated Name");
        assert!(settings.hide_email);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_settings_error() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .update_settings(UserSettingsOptions::default())
            .await;
        assert!(result.is_err());
    }
}
