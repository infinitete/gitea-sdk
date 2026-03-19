// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::admin::UserBadgeOption;
use crate::types::Badge;

use super::AdminApi;

impl<'a> AdminApi<'a> {
    // ── admin_badges.go ──────────────────────────────────────────────

    /// List badges of a user
    pub async fn list_user_badges(&self, username: &str) -> crate::Result<(Vec<Badge>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/admin/users/{}/badges", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Add badges to a user
    pub async fn add_user_badges(
        &self,
        username: &str,
        opt: UserBadgeOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/admin/users/{}/badges", escaped[0]);
        let body = json_body(&opt)?;
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if status != 204 && status != 201 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(response)
    }

    /// Delete badges from a user
    pub async fn delete_user_badges(
        &self,
        username: &str,
        opt: UserBadgeOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/admin/users/{}/badges", escaped[0]);
        let body = json_body(&opt)?;
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
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
    use crate::options::admin::UserBadgeOption;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{badge_json, create_test_client};

    #[tokio::test]
    async fn test_list_user_badges() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                badge_json(1, "contributor"),
                badge_json(2, "reviewer")
            ])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (badges, resp) = client.admin().list_user_badges("testuser").await.unwrap();
        assert_eq!(badges.len(), 2);
        assert_eq!(badges[0].slug, "contributor");
        assert_eq!(badges[1].slug, "reviewer");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_badges_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().list_user_badges("").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_list_user_badges_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_user_badges("testuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_user_badges() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let resp = client
            .admin()
            .add_user_badges("testuser", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_user_badges_created() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let resp = client
            .admin()
            .add_user_badges("testuser", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_add_user_badges_unexpected_status() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let result = client.admin().add_user_badges("testuser", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unexpected status: 500")
        );
    }

    #[tokio::test]
    async fn test_add_user_badges_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let result = client.admin().add_user_badges("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_delete_user_badges() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let resp = client
            .admin()
            .delete_user_badges("testuser", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_user_badges_unexpected_status() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/users/testuser/badges"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let result = client.admin().delete_user_badges("testuser", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unexpected status: 500")
        );
    }

    #[tokio::test]
    async fn test_delete_user_badges_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = UserBadgeOption {
            badge_slugs: vec!["contributor".to_string()],
        };
        let result = client.admin().delete_user_badges("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }
}
