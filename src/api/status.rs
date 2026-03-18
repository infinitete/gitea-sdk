// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Commit status API endpoints for managing Gitea CI/CD commit statuses.

use crate::Client;
use crate::Response;
use crate::options::status::*;
use crate::pagination::QueryEncode;
use crate::types::{CombinedStatus, Status};

/// API methods for commit statuses. Access via [`Client::status()`](crate::Client::status).
pub struct StatusApi<'a> {
    client: &'a Client,
}

fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

impl<'a> StatusApi<'a> {
    /// Create a new `StatusApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// CreateStatus creates a new Status for a given Commit
    pub async fn create_status(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        opt: CreateStatusOption,
    ) -> crate::Result<(Status, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let sha_encoded = {
            use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
            utf8_percent_encode(sha, NON_ALPHANUMERIC).to_string()
        };
        let path = format!(
            "/repos/{}/{}/statuses/{}",
            escaped[0], escaped[1], sha_encoded
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListStatuses returns all statuses for a given Commit by ref
    pub async fn list_statuses(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        opt: ListStatusesOption,
    ) -> crate::Result<(Vec<Status>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, ref_])?;
        let path = format!(
            "/repos/{}/{}/commits/{}/statuses?{}",
            escaped[0],
            escaped[1],
            escaped[2],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetCombinedStatus returns the CombinedStatus for a given Commit
    pub async fn get_combined_status(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(CombinedStatus, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, ref_])?;
        let path = format!(
            "/repos/{}/{}/commits/{}/status",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::StatusState;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn status_json(id: i64, state: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "status": state,
            "target_url": "https://ci.example.com/build/1",
            "description": "Build passed",
            "url": "https://gitea.example.com/api/v1/repos/test/repo/statuses/abc123",
            "context": "ci/build",
            "creator": null,
            "created": "2024-01-15T10:00:00Z",
            "updated": "2024-01-15T10:00:00Z"
        })
    }

    #[tokio::test]
    async fn test_create_status() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/statuses/abc123"))
            .respond_with(ResponseTemplate::new(201).set_body_json(status_json(1, "success")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateStatusOption {
            state: StatusState::Success,
            target_url: Some("https://ci.example.com/build/1".to_string()),
            description: Some("Build passed".to_string()),
            context: Some("ci/build".to_string()),
        };
        let (status, resp) = client
            .status()
            .create_status("testowner", "testrepo", "abc123", opt)
            .await
            .unwrap();
        assert_eq!(status.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_list_statuses() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/commits/main/statuses",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec![status_json(1, "success")]))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (statuses, resp) = client
            .status()
            .list_statuses("testowner", "testrepo", "main", Default::default())
            .await
            .unwrap();
        assert_eq!(statuses.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_combined_status() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/commits/main/status"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "success",
                "sha": "abc123def456",
                "total_count": 2,
                "statuses": [],
                "commit_url": "",
                "url": ""
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (combined, resp) = client
            .status()
            .get_combined_status("testowner", "testrepo", "main")
            .await
            .unwrap();
        assert_eq!(combined.state, StatusState::Success);
        assert_eq!(combined.total_count, 2);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_status_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/statuses/abc123"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(serde_json::json!({"message": "internal error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateStatusOption {
            state: StatusState::Success,
            target_url: None,
            description: None,
            context: None,
        };
        let result = client
            .status()
            .create_status("testowner", "testrepo", "abc123", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_statuses_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/commits/main/statuses",
            ))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(serde_json::json!({"message": "internal error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .status()
            .list_statuses("testowner", "testrepo", "main", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_combined_status_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/commits/main/status"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(serde_json::json!({"message": "internal error"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .status()
            .get_combined_status("testowner", "testrepo", "main")
            .await;
        assert!(result.is_err());
    }
}
