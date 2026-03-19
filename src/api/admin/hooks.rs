// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::admin::{CreateHookOption, EditHookOption, ListAdminHooksOptions};
use crate::pagination::QueryEncode;
use crate::types::Hook;

use super::AdminApi;

impl<'a> AdminApi<'a> {
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
}

#[cfg(test)]
mod tests {
    use crate::options::admin::{CreateHookOption, EditHookOption};
    #[allow(unused_imports)]
    use crate::{HookType, TrustModel};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, hook_json};

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
}
