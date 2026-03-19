// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::AccessToken;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_app.go ────────────────────────────────────────────────────

    /// ListAccessTokens lists all the access tokens of user (BasicAuth required).
    pub async fn list_access_tokens(
        &self,
        username: &str,
        opt: ListAccessTokensOptions,
    ) -> crate::Result<(Vec<AccessToken>, Response)> {
        if username.is_empty() {
            return Err(crate::Error::Validation(
                "\"username\" not set: only BasicAuth allowed".to_string(),
            ));
        }
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/users/{}/tokens?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CreateAccessToken create one access token with options (BasicAuth required).
    pub async fn create_access_token(
        &self,
        username: &str,
        opt: CreateAccessTokenOption,
    ) -> crate::Result<(AccessToken, Response)> {
        if username.is_empty() {
            return Err(crate::Error::Validation(
                "\"username\" not set: only BasicAuth allowed".to_string(),
            ));
        }
        opt.validate()?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/users/{}/tokens", escaped[0]);
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteAccessToken delete token by name (BasicAuth required).
    pub async fn delete_access_token(
        &self,
        username: &str,
        token_name: &str,
    ) -> crate::Result<Response> {
        if username.is_empty() {
            return Err(crate::Error::Validation(
                "\"username\" not set: only BasicAuth allowed".to_string(),
            ));
        }
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[username, token_name])?;
        let path = format!("/users/{}/tokens/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::create_test_client;

    // ── list_access_tokens ────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_access_tokens_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"id": 1, "name": "my-token", "sha1": "abc123", "token_last_eight": "abc12345"}
        ]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/testuser/tokens"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (tokens, resp) = client
            .users()
            .list_access_tokens("testuser", Default::default())
            .await
            .unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].name, "my-token");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_access_tokens_empty_username() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .users()
            .list_access_tokens("", Default::default())
            .await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("username"));
    }

    #[tokio::test]
    async fn test_list_access_tokens_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/testuser/tokens"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_access_tokens("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── create_access_token ───────────────────────────────────────────

    #[tokio::test]
    async fn test_create_access_token_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "id": 1,
            "name": "new-token",
            "sha1": "sha1hash",
            "token_last_eight": "abc12345"
        });

        Mock::given(method("POST"))
            .and(path("/api/v1/users/testuser/tokens"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateAccessTokenOption {
            name: "new-token".to_string(),
            scopes: vec![],
        };
        let (token, resp) = client
            .users()
            .create_access_token("testuser", opt)
            .await
            .unwrap();
        assert_eq!(token.name, "new-token");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_access_token_empty_username() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateAccessTokenOption {
            name: "tok".to_string(),
            scopes: vec![],
        };
        let result = client.users().create_access_token("", opt).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("username"));
    }

    #[tokio::test]
    async fn test_create_access_token_validation() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateAccessTokenOption {
            name: String::new(),
            scopes: vec![],
        };
        let result = client.users().create_access_token("testuser", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_access_token_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/users/testuser/tokens"))
            .respond_with(ResponseTemplate::new(403))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateAccessTokenOption {
            name: "tok".to_string(),
            scopes: vec![],
        };
        let result = client.users().create_access_token("testuser", opt).await;
        assert!(result.is_err());
    }

    // ── delete_access_token ───────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_access_token_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"^/api/v1/users/testuser/tokens/mytoken"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .users()
            .delete_access_token("testuser", "mytoken")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_access_token_empty_username() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().delete_access_token("", "tok").await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("username"));
    }

    #[tokio::test]
    async fn test_delete_access_token_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"^/api/v1/users/testuser/tokens/mytoken"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .delete_access_token("testuser", "mytoken")
            .await;
        assert!(result.is_err());
    }
}
