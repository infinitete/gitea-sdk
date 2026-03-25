// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::query::build_query_string;
use crate::internal::request::{json_body, json_header};
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::PublicKey;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_key.go ────────────────────────────────────────────────────

    /// `ListPublicKeys` list all the public keys of the user
    pub async fn list_public_keys(
        &self,
        user: &str,
        opt: ListPublicKeysOptions,
    ) -> crate::Result<(Vec<PublicKey>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let query = opt.query_encode();
        let base_path = format!("/users/{}/keys", escaped[0]);
        let path = build_query_string(&base_path, &query);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `ListMyPublicKeys` list all the public keys of current user
    pub async fn list_my_public_keys(
        &self,
        opt: ListPublicKeysOptions,
    ) -> crate::Result<(Vec<PublicKey>, Response)> {
        let query = opt.query_encode();
        let path = build_query_string("/user/keys", &query);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetPublicKey` get current user's public key by key id
    pub async fn get_public_key(&self, key_id: i64) -> crate::Result<(PublicKey, Response)> {
        let path = format!("/user/keys/{key_id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreatePublicKey` create public key with options
    pub async fn create_public_key(
        &self,
        opt: CreateKeyOption,
    ) -> crate::Result<(PublicKey, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/keys",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeletePublicKey` delete public key with key id
    pub async fn delete_public_key(&self, key_id: i64) -> crate::Result<Response> {
        let path = format!("/user/keys/{key_id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::create_test_client;

    // ── list_public_keys ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_public_keys_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"id": 1, "key": "ssh-rsa AAAA...", "title": "my-key"}
        ]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/testuser/keys"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (keys, resp) = client
            .users()
            .list_public_keys("testuser", Default::default())
            .await
            .unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_public_keys_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/testuser/keys"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_public_keys("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_public_keys_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .users()
            .list_public_keys("", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_my_public_keys ───────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_public_keys_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"id": 1, "key": "ssh-rsa AAAA...", "title": "my-key"}
        ]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/user/keys"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (keys, resp) = client
            .users()
            .list_my_public_keys(Default::default())
            .await
            .unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_public_keys_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/user/keys"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_my_public_keys(Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_public_key ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_public_key_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "id": 42,
            "key": "ssh-rsa AAAA...",
            "title": "deploy-key"
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/user/keys/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (key, resp) = client.users().get_public_key(42).await.unwrap();
        assert_eq!(key.id, 42);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_public_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/keys/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_public_key(999).await;
        assert!(result.is_err());
    }

    // ── create_public_key ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_public_key_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "id": 10,
            "key": "ssh-rsa AAAA...",
            "title": "new-key"
        });

        Mock::given(method("POST"))
            .and(path("/api/v1/user/keys"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateKeyOption {
            title: "new-key".to_string(),
            key: "ssh-rsa AAAA...".to_string(),
            read_only: false,
        };
        let (key, resp) = client.users().create_public_key(opt).await.unwrap();
        assert_eq!(key.id, 10);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_public_key_validation_empty_key() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateKeyOption {
            title: "my-key".to_string(),
            key: String::new(),
            read_only: false,
        };
        let result = client.users().create_public_key(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_public_key_validation_empty_title() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateKeyOption {
            title: String::new(),
            key: "ssh-rsa AAAA...".to_string(),
            read_only: false,
        };
        let result = client.users().create_public_key(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_public_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/keys"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateKeyOption {
            title: "dup".to_string(),
            key: "ssh-rsa AAAA...".to_string(),
            read_only: false,
        };
        let result = client.users().create_public_key(opt).await;
        assert!(result.is_err());
    }

    // ── delete_public_key ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_public_key_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/keys/42"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.users().delete_public_key(42).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_public_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/keys/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().delete_public_key(999).await;
        assert!(result.is_err());
    }
}
