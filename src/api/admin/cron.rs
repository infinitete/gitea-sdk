// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::admin::ListCronTasksOptions;
use crate::pagination::QueryEncode;
use crate::types::CronTask;

use super::AdminApi;

impl<'a> AdminApi<'a> {
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
}

#[cfg(test)]
mod tests {
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, cron_task_json};

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
}
