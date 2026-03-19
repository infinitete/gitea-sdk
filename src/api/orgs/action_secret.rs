// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::*;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    /// CreateOrgActionSecret creates a secret for the specified organization
    pub async fn create_org_action_secret(
        &self,
        org: &str,
        opt: CreateSecretOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/actions/secrets/{}", escaped[0], opt.name);
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
            201 | 204 => Ok(response),
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── create_org_action_secret ─────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_action_secret_happy_201() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_secret("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_action_secret_happy_204() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_secret("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_create_org_action_secret_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let result = client.orgs().create_org_action_secret("testorg", opt).await;
        assert!(result.is_err());
    }
}
