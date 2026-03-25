// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::query::build_query_string;
use crate::internal::request::json_header;
use crate::options::user::ListUserBlocksOptions;
use crate::pagination::QueryEncode;
use crate::types::User;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_block.go ──────────────────────────────────────────────────

    /// `ListMyBlocks` lists users blocked by the authenticated user
    pub async fn list_my_blocks(
        &self,
        opt: ListUserBlocksOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let query = opt.query_encode();
        let path = build_query_string("/user/blocks", &query);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// `CheckUserBlock` checks if a user is blocked by the authenticated user
    pub async fn check_user_block(&self, username: &str) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/user/blocks/{}", escaped[0]);
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

    /// `BlockUser` blocks a user
    pub async fn block_user(&self, username: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/user/blocks/{}", escaped[0]);
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

    /// `UnblockUser` unblocks a user
    pub async fn unblock_user(&self, username: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/user/blocks/{}", escaped[0]);
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::{create_test_client, user_json};

    // ── list_my_blocks ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_blocks_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "blocked1")]);

        Mock::given(method("GET"))
            .and(path("/api/v1/user/blocks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (blocked, resp) = client
            .users()
            .list_my_blocks(Default::default())
            .await
            .unwrap();
        assert_eq!(blocked.len(), 1);
        assert_eq!(blocked[0].user_name, "blocked1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_blocks_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/blocks"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_my_blocks(Default::default()).await;
        assert!(result.is_err());
    }

    // ── check_user_block ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_check_user_block_true() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/blocks/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (blocked, resp) = client.users().check_user_block("someuser").await.unwrap();
        assert!(blocked);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_user_block_false() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/blocks/someuser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (blocked, resp) = client.users().check_user_block("someuser").await.unwrap();
        assert!(!blocked);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_check_user_block_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().check_user_block("").await;
        assert!(result.is_err());
    }

    // ── block_user ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_block_user_happy() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/user/blocks/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.users().block_user("someuser").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_block_user_unexpected_status() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/user/blocks/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().block_user("someuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_block_user_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().block_user("").await;
        assert!(result.is_err());
    }

    // ── unblock_user ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_unblock_user_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/blocks/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.users().unblock_user("someuser").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unblock_user_unexpected_status() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/blocks/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().unblock_user("someuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unblock_user_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().unblock_user("").await;
        assert!(result.is_err());
    }
}
