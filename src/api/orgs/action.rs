// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::Secret;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    // ── org_action.go ─────────────────────────────────────────────────────

    /// ListOrgActionSecret list an organization's secrets
    pub async fn list_org_action_secrets(
        &self,
        org: &str,
        opt: ListOrgActionSecretOption,
    ) -> crate::Result<(Vec<Secret>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!(
            "/orgs/{}/actions/secrets?{}",
            escaped[0],
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

    /// ListOrgActionVariable lists an organization's action variables
    pub async fn list_org_action_variables(
        &self,
        org: &str,
        opt: ListOrgActionVariableOption,
    ) -> crate::Result<(Vec<OrgActionVariable>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!(
            "/orgs/{}/actions/variables?{}",
            escaped[0],
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

    /// GetOrgActionVariable gets a single organization's action variable by name
    pub async fn get_org_action_variable(
        &self,
        org: &str,
        name: &str,
    ) -> crate::Result<(OrgActionVariable, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, name])?;
        let path = format!("/orgs/{}/actions/variables/{}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CreateOrgActionVariable creates a variable for the specified organization
    pub async fn create_org_action_variable(
        &self,
        org: &str,
        opt: CreateOrgActionVariableOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/actions/variables/{}", escaped[0], opt.name);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        match status {
            201 | 204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
            400 => Err(crate::Error::Validation("bad request".to_string())),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// UpdateOrgActionVariable updates a variable for the specified organization
    pub async fn update_org_action_variable(
        &self,
        org: &str,
        name: &str,
        opt: UpdateOrgActionVariableOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, name])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/actions/variables/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        match status {
            200 | 204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
            400 => Err(crate::Error::Validation("bad request".to_string())),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_org_action_secrets ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_action_secrets_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([secret_json("MY_SECRET")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/secrets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (secrets, resp) = client
            .orgs()
            .list_org_action_secrets("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(secrets.len(), 1);
        assert_eq!(secrets[0].name, "MY_SECRET");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_action_secrets_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/secrets"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_action_secrets("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_org_action_variables ────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_action_variables_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_action_variable_json(1, "MY_VAR")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (vars, resp) = client
            .orgs()
            .list_org_action_variables("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "MY_VAR");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_action_variables_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_action_variables("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_org_action_variable ──────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_action_variable_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(org_action_variable_json(1, "MY_VAR")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (var, resp) = client
            .orgs()
            .get_org_action_variable("testorg", "MY_VAR")
            .await
            .unwrap();
        assert_eq!(var.name, "MY_VAR");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_action_variable_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MISSING"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .get_org_action_variable("testorg", "MISSING")
            .await;
        assert!(result.is_err());
    }

    // ── create_org_action_variable ───────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_action_variable_happy_201() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_action_variable_happy_204() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_create_org_action_variable_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let result = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_action_variable_error_bad_request() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let result = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await;
        assert!(result.is_err());
    }

    // ── update_org_action_variable ───────────────────────────────────────

    #[tokio::test]
    async fn test_update_org_action_variable_happy_200() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateOrgActionVariableOption {
            value: "updated-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .update_org_action_variable("testorg", "MY_VAR", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_org_action_variable_happy_204() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateOrgActionVariableOption {
            value: "updated-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .update_org_action_variable("testorg", "MY_VAR", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_update_org_action_variable_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateOrgActionVariableOption {
            value: "updated-value".to_string(),
            description: None,
        };
        let result = client
            .orgs()
            .update_org_action_variable("testorg", "MY_VAR", opt)
            .await;
        assert!(result.is_err());
    }
}
