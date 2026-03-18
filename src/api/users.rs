// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! User API endpoints for managing Gitea user accounts, keys, and settings.

use crate::Client;
use crate::Response;
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::user::UserHeatmapData;
use crate::types::{AccessToken, Activity, Email, GPGKey, PublicKey, User, UserSettings};

/// API methods for users. Access via [`Client::users()`](crate::Client::users).
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
    /// Create a new `UsersApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

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
        if users.len() == 1
            && let Some(user) = users.into_iter().next()
        {
            return Ok((user, resp));
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

    // ── user_heatmap.go ───────────────────────────────────────────────────

    /// GetUserHeatmap gets a user's heatmap data.
    pub async fn get_user_heatmap(
        &self,
        username: &str,
    ) -> crate::Result<(Vec<UserHeatmapData>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/users/{}/heatmap", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── user_activity_feeds.go ───────────────────────────────────────────

    /// ListUserActivityFeeds lists a user's activity feeds.
    pub async fn list_user_activity_feeds(
        &self,
        username: &str,
        opt: ListUserActivityFeedsOptions,
    ) -> crate::Result<(Vec<Activity>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let mut path = format!("/users/{}/activities/feeds", escaped[0]);
        let query = opt.query_encode();
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query);
        }
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
    use wiremock::matchers::{method, path, path_regex, query_param};
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
    async fn test_get_user_heatmap() {
        let server = MockServer::start().await;
        let heatmap = serde_json::json!([
            {"timestamp": 1, "contributions": 3},
            {"timestamp": 2, "contributions": 0}
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/heatmap"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&heatmap))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (data, resp) = client.users().get_user_heatmap("testuser").await.unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].contributions, 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_activity_feeds() {
        let server = MockServer::start().await;
        let activities = serde_json::json!([{
            "id": 1,
            "act_user_id": 2,
            "act_user": null,
            "op_type": "create_repo",
            "content": "",
            "repo_id": 3,
            "repo": null,
            "comment_id": 0,
            "comment": null,
            "ref_name": "",
            "is_private": false,
            "user_id": 2,
            "created": "2024-01-15T10:00:00Z"
        }]);

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/activities/feeds"))
            .and(query_param("only-performed-by", "true"))
            .and(query_param("date", "2024-01-15"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&activities))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = ListUserActivityFeedsOptions {
            list_options: Default::default(),
            only_performed_by: true,
            date: "2024-01-15".to_string(),
        };
        let (list, resp) = client
            .users()
            .list_user_activity_feeds("testuser", opt)
            .await
            .unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].op_type, "create_repo");
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

    // ── add_email ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_add_email_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"email": "new@example.com", "verified": false, "primary": false}
        ]);

        Mock::given(method("POST"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateEmailOption {
            emails: vec!["new@example.com".to_string()],
        };
        let (emails, resp) = client.users().add_email(opt).await.unwrap();
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].email, "new@example.com");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_add_email_validation_empty() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateEmailOption { emails: vec![] };
        let result = client.users().add_email(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_email_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/emails"))
            .respond_with(
                ResponseTemplate::new(422)
                    .set_body_json(serde_json::json!({"message": "invalid email"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateEmailOption {
            emails: vec!["bad-email".to_string()],
        };
        let result = client.users().add_email(opt).await;
        assert!(result.is_err());
    }

    // ── delete_email ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_email_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = DeleteEmailOption {
            emails: vec!["old@example.com".to_string()],
        };
        let resp = client.users().delete_email(opt).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_email_validation_empty() {
        let client = create_test_client(&MockServer::start().await);
        let opt = DeleteEmailOption { emails: vec![] };
        let result = client.users().delete_email(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_email_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = DeleteEmailOption {
            emails: vec!["nonexistent@example.com".to_string()],
        };
        let result = client.users().delete_email(opt).await;
        assert!(result.is_err());
    }

    // ── list_emails error ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_emails_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_emails(Default::default()).await;
        assert!(result.is_err());
    }

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

    // ── list_user_activity_feeds error ────────────────────────────────

    #[tokio::test]
    async fn test_list_user_activity_feeds_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/activities/feeds"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_user_activity_feeds("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_user_activity_feeds validation ───────────────────────────

    #[tokio::test]
    async fn test_list_user_activity_feeds_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .users()
            .list_user_activity_feeds("", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_user_heatmap error ────────────────────────────────────────

    #[tokio::test]
    async fn test_get_user_heatmap_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/heatmap"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_user_heatmap("testuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_heatmap_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().get_user_heatmap("").await;
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

    // ── search error ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_search_users_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().search(SearchUsersOption::default()).await;
        assert!(result.is_err());
    }

    // ── get_my_info error ─────────────────────────────────────────────

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

    // ── get_settings ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_settings_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "full_name": "Test User",
            "website": "",
            "description": "",
            "location": "",
            "language": "en-US",
            "theme": "gitea-dark",
            "diff_view_style": "unified",
            "hide_email": false,
            "hide_activity": false
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (settings, resp) = client.users().get_settings().await.unwrap();
        assert_eq!(settings.full_name, "Test User");
        assert_eq!(settings.theme, "gitea-dark");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_settings_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_settings().await;
        assert!(result.is_err());
    }

    // ── update_settings ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_settings_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "full_name": "Updated Name",
            "website": "",
            "description": "",
            "location": "",
            "language": "en-US",
            "theme": "gitea-light",
            "diff_view_style": "unified",
            "hide_email": true,
            "hide_activity": false
        });

        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = UserSettingsOptions {
            full_name: Some("Updated Name".to_string()),
            hide_email: Some(true),
            ..Default::default()
        };
        let (settings, resp) = client.users().update_settings(opt).await.unwrap();
        assert_eq!(settings.full_name, "Updated Name");
        assert!(settings.hide_email);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_settings_error() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/settings"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .update_settings(UserSettingsOptions::default())
            .await;
        assert!(result.is_err());
    }

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

    // ── list_gpg_keys ─────────────────────────────────────────────────

    fn gpg_key_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "primary_key_id": "0",
            "key_id": format!("KEY{id}"),
            "public_key": "-----BEGIN PGP PUBLIC KEY BLOCK-----",
            "emails": [],
            "subs_key": [],
            "can_sign": true,
            "can_encrypt_comms": false,
            "can_encrypt_storage": false,
            "can_certify": true,
            "verified": true
        })
    }

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
