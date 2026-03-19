// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::user::UpdateUserAvatarOption;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_social.go ─────────────────────────────────────────────────

    /// UpdateUserAvatar updates the authenticated user's avatar
    pub async fn update_avatar(&self, opt: UpdateUserAvatarOption) -> crate::Result<Response> {
        opt.validate()?;
        let body = json_body(&opt)?;
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
                "/user/avatar",
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

    /// DeleteUserAvatar deletes the authenticated user's avatar
    pub async fn delete_avatar(&self) -> crate::Result<Response> {
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                "/user/avatar",
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
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::create_test_client;

    // ── update_avatar ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_avatar_happy() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/avatar"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = UpdateUserAvatarOption {
            image: "base64data".to_string(),
        };
        let resp = client.users().update_avatar(opt).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_update_avatar_validation_empty() {
        let client = create_test_client(&MockServer::start().await);
        let opt = UpdateUserAvatarOption {
            image: String::new(),
        };
        let result = client.users().update_avatar(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_avatar_unexpected_status() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/avatar"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = UpdateUserAvatarOption {
            image: "base64data".to_string(),
        };
        let result = client.users().update_avatar(opt).await;
        assert!(result.is_err());
    }

    // ── delete_avatar ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_avatar_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/avatar"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.users().delete_avatar().await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_avatar_unexpected_status() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/avatar"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().delete_avatar().await;
        assert!(result.is_err());
    }
}
