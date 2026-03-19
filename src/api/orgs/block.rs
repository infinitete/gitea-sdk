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
    // ── org_block.go ──────────────────────────────────────────────────────

    /// ListOrgBlocks lists users blocked by the organization
    pub async fn list_org_blocks(
        &self,
        org: &str,
        opt: ListOrgBlocksOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/blocks?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CheckOrgBlock checks if a user is blocked by the organization
    pub async fn check_org_block(
        &self,
        org: &str,
        username: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, username])?;
        let path = format!("/orgs/{}/blocks/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        Ok((status == 204, response))
    }

    /// BlockOrgUser blocks a user from the organization
    pub async fn block_org_user(&self, org: &str, username: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, username])?;
        let path = format!("/orgs/{}/blocks/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(response)
    }

    /// UnblockOrgUser unblocks a user from the organization
    pub async fn unblock_org_user(&self, org: &str, username: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, username])?;
        let path = format!("/orgs/{}/blocks/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_org_blocks ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_blocks_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "baduser")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (users, resp) = client
            .orgs()
            .list_org_blocks("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_name, "baduser");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_blocks_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_blocks("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── check_org_block ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_check_org_block_is_blocked() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_blocked, resp) = client
            .orgs()
            .check_org_block("testorg", "baduser")
            .await
            .unwrap();
        assert!(is_blocked);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_org_block_not_blocked() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks/gooduser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_blocked, resp) = client
            .orgs()
            .check_org_block("testorg", "gooduser")
            .await
            .unwrap();
        assert!(!is_blocked);
        assert_eq!(resp.status, 404);
    }

    // ── block_org_user ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_block_org_user_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .block_org_user("testorg", "baduser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_block_org_user_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().block_org_user("testorg", "baduser").await;
        assert!(result.is_err());
    }

    // ── unblock_org_user ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_unblock_org_user_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .unblock_org_user("testorg", "baduser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unblock_org_user_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().unblock_org_user("testorg", "baduser").await;
        assert!(result.is_err());
    }
}
