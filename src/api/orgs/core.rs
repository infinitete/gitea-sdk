// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::{OrgPermissions, Organization};

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    // ── org.go ────────────────────────────────────────────────────────────

    /// ListOrgs lists all public organizations
    pub async fn list_orgs(
        &self,
        opt: ListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let path = format!("/orgs?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyOrgs list all of current user's organizations
    pub async fn list_my_orgs(
        &self,
        opt: ListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let path = format!("/user/orgs?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListUserOrgs list all of some user's organizations
    pub async fn list_user_orgs(
        &self,
        user: &str,
        opt: ListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/orgs?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetOrg get one organization by name
    pub async fn get_org(&self, org: &str) -> crate::Result<(Organization, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateOrg creates an organization
    pub async fn create_org(
        &self,
        opt: CreateOrgOption,
    ) -> crate::Result<(Organization, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/orgs",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditOrg modify one organization via options
    pub async fn edit_org(&self, org: &str, opt: EditOrgOption) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteOrg deletes an organization
    pub async fn delete_org(&self, org: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetOrgPermissions returns user permissions for specific organization
    pub async fn get_org_permissions(
        &self,
        org: &str,
        user: &str,
    ) -> crate::Result<(OrgPermissions, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user, org])?;
        let path = format!("/users/{}/orgs/{}/permissions", escaped[0], escaped[1]);
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
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_orgs ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_orgs_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_json(1, "org1"), org_json(2, "org2")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (orgs, resp) = client.orgs().list_orgs(Default::default()).await.unwrap();
        assert_eq!(orgs.len(), 2);
        assert_eq!(orgs[0].name, "org1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_orgs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(json!({"message": "Internal Server Error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_orgs(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_my_orgs ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_orgs_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_json(1, "myorg")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/user/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (orgs, resp) = client
            .orgs()
            .list_my_orgs(Default::default())
            .await
            .unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].name, "myorg");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_orgs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/orgs"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_my_orgs(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_user_orgs ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_user_orgs_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_json(1, "userorg")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (orgs, resp) = client
            .orgs()
            .list_user_orgs("testuser", Default::default())
            .await
            .unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].name, "userorg");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_orgs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_user_orgs("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_user_orgs_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.orgs().list_user_orgs("", Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_org ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(200).set_body_json(org_json(1, "testorg")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (org, resp) = client.orgs().get_org("testorg").await.unwrap();
        assert_eq!(org.name, "testorg");
        assert_eq!(org.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org("testorg").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_org_empty_name() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org("").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    // ── create_org ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs"))
            .respond_with(ResponseTemplate::new(201).set_body_json(org_json(3, "neworg")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            full_name: Some("New Org".to_string()),
            ..Default::default()
        };
        let (org, resp) = client.orgs().create_org(opt).await.unwrap();
        assert_eq!(org.name, "neworg");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs"))
            .respond_with(
                ResponseTemplate::new(422).set_body_json(json!({"message": "validation failed"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            ..Default::default()
        };
        let result = client.orgs().create_org(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_validation_empty_name() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: String::new(),
            ..Default::default()
        };
        let result = client.orgs().create_org(opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("org name is required")
        );
    }

    // ── edit_org ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgOption {
            description: Some("updated".to_string()),
            ..Default::default()
        };
        let resp = client.orgs().edit_org("testorg", opt).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgOption {
            description: Some("updated".to_string()),
            ..Default::default()
        };
        let result = client.orgs().edit_org("testorg", opt).await;
        assert!(result.is_err());
    }

    // ── delete_org ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_org("testorg").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_org("testorg").await;
        assert!(result.is_err());
    }

    // ── get_org_permissions ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_permissions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs/testorg/permissions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(org_permissions_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (perms, resp) = client
            .orgs()
            .get_org_permissions("testorg", "testuser")
            .await
            .unwrap();
        assert!(perms.is_owner);
        assert!(perms.can_create_repository);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_permissions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs/testorg/permissions"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .get_org_permissions("testorg", "testuser")
            .await;
        assert!(result.is_err());
    }
}
