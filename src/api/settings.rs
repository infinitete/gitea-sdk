// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::types::settings::{
    GlobalAPISettings, GlobalAttachmentSettings, GlobalRepoSettings, GlobalUISettings,
};

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
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

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
}
