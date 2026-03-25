// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_key.go (4 methods) ───────────────────────────────────

    /// `ListDeployKeys` list deploy keys
    pub async fn list_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
        opt: ListDeployKeysOptions,
    ) -> crate::Result<(Vec<DeployKey>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/keys?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetDeployKey` get a deploy key
    pub async fn get_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(DeployKey, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/keys/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateDeployKey` create a deploy key
    pub async fn create_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateKeyOption,
    ) -> crate::Result<(DeployKey, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/keys", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteDeployKey` delete a deploy key
    pub async fn delete_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/keys/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_deploy_keys_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_deploy_key_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_deploy_keys("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (keys, resp) = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_deploy_keys_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_deploy_keys("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_deploy_key_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_deploy_key_json(1)))
            .mount(&server)
            .await;
        let result = client.repos().get_deploy_key("owner", "repo", 1).await;
        assert!(result.is_ok());
        let (key, resp) = result.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(key.title, "deploy-key");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_deploy_key_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_deploy_key("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_deploy_key_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_deploy_key_json(1)))
            .mount(&server)
            .await;
        let opt = CreateKeyOption {
            title: "deploy-key".to_string(),
            key: "ssh-rsa AAAAB3...".to_string(),
            read_only: true,
        };
        let result = client.repos().create_deploy_key("owner", "repo", opt).await;
        assert!(result.is_ok());
        let (key, resp) = result.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_deploy_key_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateKeyOption {
            title: "deploy-key".to_string(),
            key: "ssh-rsa AAAAB3...".to_string(),
            read_only: true,
        };
        let result = client.repos().create_deploy_key("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_deploy_key_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client.repos().delete_deploy_key("owner", "repo", 1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_deploy_key_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().delete_deploy_key("owner", "repo", 1).await;
        assert!(result.is_err());
    }
}
