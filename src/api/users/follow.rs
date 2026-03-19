// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::User;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    async fn get_following_state(&self, path: &str) -> crate::Result<bool> {
        match self
            .client()
            .get_response(reqwest::Method::GET, path, None, None::<&str>)
            .await
        {
            Ok((_, response)) => {
                if response.status == 204 {
                    Ok(true)
                } else {
                    Err(crate::Error::UnknownApi {
                        status: response.status,
                        body: format!("unexpected status: {}", response.status),
                    })
                }
            }
            Err(crate::Error::Api { status: 404, .. })
            | Err(crate::Error::UnknownApi { status: 404, .. }) => Ok(false),
            Err(err) => Err(err),
        }
    }

    // ── user_follow.go ─────────────────────────────────────────────────

    /// ListMyFollowers list all the followers of current user
    pub async fn list_my_followers(
        &self,
        opt: ListFollowersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/user/followers?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListFollowers list all the followers of one user
    pub async fn list_followers(
        &self,
        user: &str,
        opt: ListFollowersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/followers?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyFollowing list all the users current user followed
    pub async fn list_my_following(
        &self,
        opt: ListFollowingOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/user/following?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListFollowing list all the users the user followed
    pub async fn list_following(
        &self,
        user: &str,
        opt: ListFollowingOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/following?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// IsFollowing if current user followed the target.
    /// Unlike Go SDK, this always propagates errors via Result.
    pub async fn is_following(&self, target: &str) -> crate::Result<bool> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[target])?;
        let path = format!("/user/following/{}", escaped[0]);
        self.get_following_state(&path).await
    }

    /// IsUserFollowing if the user followed the target.
    /// Unlike Go SDK, this always propagates errors via Result.
    pub async fn is_user_following(&self, user: &str, target: &str) -> crate::Result<bool> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user, target])?;
        let path = format!("/users/{}/following/{}", escaped[0], escaped[1]);
        self.get_following_state(&path).await
    }

    /// Follow set current user follow the target
    pub async fn follow(&self, target: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[target])?;
        let path = format!("/user/following/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// Unfollow set current user unfollow the target
    pub async fn unfollow(&self, target: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[target])?;
        let path = format!("/user/following/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::{create_test_client, user_json};

    #[tokio::test]
    async fn test_follow_and_unfollow() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/user/following/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/following/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);

        let resp = client.users().follow("someuser").await.unwrap();
        assert_eq!(resp.status, 204);

        let resp = client.users().unfollow("someuser").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_is_following() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/following/existing"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/following/nonexistent"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);

        let following = client.users().is_following("existing").await.unwrap();
        assert!(following);

        let not_following = client.users().is_following("nonexistent").await.unwrap();
        assert!(!not_following);
    }

    #[tokio::test]
    async fn test_is_following_propagates_unexpected_status() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/following/unauthorized"))
            .respond_with(
                ResponseTemplate::new(401)
                    .set_body_json(serde_json::json!({"message": "token expired"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().is_following("unauthorized").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 401);
                assert_eq!(message, "token expired");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_is_user_following_propagates_unexpected_status() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/source/following/target"))
            .respond_with(ResponseTemplate::new(500).set_body_string("internal error"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().is_user_following("source", "target").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::UnknownApi { status, body } => {
                assert_eq!(status, 500);
                assert_eq!(body, "internal error");
            }
            other => panic!("expected Error::UnknownApi, got: {other}"),
        }
    }

    // ── list_my_followers ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_followers_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "follower1"), user_json(2, "follower2")]);

        Mock::given(method("GET"))
            .and(path("/api/v1/user/followers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (followers, resp) = client
            .users()
            .list_my_followers(Default::default())
            .await
            .unwrap();
        assert_eq!(followers.len(), 2);
        assert_eq!(followers[0].user_name, "follower1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_followers_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/followers"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_my_followers(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_followers ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_followers_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "follower1")]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/someuser/followers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (followers, resp) = client
            .users()
            .list_followers("someuser", Default::default())
            .await
            .unwrap();
        assert_eq!(followers.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_followers_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/someuser/followers"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_followers("someuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_followers_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().list_followers("", Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_my_following ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_following_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "followed1")]);

        Mock::given(method("GET"))
            .and(path("/api/v1/user/following"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (following, resp) = client
            .users()
            .list_my_following(Default::default())
            .await
            .unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_following_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/following"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_my_following(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_following ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_following_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "followed1")]);

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/someuser/following"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (following, resp) = client
            .users()
            .list_following("someuser", Default::default())
            .await
            .unwrap();
        assert_eq!(following.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_following_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/someuser/following"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_following("someuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_following_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().list_following("", Default::default()).await;
        assert!(result.is_err());
    }

    // ── is_user_following happy paths ─────────────────────────────────

    #[tokio::test]
    async fn test_is_user_following_true() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/src/following/tgt"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .is_user_following("src", "tgt")
            .await
            .unwrap();
        assert!(result);
    }

    #[tokio::test]
    async fn test_is_user_following_false() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"^/api/v1/users/src/following/tgt"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .is_user_following("src", "tgt")
            .await
            .unwrap();
        assert!(!result);
    }

    #[tokio::test]
    async fn test_is_user_following_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().is_user_following("", "tgt").await;
        assert!(result.is_err());
    }

    // ── follow / unfollow error paths ─────────────────────────────────

    #[tokio::test]
    async fn test_follow_error() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/user/following/someuser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().follow("someuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unfollow_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/following/someuser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().unfollow("someuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_follow_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().follow("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unfollow_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().unfollow("").await;
        assert!(result.is_err());
    }
}
