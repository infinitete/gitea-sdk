// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::admin::ListUnadoptedReposOptions;
use crate::pagination::QueryEncode;

use super::AdminApi;

impl<'a> AdminApi<'a> {
    // ── admin_repo.go ────────────────────────────────────────────────

    /// List unadopted repositories
    pub async fn list_unadopted_repos(
        &self,
        opt: ListUnadoptedReposOptions,
    ) -> crate::Result<(Vec<String>, Response)> {
        let path = format!("/admin/unadopted?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Adopt an unadopted repository
    pub async fn adopt_unadopted_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/admin/unadopted/{}/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Delete an unadopted repository
    pub async fn delete_unadopted_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/admin/unadopted/{}/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::create_test_client;

    #[tokio::test]
    async fn test_list_unadopted_repos() {
        let server = MockServer::start().await;
        let body = serde_json::json!(["org1/repo1", "org2/repo2"]);

        Mock::given(method("GET"))
            .and(path("/api/v1/admin/unadopted"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (repos, resp) = client
            .admin()
            .list_unadopted_repos(Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0], "org1/repo1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .admin()
            .adopt_unadopted_repo("org1", "repo1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo_empty_owner() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().adopt_unadopted_repo("", "repo1").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo_empty_repo() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().adopt_unadopted_repo("org1", "").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [1] is empty")
        );
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().adopt_unadopted_repo("org1", "repo1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .admin()
            .delete_unadopted_repo("org1", "repo1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo_empty_owner() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().delete_unadopted_repo("", "repo1").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo_empty_repo() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().delete_unadopted_repo("org1", "").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [1] is empty")
        );
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().delete_unadopted_repo("org1", "repo1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_unadopted_repos_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/unadopted"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .admin()
            .list_unadopted_repos(Default::default())
            .await;
        assert!(result.is_err());
    }
}
