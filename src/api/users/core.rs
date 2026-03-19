// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::types::User;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user.go ────────────────────────────────────────────────────────

    /// GetUserInfo get user info by user's name
    pub async fn get(&self, username: &str) -> crate::Result<(User, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/users/{}", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetMyUserInfo get user info of current user
    pub async fn get_my_info(&self) -> crate::Result<(User, Response)> {
        self.client()
            .get_parsed_response(reqwest::Method::GET, "/user", None, None::<&str>)
            .await
    }

    /// GetUserByID returns user by a given user ID
    pub async fn get_by_id(&self, id: i64) -> crate::Result<(User, Response)> {
        if id < 0 {
            return Err(crate::Error::Validation(format!("invalid user id {id}")));
        }
        let opt = crate::options::user::SearchUsersOption {
            uid: id,
            ..Default::default()
        };
        let (users, resp) = self.search(opt).await?;
        if users.len() == 1
            && let Some(user) = users.into_iter().next()
        {
            return Ok((user, resp));
        }
        Err(crate::Error::Validation(format!(
            "user not found with id {id}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::{create_test_client, user_json};

    #[tokio::test]
    async fn test_get_my_info() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json(1, "testuser")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (user, resp) = client.users().get_my_info().await.unwrap();
        assert_eq!(user.user_name, "testuser");
        assert_eq!(user.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_user_info() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/otheruser"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json(42, "otheruser")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (user, resp) = client.users().get("otheruser").await.unwrap();
        assert_eq!(user.user_name, "otheruser");
        assert_eq!(user.id, 42);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_error_case() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/nonexistent"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({"message": "User not found"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get("nonexistent").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "User not found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_empty_path_segment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.users().get("").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("path segment [0] is empty"));
    }

    #[tokio::test]
    async fn test_get_my_info_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_my_info().await;
        assert!(result.is_err());
    }

    // ── get_by_id ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_by_id_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": true,
            "data": [user_json(5, "testuser")]
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (user, resp) = client.users().get_by_id(5).await.unwrap();
        assert_eq!(user.id, 5);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_by_id_negative_id() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().get_by_id(-1).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("invalid user id"));
    }

    #[tokio::test]
    async fn test_get_by_id_not_found() {
        let server = MockServer::start().await;
        let body = serde_json::json!({"ok": true, "data": []});

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_by_id(999).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("user not found"));
    }

    #[tokio::test]
    async fn test_get_by_id_multiple_results() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": true,
            "data": [user_json(1, "user1"), user_json(2, "user2")]
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_by_id(1).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("user not found"));
    }
}
