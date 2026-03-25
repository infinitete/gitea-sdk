// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Actions API endpoints for managing Gitea CI/CD workflows and runs.

use bytes::Bytes;

use crate::Client;
use crate::Response;
use crate::internal::request::json_header;
use crate::options::action::*;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::action::{
    ActionTaskResponse, ActionWorkflowJob, ActionWorkflowJobsResponse, ActionWorkflowRun,
    ActionWorkflowRunsResponse,
};

/// API methods for actions. Access via [`Client::actions()`](crate::Client::actions).
pub struct ActionsApi<'a> {
    client: &'a Client,
}

impl<'a> ActionsApi<'a> {
    /// Create a new `ActionsApi` view.
    #[must_use]
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// `ListRepoActionRuns` lists workflow runs for a repository
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

    /// `GetRepoActionRun` gets a single workflow run
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

    /// `DeleteRepoActionRun` deletes a workflow run
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

    /// `ListRepoActionRunJobs` lists jobs for a workflow run
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

    /// `ListRepoActionTasks` lists workflow tasks for a repository (older Gitea)
    pub async fn list_repo_action_tasks(
        &self,
        owner: &str,
        repo: &str,
        opt: ListOptions,
    ) -> crate::Result<(ActionTaskResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/tasks?{}",
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

    /// `ListRepoActionJobs` lists all jobs for a repository
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

    /// `GetRepoActionJob` gets a single job
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

    /// `GetRepoActionJobLogs` gets the logs for a specific job
    pub async fn get_repo_action_job_logs(
        &self,
        owner: &str,
        repo: &str,
        job_id: i64,
    ) -> crate::Result<(Bytes, Response)> {
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
    use wiremock::matchers::{method, path, path_regex};
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
    async fn test_list_repo_action_tasks() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total_count": 1,
                "workflow_runs": [{
                    "id": 1,
                    "name": "CI",
                    "head_branch": "main",
                    "head_sha": "abc123",
                    "run_number": 1,
                    "event": "push",
                    "display_title": "CI",
                    "status": "completed",
                    "workflow_id": "ci.yml",
                    "url": "",
                    "created_at": "2024-01-15T10:00:00Z",
                    "updated_at": "2024-01-15T10:05:00Z",
                    "run_started_at": "2024-01-15T10:00:00Z"
                }]
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (resp, response) = client
            .actions()
            .list_repo_action_tasks("testowner", "testrepo", Default::default())
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

    #[tokio::test]
    async fn test_list_repo_action_runs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/runs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .list_repo_action_runs("testowner", "testrepo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_action_run_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/runs/\d+",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .get_repo_action_run("testowner", "testrepo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_repo_action_run() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/runs/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .actions()
            .delete_repo_action_run("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_repo_action_run_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/runs/\d+",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .delete_repo_action_run("testowner", "testrepo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_action_run_jobs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/runs/\d+/jobs",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total_count": 1,
                "jobs": [{
                    "id": 10,
                    "run_id": 1,
                    "run_url": "",
                    "run_attempt": 1,
                    "name": "build",
                    "head_branch": "main",
                    "head_sha": "abc123",
                    "status": "completed",
                    "conclusion": "success",
                    "url": "",
                    "html_url": "",
                    "created_at": "2024-01-15T10:00:00Z",
                    "started_at": "2024-01-15T10:00:00Z",
                    "completed_at": "2024-01-15T10:05:00Z",
                    "runner_id": 1,
                    "runner_name": "linux",
                    "labels": [],
                    "steps": []
                }]
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (resp, response) = client
            .actions()
            .list_repo_action_run_jobs("testowner", "testrepo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(resp.total_count, 1);
        assert_eq!(resp.jobs.len(), 1);
        assert_eq!(resp.jobs[0].id, 10);
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_action_run_jobs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/runs/\d+/jobs",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .list_repo_action_run_jobs("testowner", "testrepo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_action_tasks_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/tasks"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .list_repo_action_tasks("testowner", "testrepo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_action_jobs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/jobs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "total_count": 2,
                "jobs": [
                    {
                        "id": 20,
                        "run_id": 1,
                        "run_url": "",
                        "run_attempt": 1,
                        "name": "test",
                        "head_branch": "main",
                        "head_sha": "abc123",
                        "status": "completed",
                        "conclusion": "success",
                        "url": "",
                        "html_url": "",
                        "created_at": "2024-01-15T10:00:00Z",
                        "started_at": "2024-01-15T10:00:00Z",
                        "completed_at": "2024-01-15T10:05:00Z",
                        "runner_id": 1,
                        "runner_name": "linux",
                        "labels": [],
                        "steps": []
                    },
                    {
                        "id": 21,
                        "run_id": 2,
                        "run_url": "",
                        "run_attempt": 1,
                        "name": "build",
                        "head_branch": "main",
                        "head_sha": "def456",
                        "status": "running",
                        "conclusion": "",
                        "url": "",
                        "html_url": "",
                        "created_at": "2024-01-15T11:00:00Z",
                        "started_at": "2024-01-15T11:00:00Z",
                        "completed_at": "2024-01-15T11:00:00Z",
                        "runner_id": 2,
                        "runner_name": "windows",
                        "labels": [],
                        "steps": []
                    }
                ]
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (resp, response) = client
            .actions()
            .list_repo_action_jobs("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(resp.total_count, 2);
        assert_eq!(resp.jobs.len(), 2);
        assert_eq!(response.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_action_jobs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/actions/jobs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .list_repo_action_jobs("testowner", "testrepo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_action_job() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/jobs/\d+",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 10,
                "run_id": 1,
                "run_url": "",
                "run_attempt": 1,
                "name": "build",
                "head_branch": "main",
                "head_sha": "abc123",
                "status": "completed",
                "conclusion": "success",
                "url": "",
                "html_url": "",
                "created_at": "2024-01-15T10:00:00Z",
                "started_at": "2024-01-15T10:00:00Z",
                "completed_at": "2024-01-15T10:05:00Z",
                "runner_id": 1,
                "runner_name": "linux",
                "labels": ["ubuntu-latest"],
                "steps": [{
                    "name": "checkout",
                    "number": 1,
                    "status": "completed",
                    "conclusion": "success",
                    "started_at": "2024-01-15T10:00:00Z",
                    "completed_at": "2024-01-15T10:01:00Z"
                }]
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (job, resp) = client
            .actions()
            .get_repo_action_job("testowner", "testrepo", 10)
            .await
            .unwrap();
        assert_eq!(job.id, 10);
        assert_eq!(job.name, "build");
        assert_eq!(job.labels.len(), 1);
        assert_eq!(job.steps.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_action_job_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/jobs/\d+",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .get_repo_action_job("testowner", "testrepo", 10)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_action_job_logs() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/jobs/\d+/logs",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_string("line 1\nline 2\nline 3"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (body, resp) = client
            .actions()
            .get_repo_action_job_logs("testowner", "testrepo", 10)
            .await
            .unwrap();
        assert_eq!(String::from_utf8_lossy(&body), "line 1\nline 2\nline 3");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_action_job_logs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/testowner/testrepo/actions/jobs/\d+/logs",
            ))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .actions()
            .get_repo_action_job_logs("testowner", "testrepo", 10)
            .await;
        assert!(result.is_err());
    }
}
