// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::{AccessToken, Email, GPGKey, PublicKey, User, UserSettings};

pub struct UsersApi<'a> {
    client: &'a Client,
}

fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

#[derive(serde::Deserialize)]
struct SearchUsersResponse {
    #[serde(default)]
    data: Vec<User>,
}

impl<'a> UsersApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

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
        let opt = SearchUsersOption {
            uid: id,
            ..Default::default()
        };
        let (users, resp) = self.search(opt).await?;
        if users.len() == 1 {
            return Ok((users.into_iter().next().unwrap(), resp));
        }
        Err(crate::Error::Validation(format!(
            "user not found with id {id}"
        )))
    }

    // ── user_search.go ─────────────────────────────────────────────────

    /// SearchUsers finds users by query
    pub async fn search(&self, opt: SearchUsersOption) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/users/search?{}", opt.query_encode());
        let (search_resp, response) = self
            .client()
            .get_parsed_response::<SearchUsersResponse, _>(
                reqwest::Method::GET,
                &path,
                None,
                None::<&str>,
            )
            .await?;
        Ok((search_resp.data, response))
    }

    // ── user_email.go ──────────────────────────────────────────────────

    /// ListEmails all the email addresses of user
    pub async fn list_emails(
        &self,
        opt: ListEmailsOptions,
    ) -> crate::Result<(Vec<Email>, Response)> {
        let path = format!("/user/emails?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddEmail add one email to current user with options
    pub async fn add_email(&self, opt: CreateEmailOption) -> crate::Result<(Vec<Email>, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/emails",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteEmail delete one email of current users
    pub async fn delete_email(&self, opt: DeleteEmailOption) -> crate::Result<Response> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                "/user/emails",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── user_key.go ────────────────────────────────────────────────────

    /// ListPublicKeys list all the public keys of the user
    pub async fn list_public_keys(
        &self,
        user: &str,
        opt: ListPublicKeysOptions,
    ) -> crate::Result<(Vec<PublicKey>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/keys?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyPublicKeys list all the public keys of current user
    pub async fn list_my_public_keys(
        &self,
        opt: ListPublicKeysOptions,
    ) -> crate::Result<(Vec<PublicKey>, Response)> {
        let path = format!("/user/keys?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetPublicKey get current user's public key by key id
    pub async fn get_public_key(&self, key_id: i64) -> crate::Result<(PublicKey, Response)> {
        let path = format!("/user/keys/{key_id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreatePublicKey create public key with options
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

    /// DeletePublicKey delete public key with key id
    pub async fn delete_public_key(&self, key_id: i64) -> crate::Result<Response> {
        let path = format!("/user/keys/{key_id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
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
    pub async fn is_following(&self, target: &str) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[target])?;
        let path = format!("/user/following/{}", escaped[0]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        Ok((status == 204, response))
    }

    /// IsUserFollowing if the user followed the target.
    /// Unlike Go SDK, this always propagates errors via Result.
    pub async fn is_user_following(
        &self,
        user: &str,
        target: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user, target])?;
        let path = format!("/users/{}/following/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        Ok((status == 204, response))
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

    // ── user_settings.go ───────────────────────────────────────────────

    /// GetUserSettings returns user settings
    pub async fn get_settings(&self) -> crate::Result<(UserSettings, Response)> {
        self.client()
            .get_parsed_response(reqwest::Method::GET, "/user/settings", None, None::<&str>)
            .await
    }

    /// UpdateUserSettings returns user settings
    pub async fn update_settings(
        &self,
        opt: UserSettingsOptions,
    ) -> crate::Result<(UserSettings, Response)> {
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                "/user/settings",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── user_block.go ──────────────────────────────────────────────────

    /// ListMyBlocks lists users blocked by the authenticated user
    pub async fn list_my_blocks(
        &self,
        opt: ListUserBlocksOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/user/blocks?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CheckUserBlock checks if a user is blocked by the authenticated user
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

    /// BlockUser blocks a user
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

    /// UnblockUser unblocks a user
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

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn user_json(id: i64, login: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "login": login,
            "login_name": "",
            "source_id": 0,
            "full_name": "",
            "email": "",
            "avatar_url": "",
            "html_url": "",
            "language": "",
            "is_admin": false,
            "restricted": false,
            "active": true,
            "prohibit_login": false,
            "location": "",
            "website": "",
            "description": "",
            "visibility": "public",
            "followers_count": 0,
            "following_count": 0,
            "starred_repos_count": 0
        })
    }

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
    async fn test_list_emails() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"email": "test@example.com", "verified": true, "primary": true},
            {"email": "alt@example.com", "verified": false, "primary": false}
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (emails, resp) = client
            .users()
            .list_emails(Default::default())
            .await
            .unwrap();
        assert_eq!(emails.len(), 2);
        assert_eq!(emails[0].email, "test@example.com");
        assert!(emails[0].verified);
        assert_eq!(resp.status, 200);
    }

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

        let (following, _) = client.users().is_following("existing").await.unwrap();
        assert!(following);

        let (not_following, _) = client.users().is_following("nonexistent").await.unwrap();
        assert!(!not_following);
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
    async fn test_search_users() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": true,
            "data": [user_json(1, "testuser")]
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = SearchUsersOption {
            key_word: "testuser".to_string(),
            ..Default::default()
        };
        let (users, resp) = client.users().search(opt).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_name, "testuser");
        assert_eq!(resp.status, 200);
    }
}
