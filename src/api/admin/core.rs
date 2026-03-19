// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::admin::*;
use crate::options::user::CreateKeyOption;
use crate::pagination::QueryEncode;
use crate::types::{PublicKey, User};

use super::AdminApi;

impl<'a> AdminApi<'a> {
    // ── admin_user.go ────────────────────────────────────────────────

    /// List all users
    pub async fn list_users(
        &self,
        opt: AdminListUsersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/admin/users?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// Create a user
    pub async fn create_user(&self, opt: CreateUserOption) -> crate::Result<(User, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/admin/users",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// Edit a user
    pub async fn edit_user(&self, user: &str, opt: EditUserOption) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}", escaped[0]);
        let body = json_body(&opt)?;
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// Delete a user
    pub async fn delete_user(&self, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// Rename a user
    pub async fn rename_user(
        &self,
        username: &str,
        opt: RenameUserOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/admin/users/{}/rename", escaped[0]);
        let body = json_body(&opt)?;
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// Add a public key for a user
    pub async fn create_user_public_key(
        &self,
        user: &str,
        opt: CreateKeyOption,
    ) -> crate::Result<(PublicKey, Response)> {
        opt.validate()?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}/keys", escaped[0]);
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

    /// Delete a user's public key
    pub async fn delete_user_public_key(&self, user: &str, key_id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}/keys/{key_id}", escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::options::admin::{CreateUserOption, EditUserOption, RenameUserOption};
    use crate::options::user::CreateKeyOption;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, public_key_json, user_json};

    #[tokio::test]
    async fn test_list_users() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "admin"), user_json(2, "user1")]);

        Mock::given(method("GET"))
            .and(path("/api/v1/admin/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (users, resp) = client.admin().list_users(Default::default()).await.unwrap();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].user_name, "admin");
        assert_eq!(users[1].user_name, "user1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_user() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users"))
            .respond_with(ResponseTemplate::new(201).set_body_json(user_json(3, "newuser")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateUserOption {
            username: "newuser".to_string(),
            email: "newuser@example.com".to_string(),
            password: "secret123".to_string(),
            ..Default::default()
        };
        let (user, resp) = client.admin().create_user(opt).await.unwrap();
        assert_eq!(user.user_name, "newuser");
        assert_eq!(user.id, 3);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_user_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);

        let opt = CreateUserOption {
            username: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            ..Default::default()
        };
        let result = client.admin().create_user(opt).await;
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("username is empty"));
    }

    #[tokio::test]
    async fn test_delete_user() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/users/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.admin().delete_user("someuser").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_edit_user() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/api/v1/admin/users/someuser"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = EditUserOption {
            full_name: Some("New Name".to_string()),
            ..Default::default()
        };
        let resp = client.admin().edit_user("someuser", opt).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_rename_user() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/oldname/rename"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = RenameUserOption {
            new_username: "newname".to_string(),
        };
        let resp = client.admin().rename_user("oldname", opt).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_empty_path_segment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().delete_user("").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("path segment [0] is empty"));
    }

    #[tokio::test]
    async fn test_list_users_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/users"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(serde_json::json!({"message": "internal error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_users(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users"))
            .respond_with(
                ResponseTemplate::new(409)
                    .set_body_json(serde_json::json!({"message": "user exists"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateUserOption {
            username: "existing".to_string(),
            email: "existing@example.com".to_string(),
            password: "secret123".to_string(),
            ..Default::default()
        };
        let result = client.admin().create_user(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/users/someuser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().delete_user("someuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_user_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/admin/users/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditUserOption {
            full_name: Some("New Name".to_string()),
            ..Default::default()
        };
        let result = client.admin().edit_user("someuser", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_user_empty_username() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = EditUserOption::default();
        let result = client.admin().edit_user("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_rename_user_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/oldname/rename"))
            .respond_with(ResponseTemplate::new(409))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = RenameUserOption {
            new_username: "newname".to_string(),
        };
        let result = client.admin().rename_user("oldname", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_rename_user_empty_username() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = RenameUserOption {
            new_username: "target".to_string(),
        };
        let result = client.admin().rename_user("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_create_user_public_key() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/janedoe/keys"))
            .respond_with(ResponseTemplate::new(201).set_body_json(public_key_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateKeyOption {
            title: "my-key".to_string(),
            key: "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC...".to_string(),
            read_only: false,
        };
        let (pk, resp) = client
            .admin()
            .create_user_public_key("janedoe", opt)
            .await
            .unwrap();
        assert_eq!(pk.id, 1);
        assert_eq!(pk.title.as_deref(), Some("my-key"));
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_user_public_key_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateKeyOption {
            title: String::new(),
            key: String::new(),
            read_only: false,
        };
        let result = client.admin().create_user_public_key("janedoe", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_public_key_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateKeyOption {
            title: "my-key".to_string(),
            key: "ssh-rsa AAAA...".to_string(),
            read_only: false,
        };
        let result = client.admin().create_user_public_key("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_create_user_public_key_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/janedoe/keys"))
            .respond_with(ResponseTemplate::new(422))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateKeyOption {
            title: "my-key".to_string(),
            key: "ssh-rsa AAAAB3NzaC1yc2E...".to_string(),
            read_only: false,
        };
        let result = client.admin().create_user_public_key("janedoe", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_public_key() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/users/janedoe/keys/42"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .admin()
            .delete_user_public_key("janedoe", 42)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_user_public_key_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().delete_user_public_key("", 1).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_delete_user_public_key_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/users/janedoe/keys/42"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().delete_user_public_key("janedoe", 42).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_users_empty_result() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/users"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (users, resp) = client.admin().list_users(Default::default()).await.unwrap();
        assert!(users.is_empty());
        assert_eq!(resp.status, 200);
    }
}
