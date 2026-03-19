// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Webhook API endpoints for managing Gitea repository and organization webhooks.

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::hook::*;
use crate::pagination::QueryEncode;
use crate::types::Hook;

/// API methods for webhooks. Access via [`Client::hooks()`](crate::Client::hooks).
pub struct HooksApi<'a> {
    client: &'a Client,
}

impl<'a> HooksApi<'a> {
    /// Create a new `HooksApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    // ── org hooks ─────────────────────────────────────────────────────

    /// ListOrgHooks list all the hooks of one organization
    pub async fn list_org_hooks(
        &self,
        org: &str,
        opt: ListHooksOptions,
    ) -> crate::Result<(Vec<Hook>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/hooks?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetOrgHook get a hook of an organization
    pub async fn get_org_hook(&self, org: &str, id: i64) -> crate::Result<(Hook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/hooks/{id}", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateOrgHook create one hook for an organization, with options
    pub async fn create_org_hook(
        &self,
        org: &str,
        opt: CreateHookOption,
    ) -> crate::Result<(Hook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/hooks", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditOrgHook modify one hook of an organization, with hook id and options
    pub async fn edit_org_hook(
        &self,
        org: &str,
        id: i64,
        opt: EditHookOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/hooks/{id}", escaped[0]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteOrgHook delete one hook from an organization, with hook id
    pub async fn delete_org_hook(&self, org: &str, id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/hooks/{id}", escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo hooks ────────────────────────────────────────────────────

    /// ListRepoHooks list all the hooks of one repository
    pub async fn list_repo_hooks(
        &self,
        owner: &str,
        repo: &str,
        opt: ListHooksOptions,
    ) -> crate::Result<(Vec<Hook>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/hooks?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRepoHook get a hook of a repository
    pub async fn get_repo_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Hook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/hooks/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateRepoHook create one hook for a repository, with options
    pub async fn create_repo_hook(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateHookOption,
    ) -> crate::Result<(Hook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/hooks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditRepoHook modify one hook of a repository, with hook id and options
    pub async fn edit_repo_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditHookOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/hooks/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteRepoHook delete one hook from a repository, with hook id
    pub async fn delete_repo_hook(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/hooks/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── user hooks ────────────────────────────────────────────────────

    /// ListMyHooks list all the hooks of the authenticated user
    pub async fn list_my_hooks(
        &self,
        opt: ListHooksOptions,
    ) -> crate::Result<(Vec<Hook>, Response)> {
        let path = format!("/user/hooks?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetMyHook get a hook of the authenticated user
    pub async fn get_my_hook(&self, id: i64) -> crate::Result<(Hook, Response)> {
        let path = format!("/user/hooks/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateMyHook create one hook for the authenticated user, with options
    pub async fn create_my_hook(&self, opt: CreateHookOption) -> crate::Result<(Hook, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/hooks",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditMyHook modify one hook of the authenticated user, with hook id and options
    pub async fn edit_my_hook(&self, id: i64, opt: EditHookOption) -> crate::Result<Response> {
        let body = json_body(&opt)?;
        let path = format!("/user/hooks/{id}");
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteMyHook delete one hook from the authenticated user, with hook id
    pub async fn delete_my_hook(&self, id: i64) -> crate::Result<Response> {
        let path = format!("/user/hooks/{id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::HookType;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn hook_json(id: i64, hook_type: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "type": hook_type,
            "branch_filter": "",
            "config": {"url": "https://example.com/hook"},
            "events": ["push"],
            "authorization_header": "",
            "active": true,
            "updated_at": "2024-01-15T10:00:00Z",
            "created_at": "2024-01-15T10:00:00Z"
        })
    }

    fn create_hook_option() -> CreateHookOption {
        let mut config = std::collections::HashMap::new();
        config.insert("url".to_string(), "https://example.com/hook".to_string());
        CreateHookOption {
            hook_type: HookType::Gitea,
            config,
            events: vec!["push".to_string()],
            branch_filter: None,
            active: true,
            authorization_header: None,
        }
    }

    #[tokio::test]
    async fn test_list_org_hooks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/hooks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec![hook_json(1, "slack")]))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hooks, resp) = client
            .hooks()
            .list_org_hooks("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_repo_hook() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(hook_json(2, "gitea")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let mut config = std::collections::HashMap::new();
        config.insert("url".to_string(), "https://example.com/hook".to_string());
        let opt = CreateHookOption {
            hook_type: HookType::Gitea,
            config,
            events: vec!["push".to_string()],
            branch_filter: None,
            active: true,
            authorization_header: None,
        };
        let (hook, resp) = client
            .hooks()
            .create_repo_hook("testowner", "testrepo", opt)
            .await
            .unwrap();
        assert_eq!(hook.id, 2);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_delete_org_hook() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/hooks/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.hooks().delete_org_hook("testorg", 1).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_get_org_hook() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/hooks/3"))
            .respond_with(ResponseTemplate::new(200).set_body_json(hook_json(3, "discord")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hook, resp) = client.hooks().get_org_hook("testorg", 3).await.unwrap();
        assert_eq!(hook.id, 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_org_hook() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/hooks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(hook_json(10, "gitea")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = create_hook_option();
        let (hook, resp) = client
            .hooks()
            .create_org_hook("testorg", opt)
            .await
            .unwrap();
        assert_eq!(hook.id, 10);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_edit_org_hook() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg/hooks/4"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = EditHookOption {
            active: Some(true),
            events: Some(vec!["pull_request".to_string()]),
            ..Default::default()
        };
        let resp = client
            .hooks()
            .edit_org_hook("testorg", 4, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    // ── org hook error paths ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_hooks_empty_org() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.hooks().list_org_hooks("", Default::default()).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));
    }

    #[tokio::test]
    async fn test_get_org_hook_empty_org() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.hooks().get_org_hook("", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_hook_empty_org() {
        let client = create_test_client(&MockServer::start().await);
        let opt = create_hook_option();
        let result = client.hooks().create_org_hook("", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_hook_unknown_type() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let mut config = std::collections::HashMap::new();
        config.insert("url".to_string(), "https://example.com/hook".to_string());
        let opt = CreateHookOption {
            hook_type: HookType::Unknown,
            config,
            events: vec![],
            branch_filter: None,
            active: false,
            authorization_header: None,
        };
        let result = client.hooks().create_org_hook("org", opt).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hook type needed"));
    }

    #[tokio::test]
    async fn test_edit_org_hook_empty_org() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .hooks()
            .edit_org_hook("", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_org_hook_empty_org() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.hooks().delete_org_hook("", 1).await;
        assert!(result.is_err());
    }

    // ── repo hook happy paths ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_hooks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(vec![hook_json(1, "slack"), hook_json(2, "discord")]),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hooks, resp) = client
            .hooks()
            .list_repo_hooks("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(hooks.len(), 2);
        assert_eq!(hooks[0].id, 1);
        assert_eq!(hooks[1].id, 2);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_hook() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(hook_json(5, "telegram")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hook, resp) = client
            .hooks()
            .get_repo_hook("testowner", "testrepo", 5)
            .await
            .unwrap();
        assert_eq!(hook.id, 5);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_repo_hook() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks/3"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = EditHookOption {
            active: Some(false),
            ..Default::default()
        };
        let resp = client
            .hooks()
            .edit_repo_hook("testowner", "testrepo", 3, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_delete_repo_hook() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks/7"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .hooks()
            .delete_repo_hook("testowner", "testrepo", 7)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    // ── repo hook error paths ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_hooks_empty_owner() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .hooks()
            .list_repo_hooks("", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_hooks_empty_repo() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .hooks()
            .list_repo_hooks("owner", "", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_hook_empty_owner() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.hooks().get_repo_hook("", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_repo_hook_empty_owner() {
        let client = create_test_client(&MockServer::start().await);
        let opt = create_hook_option();
        let result = client.hooks().create_repo_hook("", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_repo_hook_empty_owner() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .hooks()
            .edit_repo_hook("", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_repo_hook_empty_owner() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.hooks().delete_repo_hook("", "repo", 1).await;
        assert!(result.is_err());
    }

    // ── user hook happy paths ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_hooks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/hooks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec![
                hook_json(10, "gitea"),
                hook_json(11, "slack"),
                hook_json(12, "discord"),
            ]))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hooks, resp) = client
            .hooks()
            .list_my_hooks(Default::default())
            .await
            .unwrap();
        assert_eq!(hooks.len(), 3);
        assert_eq!(hooks[2].id, 12);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_my_hook() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/hooks/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(hook_json(42, "dingtalk")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (hook, resp) = client.hooks().get_my_hook(42).await.unwrap();
        assert_eq!(hook.id, 42);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_my_hook() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/user/hooks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(hook_json(99, "msteams")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = create_hook_option();
        let (hook, resp) = client.hooks().create_my_hook(opt).await.unwrap();
        assert_eq!(hook.id, 99);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_edit_my_hook() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/hooks/8"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = EditHookOption {
            events: Some(vec!["issues".to_string()]),
            ..Default::default()
        };
        let resp = client.hooks().edit_my_hook(8, opt).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_delete_my_hook() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/hooks/55"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client.hooks().delete_my_hook(55).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    // ── user hook error paths ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_my_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.hooks().get_my_hook(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_my_hook_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/hooks/8"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = EditHookOption {
            events: Some(vec!["issues".to_string()]),
            ..Default::default()
        };
        let result = client.hooks().edit_my_hook(8, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_my_hook_unknown_type() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let mut config = std::collections::HashMap::new();
        config.insert("url".to_string(), "https://example.com/hook".to_string());
        let opt = CreateHookOption {
            hook_type: HookType::Unknown,
            config,
            events: vec![],
            branch_filter: None,
            active: false,
            authorization_header: None,
        };
        let result = client.hooks().create_my_hook(opt).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("hook type needed"));
    }

    // ── server error paths ────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_hooks_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/hooks"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .hooks()
            .list_org_hooks("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_org_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.hooks().get_org_hook("testorg", 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_org_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.hooks().delete_org_hook("testorg", 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_hooks_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .hooks()
            .list_repo_hooks("testowner", "testrepo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_repo_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .hooks()
            .delete_repo_hook("testowner", "testrepo", 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_my_hooks_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/hooks"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.hooks().list_my_hooks(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_my_hook_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/hooks/999"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.hooks().delete_my_hook(999).await;
        assert!(result.is_err());
    }
}
