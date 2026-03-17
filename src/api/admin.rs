// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::admin::*;
use crate::options::user::CreateKeyOption;
use crate::pagination::QueryEncode;
use crate::types::{Badge, CronTask, Email, Organization, PublicKey, User};

pub struct AdminApi<'a> {
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

impl<'a> AdminApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

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

    // ── admin_repo.go ────────────────────────────────────────────────

    /// List unadopted repositories
    pub async fn list_unadopted_repos(
        &self,
        opt: ListUnadoptedReposOptions,
    ) -> crate::Result<(Vec<String>, Response)> {
        let path = format!("/admin/unadopted?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Adopt an unadopted repository
    pub async fn adopt_unadopted_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/admin/unadopted/{}/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Delete an unadopted repository
    pub async fn delete_unadopted_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/admin/unadopted/{}/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    // ── admin_org.go ─────────────────────────────────────────────────

    /// List all organizations
    pub async fn list_orgs(
        &self,
        opt: AdminListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let path = format!("/admin/orgs?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── admin_cron.go ────────────────────────────────────────────────

    /// List available cron tasks
    pub async fn list_cron_tasks(
        &self,
        opt: ListCronTasksOptions,
    ) -> crate::Result<(Vec<CronTask>, Response)> {
        let path = format!("/admin/cron?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Run a cron task
    pub async fn run_cron_task(&self, task: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[task])?;
        let path = format!("/admin/cron/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    // ── admin_hooks.go ───────────────────────────────────────────────

    // Note: Hook type is not yet defined. Methods are provided with
    // serde_json::Value placeholder for Hook; replace with proper Hook
    // type when hooks.rs is implemented.

    /// List all system webhooks
    pub async fn list_hooks(
        &self,
        opt: ListAdminHooksOptions,
    ) -> crate::Result<(Vec<serde_json::Value>, Response)> {
        let path = format!("/admin/hooks?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Create a system webhook
    pub async fn create_hook(
        &self,
        opt: CreateHookOption,
    ) -> crate::Result<(serde_json::Value, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/admin/hooks",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// Get a system webhook by ID
    pub async fn get_hook(&self, id: i64) -> crate::Result<(serde_json::Value, Response)> {
        let path = format!("/admin/hooks/{id}");
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Edit a system webhook
    pub async fn edit_hook(
        &self,
        id: i64,
        opt: EditHookOption,
    ) -> crate::Result<(serde_json::Value, Response)> {
        let body = json_body(&opt)?;
        let path = format!("/admin/hooks/{id}");
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// Delete a system webhook
    pub async fn delete_hook(&self, id: i64) -> crate::Result<Response> {
        let path = format!("/admin/hooks/{id}");
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

    // ── admin_email.go ───────────────────────────────────────────────

    /// List all email addresses
    pub async fn list_emails(
        &self,
        opt: ListAdminEmailsOptions,
    ) -> crate::Result<(Vec<Email>, Response)> {
        let path = format!("/admin/emails?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Search email addresses
    pub async fn search_emails(
        &self,
        opt: SearchAdminEmailsOptions,
    ) -> crate::Result<(Vec<Email>, Response)> {
        let path = format!("/admin/emails/search?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

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
    async fn test_list_orgs() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "id": 1,
                "name": "myorg",
                "username": "myorg",
                "full_name": "My Org",
                "email": "",
                "avatar_url": "",
                "description": "",
                "website": "",
                "location": "",
                "visibility": "public",
                "repo_admin_change_team_access": false
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/admin/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (orgs, resp) = client.admin().list_orgs(Default::default()).await.unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].user_name, "myorg");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_unadopted_repos() {
        let server = MockServer::start().await;
        let body = serde_json::json!(["org1/repo1", "org2/repo2"]);

        Mock::given(method("GET"))
            .and(path("/api/v1/admin/unadopted"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (repos, resp) = client
            .admin()
            .list_unadopted_repos(Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0], "org1/repo1");
        assert_eq!(resp.status, 200);
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
}
