// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Shared test utilities for integration tests.
//!
//! Import via `mod common;` in integration test files.
//!
//! # Example
//!
//! ```ignore
//! mod common;
//!
//! use common::{create_test_client, mock_json_response};
//! ```

use serde_json::Value;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use gitea_sdk::Client;

/// Build a [`Client`] pointing at the given mock server URL with a test token.
pub fn create_test_client(server: &MockServer) -> Client {
    Client::builder(&server.uri())
        .token("test-token")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap()
}

/// Mount a mock returning a JSON body for the given HTTP method and full path.
///
/// The `path` should include the `/api/v1/` prefix (e.g. `"/api/v1/user"`).
pub async fn mock_json_response(
    server: &MockServer,
    http_method: &str,
    path: &str,
    status: u16,
    body: Value,
) -> Mock {
    let template = ResponseTemplate::new(status).set_body_json(body);
    mount_mock(server, http_method, path, template).await
}

/// Mount a mock returning an empty body for the given HTTP method and full path.
pub async fn mock_empty_response(
    server: &MockServer,
    http_method: &str,
    path: &str,
    status: u16,
) -> Mock {
    let template = ResponseTemplate::new(status).set_body_string("");
    mount_mock(server, http_method, path, template).await
}

/// Mount a mock returning a JSON error body (`{"message": "..."}`).
pub async fn mock_error_response(
    server: &MockServer,
    http_method: &str,
    path: &str,
    status: u16,
    message: &str,
) -> Mock {
    mock_json_response(server, http_method, path, status, error_json(message)).await
}

/// Create a standard Gitea API error JSON value.
pub fn error_json(message: &str) -> Value {
    serde_json::json!({"message": message})
}

/// Internal helper: build and mount a wiremock with the given method, path, and response template.
async fn mount_mock(
    server: &MockServer,
    http_method: &str,
    path: &str,
    template: ResponseTemplate,
) -> Mock {
    let mock = Mock::given(method(http_method))
        .and(path(path))
        .respond_with(template);
    mock.mount(server).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_json_structure() {
        let val = error_json("Not Found");
        assert_eq!(val["message"], "Not Found");
    }

    #[test]
    fn test_error_json_empty_message() {
        let val = error_json("");
        assert_eq!(val["message"], "");
    }

    #[tokio::test]
    async fn test_create_test_client_returns_client() {
        let server = MockServer::start().await;
        let _client = create_test_client(&server);
    }

    #[tokio::test]
    async fn test_mock_json_response_mounts() {
        let server = MockServer::start().await;
        mock_json_response(
            &server,
            "GET",
            "/api/v1/test",
            200,
            serde_json::json!({"ok": true}),
        )
        .await;

        let resp = reqwest::get(format!("{}/api/v1/test", server.uri()))
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 200);
        let body: Value = resp.json().await.unwrap();
        assert_eq!(body["ok"], true);
    }

    #[tokio::test]
    async fn test_mock_empty_response_mounts() {
        let server = MockServer::start().await;
        mock_empty_response(&server, "DELETE", "/api/v1/test/1", 204).await;

        let client = reqwest::Client::new();
        let resp = client
            .delete(format!("{}/api/v1/test/1", server.uri()))
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 204);
    }

    #[tokio::test]
    async fn test_mock_error_response_mounts() {
        let server = MockServer::start().await;
        mock_error_response(&server, "GET", "/api/v1/test", 404, "Not Found").await;

        let resp = reqwest::get(format!("{}/api/v1/test", server.uri()))
            .await
            .unwrap();
        assert_eq!(resp.status().as_u16(), 404);
        let body: Value = resp.json().await.unwrap();
        assert_eq!(body["message"], "Not Found");
    }
}
