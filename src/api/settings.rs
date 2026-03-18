// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Settings API endpoints for retrieving Gitea instance configuration.

use crate::Client;
use crate::Response;
use crate::types::settings::{
    GlobalAPISettings, GlobalAttachmentSettings, GlobalRepoSettings, GlobalUISettings,
};

/// API methods for settings. Access via [`Client::settings()`](crate::Client::settings).
pub struct SettingsApi<'a> {
    client: &'a Client,
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

impl<'a> SettingsApi<'a> {
    /// Create a new `SettingsApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// GetGlobalUISettings get global ui settings witch are exposed by API
    pub async fn get_ui_settings(&self) -> crate::Result<(GlobalUISettings, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/settings/ui",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetGlobalRepoSettings get global repository settings witch are exposed by API
    pub async fn get_repo_settings(&self) -> crate::Result<(GlobalRepoSettings, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/settings/repository",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetGlobalAPISettings get global api settings witch are exposed by it
    pub async fn get_api_settings(&self) -> crate::Result<(GlobalAPISettings, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/settings/api",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetGlobalAttachmentSettings get global repository settings witch are exposed by API
    pub async fn get_attachment_settings(
        &self,
    ) -> crate::Result<(GlobalAttachmentSettings, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/settings/attachment",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    #[tokio::test]
    async fn test_get_ui_settings() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/ui"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "default_theme": "gitea-auto",
                "allowed_reactions": ["+1", "-1", "laugh"],
                "custom_emojis": []
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (settings, resp) = client.settings().get_ui_settings().await.unwrap();
        assert_eq!(settings.default_theme, "gitea-auto");
        assert_eq!(settings.allowed_reactions.len(), 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_ui_settings_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/ui"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "message": "Internal Server Error"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.settings().get_ui_settings().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_settings() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/repository"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "mirrors_disabled": false,
                "http_git_disabled": true,
                "migrations_disabled": false,
                "stars_disabled": false,
                "time_tracking_disabled": false,
                "lfs_disabled": true
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (settings, resp) = client.settings().get_repo_settings().await.unwrap();
        assert!(!settings.mirrors_disabled);
        assert!(settings.http_git_disabled);
        assert!(settings.lfs_disabled);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_settings_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/repository"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "message": "Forbidden"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.settings().get_repo_settings().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_api_settings() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/api"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "max_response_items": 50,
                "default_paging_num": 30,
                "default_git_trees_per_page": 1000,
                "default_max_blob_size": 10485760
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (settings, resp) = client.settings().get_api_settings().await.unwrap();
        assert_eq!(settings.max_response_items, 50);
        assert_eq!(settings.default_paging_num, 30);
        assert_eq!(settings.default_git_trees_per_page, 1000);
        assert_eq!(settings.default_max_blob_size, 10485760);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_api_settings_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/api"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "message": "Internal Server Error"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.settings().get_api_settings().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_attachment_settings() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/attachment"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "enabled": true,
                "allowed_types": ".png,.jpg,.jpeg",
                "max_size": 4194304,
                "max_files": 5
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (settings, resp) = client.settings().get_attachment_settings().await.unwrap();
        assert!(settings.enabled);
        assert_eq!(settings.allowed_types, ".png,.jpg,.jpeg");
        assert_eq!(settings.max_size, 4194304);
        assert_eq!(settings.max_files, 5);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_attachment_settings_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/settings/attachment"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "message": "Forbidden"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.settings().get_attachment_settings().await;
        assert!(result.is_err());
    }
}
