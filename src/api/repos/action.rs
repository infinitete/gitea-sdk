// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::CreateSecretOption;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::Secret;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_action.go (8 methods) ────────────────────────────────

    /// `ListActionSecrets` list a repository's secrets
    pub async fn list_action_secrets(
        &self,
        owner: &str,
        repo: &str,
        opt: ListOptions,
    ) -> crate::Result<(Vec<Secret>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/secrets?{}",
            escaped[0],
            escaped[1],
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

    /// `ListActionVariables` list a repository's action variables
    pub async fn list_action_variables(
        &self,
        owner: &str,
        repo: &str,
        opt: ListOptions,
    ) -> crate::Result<(Vec<RepoActionVariable>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/variables?{}",
            escaped[0],
            escaped[1],
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

    /// `CreateActionSecret` create a secret for a repository
    pub async fn create_action_secret(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateSecretOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/actions/secrets/{}",
            escaped[0], escaped[1], opt.name
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteActionSecret` delete a secret from a repository
    pub async fn delete_action_secret(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/secrets/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `GetActionVariable` get a repository action variable
    pub async fn get_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<(RepoActionVariable, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateActionVariable` create a repository action variable
    pub async fn create_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        value: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = serde_json::to_string(&serde_json::json!({"value": value}))?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `UpdateActionVariable` update a repository action variable
    pub async fn update_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        value: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = serde_json::to_string(&serde_json::json!({"name": name, "value": value}))?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteActionVariable` delete a repository action variable
    pub async fn delete_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
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
    async fn test_list_action_secrets_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_secret_json("MY_SECRET")])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_secrets("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (secrets, resp) = result.unwrap();
        assert_eq!(secrets.len(), 1);
        assert_eq!(secrets[0].name, "MY_SECRET");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_action_secrets_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_secrets("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_action_variables_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                minimal_repo_action_variable_json("VAR1", "val1")
            ])))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_variables("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (vars, resp) = result.unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "VAR1");
        assert_eq!(vars[0].data, "val1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_action_variables_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_variables("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_action_secret_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let result = client
            .repos()
            .create_action_secret("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_create_action_secret_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let result = client
            .repos()
            .create_action_secret("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_action_secret_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_secret("owner", "repo", "MY_SECRET")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_action_secret_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_secret("owner", "repo", "MY_SECRET")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(minimal_repo_action_variable_json("VAR1", "val1")),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_ok());
        let (var, resp) = result.unwrap();
        assert_eq!(var.name, "VAR1");
        assert_eq!(var.data, "val1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .create_action_variable("owner", "repo", "VAR1", "val1")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_create_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .create_action_variable("owner", "repo", "VAR1", "val1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .update_action_variable("owner", "repo", "VAR1", "newval")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[tokio::test]
    async fn test_update_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .update_action_variable("owner", "repo", "VAR1", "newval")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_err());
    }
}
