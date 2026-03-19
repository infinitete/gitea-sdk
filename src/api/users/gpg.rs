// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::GPGKey;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_gpgkey.go ─────────────────────────────────────────────────

    /// ListGPGKeys list all the GPG keys of the user
    pub async fn list_gpg_keys(
        &self,
        user: &str,
        opt: ListGPGKeysOptions,
    ) -> crate::Result<(Vec<GPGKey>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/gpg_keys?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyGPGKeys list all the GPG keys of current user
    pub async fn list_my_gpg_keys(
        &self,
        opt: ListGPGKeysOptions,
    ) -> crate::Result<(Vec<GPGKey>, Response)> {
        let path = format!("/user/gpg_keys?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetGPGKey get current user's GPG key by key id
    pub async fn get_gpg_key(&self, key_id: i64) -> crate::Result<(GPGKey, Response)> {
        let path = format!("/user/gpg_keys/{key_id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateGPGKey create GPG key with options
    pub async fn create_gpg_key(
        &self,
        opt: CreateGPGKeyOption,
    ) -> crate::Result<(GPGKey, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/gpg_keys",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteGPGKey delete GPG key with key id
    pub async fn delete_gpg_key(&self, key_id: i64) -> crate::Result<Response> {
        let path = format!("/user/gpg_keys/{key_id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// GetGPGKeyVerificationToken gets a verification token for adding a GPG key.
    /// API returns text/plain, not JSON.
    pub async fn get_gpg_key_verification_token(&self) -> crate::Result<(String, Response)> {
        let (body, response) = self
            .client()
            .get_response(
                reqwest::Method::GET,
                "/user/gpg_key_token",
                None,
                None::<&str>,
            )
            .await?;
        Ok((String::from_utf8_lossy(&body).to_string(), response))
    }

    /// VerifyGPGKey verifies a GPG key by submitting a signed verification token.
    pub async fn verify_gpg_key(
        &self,
        opt: VerifyGPGKeyOption,
    ) -> crate::Result<(GPGKey, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/gpg_key_verify",
                Some(&json_header()),
                Some(body),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::{create_test_client, gpg_key_json};

    // ── list_gpg_keys ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_gpg_keys_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([gpg_key_json(1)]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/testuser/gpg_keys"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (keys, resp) = client
            .users()
            .list_gpg_keys("testuser", Default::default())
            .await
            .unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_gpg_keys_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/testuser/gpg_keys"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_gpg_keys("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_gpg_keys_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().list_gpg_keys("", Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_my_gpg_keys ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_gpg_keys_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([gpg_key_json(1), gpg_key_json(2)]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/user/gpg_keys"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (keys, resp) = client
            .users()
            .list_my_gpg_keys(Default::default())
            .await
            .unwrap();
        assert_eq!(keys.len(), 2);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_gpg_keys_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/user/gpg_keys"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_my_gpg_keys(Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_gpg_key ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_gpg_key_happy() {
        let server = MockServer::start().await;
        let body = gpg_key_json(42);

        Mock::given(method("GET"))
            .and(path("/api/v1/user/gpg_keys/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (key, resp) = client.users().get_gpg_key(42).await.unwrap();
        assert_eq!(key.id, 42);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_gpg_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/gpg_keys/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_gpg_key(999).await;
        assert!(result.is_err());
    }

    // ── create_gpg_key ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_gpg_key_happy() {
        let server = MockServer::start().await;
        let body = gpg_key_json(10);

        Mock::given(method("POST"))
            .and(path("/api/v1/user/gpg_keys"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateGPGKeyOption {
            armored_key: "-----BEGIN PGP PUBLIC KEY BLOCK-----".to_string(),
            signature: None,
        };
        let (key, resp) = client.users().create_gpg_key(opt).await.unwrap();
        assert_eq!(key.id, 10);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_gpg_key_validation_empty() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateGPGKeyOption {
            armored_key: String::new(),
            signature: None,
        };
        let result = client.users().create_gpg_key(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_gpg_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/gpg_keys"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateGPGKeyOption {
            armored_key: "-----BEGIN PGP PUBLIC KEY BLOCK-----".to_string(),
            signature: None,
        };
        let result = client.users().create_gpg_key(opt).await;
        assert!(result.is_err());
    }

    // ── delete_gpg_key ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_gpg_key_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/gpg_keys/42"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.users().delete_gpg_key(42).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_gpg_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/gpg_keys/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().delete_gpg_key(999).await;
        assert!(result.is_err());
    }

    // ── get_gpg_key_verification_token ────────────────────────────────

    #[tokio::test]
    async fn test_get_gpg_key_verification_token_happy() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/gpg_key_token"))
            .respond_with(ResponseTemplate::new(200).set_body_string("verification-token-abc"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (token, resp) = client
            .users()
            .get_gpg_key_verification_token()
            .await
            .unwrap();
        assert_eq!(token, "verification-token-abc");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_gpg_key_verification_token_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/gpg_key_token"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_gpg_key_verification_token().await;
        assert!(result.is_err());
    }

    // ── verify_gpg_key ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_verify_gpg_key_happy() {
        let server = MockServer::start().await;
        let body = gpg_key_json(1);

        Mock::given(method("POST"))
            .and(path("/api/v1/user/gpg_key_verify"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = VerifyGPGKeyOption {
            key_id: "KEY1".to_string(),
            signature: "-----BEGIN PGP SIGNATURE-----".to_string(),
        };
        let (key, resp) = client.users().verify_gpg_key(opt).await.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_verify_gpg_key_validation_empty_key_id() {
        let client = create_test_client(&MockServer::start().await);
        let opt = VerifyGPGKeyOption {
            key_id: String::new(),
            signature: "sig".to_string(),
        };
        let result = client.users().verify_gpg_key(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_gpg_key_validation_empty_signature() {
        let client = create_test_client(&MockServer::start().await);
        let opt = VerifyGPGKeyOption {
            key_id: "KEY1".to_string(),
            signature: String::new(),
        };
        let result = client.users().verify_gpg_key(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_verify_gpg_key_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/gpg_key_verify"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = VerifyGPGKeyOption {
            key_id: "KEY1".to_string(),
            signature: "bad-sig".to_string(),
        };
        let result = client.users().verify_gpg_key(opt).await;
        assert!(result.is_err());
    }
}
