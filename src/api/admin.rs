// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Admin API endpoints for Gitea instance administration tasks.

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::admin::*;
use crate::options::org::CreateOrgOption;
use crate::options::repo::CreateRepoOption;
use crate::options::user::CreateKeyOption;
use crate::pagination::QueryEncode;
use crate::types::{Badge, CronTask, Email, Hook, Organization, PublicKey, Repository, User};

/// API methods for admin tasks. Access via [`Client::admin()`](crate::Client::admin).
pub struct AdminApi<'a> {
    client: &'a Client,
}

impl<'a> AdminApi<'a> {
    /// Create a new `AdminApi` view.
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

    /// Create an organization for an existing user
    pub async fn create_org_for_user(
        &self,
        user: &str,
        opt: CreateOrgOption,
    ) -> crate::Result<(Organization, Response)> {
        opt.validate()?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}/orgs", escaped[0]);
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

    /// Create a repository for a user
    pub async fn create_repo_for_user(
        &self,
        user: &str,
        opt: CreateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        opt.validate()?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}/repos", escaped[0]);
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

    /// List all system webhooks
    pub async fn list_hooks(
        &self,
        opt: ListAdminHooksOptions,
    ) -> crate::Result<(Vec<Hook>, Response)> {
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
    pub async fn create_hook(&self, opt: CreateHookOption) -> crate::Result<(Hook, Response)> {
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
    pub async fn get_hook(&self, id: i64) -> crate::Result<(Hook, Response)> {
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
    pub async fn edit_hook(&self, id: i64, opt: EditHookOption) -> crate::Result<(Hook, Response)> {
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
    use crate::options::org::CreateOrgOption;
    use crate::options::repo::CreateRepoOption;
    #[allow(unused_imports)]
    use crate::{Hook, HookType, Repository, TrustModel};
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

    fn org_json() -> serde_json::Value {
        serde_json::json!({
            "id": 99,
            "name": "neworg",
            "username": "neworg",
            "full_name": "New Org",
            "email": "org@example.com",
            "avatar_url": "https://example.com/avatar.png",
            "description": "Org desc",
            "website": "https://example.com",
            "location": "Earth",
            "visibility": "public",
            "repo_admin_change_team_access": true
        })
    }

    fn repo_json() -> serde_json::Value {
        let mut repo = serde_json::Map::new();
        repo.insert("id".to_string(), serde_json::Value::from(2));
        repo.insert("owner".to_string(), serde_json::Value::Null);
        repo.insert("name".to_string(), serde_json::Value::from("newrepo"));
        repo.insert(
            "full_name".to_string(),
            serde_json::Value::from("janedoe/newrepo"),
        );
        repo.insert(
            "description".to_string(),
            serde_json::Value::from("A new repo"),
        );
        repo.insert("empty".to_string(), serde_json::Value::from(true));
        repo.insert("private".to_string(), serde_json::Value::from(false));
        repo.insert("fork".to_string(), serde_json::Value::from(false));
        repo.insert("template".to_string(), serde_json::Value::from(false));
        repo.insert("parent".to_string(), serde_json::Value::Null);
        repo.insert("mirror".to_string(), serde_json::Value::from(false));
        repo.insert("size".to_string(), serde_json::Value::from(0));
        repo.insert("language".to_string(), serde_json::Value::from("Rust"));
        repo.insert(
            "languages_url".to_string(),
            serde_json::Value::from("https://example.com/langs"),
        );
        repo.insert(
            "html_url".to_string(),
            serde_json::Value::from("https://example.com/janedoe/newrepo"),
        );
        repo.insert(
            "url".to_string(),
            serde_json::Value::from("https://api.example.com/repos/janedoe/newrepo"),
        );
        repo.insert(
            "link".to_string(),
            serde_json::Value::from("https://example.com/janedoe/newrepo"),
        );
        repo.insert(
            "ssh_url".to_string(),
            serde_json::Value::from("git@example.com:janedoe/newrepo.git"),
        );
        repo.insert(
            "clone_url".to_string(),
            serde_json::Value::from("https://example.com/janedoe/newrepo.git"),
        );
        repo.insert(
            "original_url".to_string(),
            serde_json::Value::from("https://example.com/janedoe/newrepo.git"),
        );
        repo.insert("website".to_string(), serde_json::Value::from(""));
        repo.insert("stars_count".to_string(), serde_json::Value::from(0));
        repo.insert("forks_count".to_string(), serde_json::Value::from(0));
        repo.insert("watchers_count".to_string(), serde_json::Value::from(0));
        repo.insert("open_issues_count".to_string(), serde_json::Value::from(0));
        repo.insert("open_pr_counter".to_string(), serde_json::Value::from(0));
        repo.insert("release_counter".to_string(), serde_json::Value::from(0));
        repo.insert(
            "default_branch".to_string(),
            serde_json::Value::from("main"),
        );
        repo.insert("archived".to_string(), serde_json::Value::from(false));
        repo.insert(
            "archived_at".to_string(),
            serde_json::Value::from("2026-01-01T00:00:00Z"),
        );
        repo.insert(
            "created_at".to_string(),
            serde_json::Value::from("2026-01-01T00:00:00Z"),
        );
        repo.insert(
            "updated_at".to_string(),
            serde_json::Value::from("2026-01-02T00:00:00Z"),
        );
        repo.insert("permissions".to_string(), serde_json::Value::Null);
        repo.insert("has_issues".to_string(), serde_json::Value::from(true));
        repo.insert("has_code".to_string(), serde_json::Value::from(true));
        repo.insert("internal_tracker".to_string(), serde_json::Value::Null);
        repo.insert("external_tracker".to_string(), serde_json::Value::Null);
        repo.insert("has_wiki".to_string(), serde_json::Value::from(true));
        repo.insert("external_wiki".to_string(), serde_json::Value::Null);
        repo.insert(
            "has_pull_requests".to_string(),
            serde_json::Value::from(true),
        );
        repo.insert("has_projects".to_string(), serde_json::Value::from(true));
        repo.insert("has_releases".to_string(), serde_json::Value::from(true));
        repo.insert("has_packages".to_string(), serde_json::Value::from(false));
        repo.insert("has_actions".to_string(), serde_json::Value::from(false));
        repo.insert(
            "ignore_whitespace_conflicts".to_string(),
            serde_json::Value::from(false),
        );
        repo.insert(
            "allow_fast_forward_only_merge".to_string(),
            serde_json::Value::from(false),
        );
        repo.insert(
            "allow_merge_commits".to_string(),
            serde_json::Value::from(true),
        );
        repo.insert("allow_rebase".to_string(), serde_json::Value::from(true));
        repo.insert(
            "allow_rebase_explicit".to_string(),
            serde_json::Value::from(true),
        );
        repo.insert(
            "allow_rebase_update".to_string(),
            serde_json::Value::from(true),
        );
        repo.insert(
            "allow_squash_merge".to_string(),
            serde_json::Value::from(true),
        );
        repo.insert(
            "default_allow_maintainer_edit".to_string(),
            serde_json::Value::from(true),
        );
        repo.insert("avatar_url".to_string(), serde_json::Value::from(""));
        repo.insert("internal".to_string(), serde_json::Value::from(false));
        repo.insert("mirror_interval".to_string(), serde_json::Value::from(""));
        repo.insert("mirror_updated".to_string(), serde_json::Value::Null);
        repo.insert(
            "default_merge_style".to_string(),
            serde_json::Value::from("merge"),
        );
        repo.insert("projects_mode".to_string(), serde_json::Value::Null);
        repo.insert(
            "default_delete_branch_after_merge".to_string(),
            serde_json::Value::from(false),
        );
        repo.insert(
            "object_format_name".to_string(),
            serde_json::Value::from(""),
        );
        repo.insert("topics".to_string(), serde_json::Value::Array(vec![]));
        repo.insert("licenses".to_string(), serde_json::Value::Array(vec![]));
        repo.insert("repo_transfer".to_string(), serde_json::Value::Null);
        serde_json::Value::Object(repo)
    }

    fn hook_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "type": "slack",
            "branch_filter": "",
            "config": {"url": "https://example.com/hook"},
            "events": ["push"],
            "authorization_header": "",
            "active": true,
            "updated_at": "2024-01-15T10:00:00Z",
            "created_at": "2024-01-15T10:00:00Z"
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
    async fn test_create_org_for_user() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/janedoe/orgs"))
            .respond_with(ResponseTemplate::new(201).set_body_json(org_json()))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            ..Default::default()
        };
        let (org, resp) = client
            .admin()
            .create_org_for_user("janedoe", opt)
            .await
            .unwrap();
        assert_eq!(org.user_name, "neworg");
        assert!(org.repo_admin_change_team_access);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_repo_for_user() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/janedoe/repos"))
            .respond_with(ResponseTemplate::new(201).set_body_json(repo_json()))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: "newrepo".to_string(),
            description: "desc".to_string(),
            private: false,
            issue_labels: "".to_string(),
            auto_init: false,
            template: false,
            gitignores: "".to_string(),
            license: "".to_string(),
            readme: "".to_string(),
            default_branch: "main".to_string(),
            trust_model: TrustModel::Default,
            object_format_name: "".to_string(),
        };
        let (repo, resp) = client
            .admin()
            .create_repo_for_user("janedoe", opt)
            .await
            .unwrap();
        assert_eq!(repo.name, "newrepo");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_list_hooks_returns_hook_struct() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/admin/hooks"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([hook_json()])),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hooks, resp) = client.admin().list_hooks(Default::default()).await.unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].hook_type, HookType::Slack);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_hook_returns_hook_struct() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/hooks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(hook_json()))
            .mount(&server)
            .await;

        let mut config = std::collections::HashMap::new();
        config.insert("url".to_string(), "https://example.com/hook".to_string());
        let opt = CreateHookOption {
            hook_type: HookType::Slack,
            config,
            events: vec!["push".to_string()],
            branch_filter: String::new(),
            active: true,
            authorization_header: String::new(),
        };

        let client = create_test_client(&server);
        let (hook, resp) = client.admin().create_hook(opt).await.unwrap();
        assert_eq!(hook.id, 1);
        assert_eq!(hook.hook_type, HookType::Slack);
        assert_eq!(resp.status, 201);
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
    async fn test_list_orgs_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/orgs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_orgs(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_for_user_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "".to_string(),
            ..Default::default()
        };
        let result = client.admin().create_org_for_user("janedoe", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_for_user_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            ..Default::default()
        };
        let result = client.admin().create_org_for_user("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_create_repo_for_user_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: String::new(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        let result = client.admin().create_repo_for_user("janedoe", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_repo_for_user_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        let result = client.admin().create_repo_for_user("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_list_unadopted_repos_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/unadopted"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .admin()
            .list_unadopted_repos(Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_hooks_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/hooks"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_hooks(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_hook_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateHookOption {
            hook_type: HookType::Unknown,
            config: std::collections::HashMap::new(),
            events: vec![],
            branch_filter: String::new(),
            active: false,
            authorization_header: String::new(),
        };
        let result = client.admin().create_hook(opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("hook type is required")
        );
    }

    #[tokio::test]
    async fn test_create_hook_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/hooks"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let mut config = std::collections::HashMap::new();
        config.insert("url".to_string(), "https://example.com/hook".to_string());
        let opt = CreateHookOption {
            hook_type: HookType::Slack,
            config,
            events: vec!["push".to_string()],
            branch_filter: String::new(),
            active: true,
            authorization_header: String::new(),
        };
        let result = client.admin().create_hook(opt).await;
        assert!(result.is_err());
    }

    fn public_key_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "key": "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC...",
            "title": "my-key",
            "fingerprint": "SHA256:abc123",
            "created": "2024-01-15T10:00:00Z",
            "last_used_at": "2024-01-16T10:00:00Z"
        })
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
    async fn test_adopt_unadopted_repo() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .admin()
            .adopt_unadopted_repo("org1", "repo1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo_empty_owner() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().adopt_unadopted_repo("", "repo1").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo_empty_repo() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().adopt_unadopted_repo("org1", "").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [1] is empty")
        );
    }

    #[tokio::test]
    async fn test_adopt_unadopted_repo_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().adopt_unadopted_repo("org1", "repo1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .admin()
            .delete_unadopted_repo("org1", "repo1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo_empty_owner() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().delete_unadopted_repo("", "repo1").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo_empty_repo() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().delete_unadopted_repo("org1", "").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [1] is empty")
        );
    }

    #[tokio::test]
    async fn test_delete_unadopted_repo_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/unadopted/org1/repo1"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().delete_unadopted_repo("org1", "repo1").await;
        assert!(result.is_err());
    }

    fn cron_task_json(name: &str) -> serde_json::Value {
        serde_json::json!({
            "name": name,
            "schedule": "@daily",
            "next": "2024-02-01T00:00:00Z",
            "prev": "2024-01-31T00:00:00Z",
            "exec_times": 10
        })
    }

    #[tokio::test]
    async fn test_list_cron_tasks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/cron"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                cron_task_json("cleanup"),
                cron_task_json("resync")
            ])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tasks, resp) = client
            .admin()
            .list_cron_tasks(Default::default())
            .await
            .unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].name, "cleanup");
        assert_eq!(tasks[1].name, "resync");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_cron_tasks_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/cron"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_cron_tasks(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_run_cron_task() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/cron/cleanup"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.admin().run_cron_task("cleanup").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_run_cron_task_empty_task() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.admin().run_cron_task("").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_run_cron_task_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/admin/cron/cleanup"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().run_cron_task("cleanup").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_hook() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/hooks/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(hook_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (hook, resp) = client.admin().get_hook(1).await.unwrap();
        assert_eq!(hook.id, 1);
        assert_eq!(hook.hook_type, HookType::Slack);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().get_hook(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_hook() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/admin/hooks/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(hook_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditHookOption {
            active: Some(false),
            ..Default::default()
        };
        let (hook, resp) = client.admin().edit_hook(1, opt).await.unwrap();
        assert_eq!(hook.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_hook_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/admin/hooks/1"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditHookOption::default();
        let result = client.admin().edit_hook(1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_hook() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/hooks/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.admin().delete_hook(1).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_hook_unexpected_status() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/hooks/1"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().delete_hook(1).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unexpected status: 500")
        );
    }

    fn email_json(email: &str) -> serde_json::Value {
        serde_json::json!({
            "email": email,
            "verified": true,
            "primary": false,
            "user_id": 1,
            "username": "testuser"
        })
    }

    #[tokio::test]
    async fn test_list_emails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                email_json("a@example.com"),
                email_json("b@example.com")
            ])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (emails, resp) = client
            .admin()
            .list_emails(Default::default())
            .await
            .unwrap();
        assert_eq!(emails.len(), 2);
        assert_eq!(emails[0].email, "a@example.com");
        assert_eq!(emails[1].email, "b@example.com");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_emails_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_emails(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_emails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([email_json("search@example.com")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchAdminEmailsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let (emails, resp) = client.admin().search_emails(opt).await.unwrap();
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].email, "search@example.com");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_emails_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails/search"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().search_emails(Default::default()).await;
        assert!(result.is_err());
    }

    fn badge_json(id: i64, slug: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "slug": slug,
            "description": "A badge",
            "image_url": "https://example.com/badge.png"
        })
    }

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

    #[tokio::test]
    async fn test_delete_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/admin/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().delete_hook(999).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("unexpected status: 404")
        );
    }
}
