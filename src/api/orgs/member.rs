// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::User;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    // ── org_member.go ─────────────────────────────────────────────────────

    /// `DeleteOrgMembership` remove a member from an organization
    pub async fn delete_org_membership(&self, org: &str, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/members/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `ListOrgMembership` list an organization's members
    pub async fn list_org_membership(
        &self,
        org: &str,
        opt: ListOrgMembershipOption,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/members?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// `ListPublicOrgMembership` list an organization's public members
    pub async fn list_public_org_membership(
        &self,
        org: &str,
        opt: ListOrgMembershipOption,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/public_members?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// `CheckOrgMembership` check if a user is a member of an organization
    pub async fn check_org_membership(
        &self,
        org: &str,
        user: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/members/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        match status {
            204 => Ok((true, response)),
            404 => Ok((false, response)),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// `CheckPublicOrgMembership` check if a user is a public member of an organization
    pub async fn check_public_org_membership(
        &self,
        org: &str,
        user: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/public_members/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        match status {
            204 => Ok((true, response)),
            404 => Ok((false, response)),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// `SetPublicOrgMembership` publicize or conceal a user's membership
    pub async fn set_public_org_membership(
        &self,
        org: &str,
        user: &str,
        visible: bool,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/public_members/{}", escaped[0], escaped[1]);
        let method = if visible {
            reqwest::Method::PUT
        } else {
            reqwest::Method::DELETE
        };
        let (status, response) = self
            .client()
            .get_status_code(method, &path, None, None::<&str>)
            .await?;
        match status {
            204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
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
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── delete_org_membership ────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_membership_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/members/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .delete_org_membership("testorg", "someuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_membership_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/members/someuser"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .delete_org_membership("testorg", "someuser")
            .await;
        assert!(result.is_err());
    }

    // ── list_org_membership ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_membership_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "member1")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (members, resp) = client
            .orgs()
            .list_org_membership("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_name, "member1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_membership_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_membership("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_public_org_membership ───────────────────────────────────────

    #[tokio::test]
    async fn test_list_public_org_membership_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "pubmember")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (members, resp) = client
            .orgs()
            .list_public_org_membership("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_name, "pubmember");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_public_org_membership_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_public_org_membership("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── check_org_membership ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_check_org_membership_is_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/exists"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_org_membership("testorg", "exists")
            .await
            .unwrap();
        assert!(is_member);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_org_membership_not_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/notexists"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_org_membership("testorg", "notexists")
            .await
            .unwrap();
        assert!(!is_member);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_check_org_membership_error_unexpected() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .check_org_membership("testorg", "someuser")
            .await;
        assert!(result.is_err());
    }

    // ── check_public_org_membership ──────────────────────────────────────

    #[tokio::test]
    async fn test_check_public_org_membership_is_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members/exists"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_public_org_membership("testorg", "exists")
            .await
            .unwrap();
        assert!(is_member);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_public_org_membership_not_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members/notexists"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_public_org_membership("testorg", "notexists")
            .await
            .unwrap();
        assert!(!is_member);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_check_public_org_membership_error_unexpected() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .check_public_org_membership("testorg", "someuser")
            .await;
        assert!(result.is_err());
    }

    // ── set_public_org_membership ────────────────────────────────────────

    #[tokio::test]
    async fn test_set_public_org_membership_publicize() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .set_public_org_membership("testorg", "someuser", true)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_set_public_org_membership_conceal() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .set_public_org_membership("testorg", "someuser", false)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_set_public_org_membership_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .set_public_org_membership("testorg", "someuser", true)
            .await;
        assert!(result.is_err());
    }
}
