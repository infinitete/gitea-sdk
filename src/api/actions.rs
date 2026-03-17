// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::action::*;
use crate::pagination::QueryEncode;
use crate::types::action::{
    ActionWorkflowJob, ActionWorkflowJobsResponse, ActionWorkflowRun, ActionWorkflowRunsResponse,
};

pub struct ActionsApi<'a> {
    client: &'a Client,
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

impl<'a> ActionsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// ListRepoActionRuns lists workflow runs for a repository
    pub async fn list_repo_action_runs(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoActionRunsOptions,
    ) -> crate::Result<(ActionWorkflowRunsResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/runs?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetRepoActionRun gets a single workflow run
    pub async fn get_repo_action_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> crate::Result<(ActionWorkflowRun, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/actions/runs/{run_id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// DeleteRepoActionRun deletes a workflow run
    pub async fn delete_repo_action_run(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/actions/runs/{run_id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ListRepoActionRunJobs lists jobs for a workflow run
    pub async fn list_repo_action_run_jobs(
        &self,
        owner: &str,
        repo: &str,
        run_id: i64,
        opt: ListRepoActionJobsOptions,
    ) -> crate::Result<(ActionWorkflowJobsResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/runs/{run_id}/jobs?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ListRepoActionJobs lists all jobs for a repository
    pub async fn list_repo_action_jobs(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoActionJobsOptions,
    ) -> crate::Result<(ActionWorkflowJobsResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/jobs?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetRepoActionJob gets a single job
    pub async fn get_repo_action_job(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> crate::Result<(ActionWorkflowJob, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/actions/jobs/{job_id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetRepoActionJobLogs gets the logs for a specific job
    pub async fn get_repo_action_job_logs(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/jobs/{job_id}/logs",
            escaped[0], escaped[1]
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
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

    #[tokio::test]
    async fn test_list_repo_action_runs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/runs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total_count": 1,
                "workflow_runs": [{
                    "id": 1,
                    "display_title": "CI",
                    "event": "push",
                    "head_branch": "main",
                    "head_sha": "abc123",
                    "path": ".gitea/workflows/ci.yml",
                    "run_attempt": 1,
                    "run_number": 1,
                    "status": "completed",
                    "conclusion": "success",
                    "url": "",
                    "html_url": "",
                    "started_at": "2024-01-15T10:00:00Z",
                    "completed_at": "2024-01-15T10:05:00Z",
                    "repository_id": 1
                }]
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (resp, response) = client
            .actions()
            .list_repo_action_runs("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.workflow_runs.len(), 1);
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_action_run() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/runs/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "display_title": "CI",
                "event": "push",
                "head_branch": "main",
                "head_sha": "abc123",
                "path": ".gitea/workflows/ci.yml",
                "run_attempt": 1,
                "run_number": 1,
                "status": "completed",
                "conclusion": "success",
                "url": "",
                "html_url": "",
                "started_at": "2024-01-15T10:00:00Z",
                "completed_at": "2024-01-15T10:05:00Z",
                "repository_id": 1
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (run, resp) = client
            .actions()
            .get_repo_action_run("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(run.id, 1);
        assert_eq!(resp.status, 200);
    }
}
