// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::*;
use crate::pagination::QueryEncode;
use crate::types::{StopWatch, TrackedTime};

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_stopwatch.go ────────────────────────────────────────
    // 4 methods (excluding deprecated GetMyStopwatches)

    /// ListMyStopwatches list all stopwatches with pagination
    pub async fn list_my_stopwatches(
        &self,
        opt: ListStopwatchesOptions,
    ) -> crate::Result<(Vec<StopWatch>, Response)> {
        let path = format!("/user/stopwatches?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeleteIssueStopwatch delete / cancel a specific stopwatch
    pub async fn delete_issue_stopwatch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/stopwatch/delete",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// StartIssueStopWatch starts a stopwatch for an existing issue
    pub async fn start_issue_stopwatch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/stopwatch/start",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// StopIssueStopWatch stops an existing stopwatch for an issue
    pub async fn stop_issue_stopwatch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/stopwatch/stop",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    // ── issue_tracked_time.go ─────────────────────────────────────
    // 7 methods (excluding deprecated GetMyTrackedTimes)

    /// ListRepoTrackedTimes list tracked times of a repository
    pub async fn list_repo_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        opt: ListTrackedTimesOptions,
    ) -> crate::Result<(Vec<TrackedTime>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/times?{}",
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

    /// ListMyTrackedTimes list tracked times of the current user with pagination and filtering
    pub async fn list_my_tracked_times(
        &self,
        opt: ListTrackedTimesOptions,
    ) -> crate::Result<(Vec<TrackedTime>, Response)> {
        let path = format!("/user/times?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// AddTime adds time to issue with the given index
    pub async fn add_time(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: AddTimeOption,
    ) -> crate::Result<(TrackedTime, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/times", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListIssueTrackedTimes list tracked times of a single issue
    pub async fn list_issue_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListTrackedTimesOptions,
    ) -> crate::Result<(Vec<TrackedTime>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/times?{}",
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

    /// ResetIssueTime reset tracked time of a single issue
    pub async fn reset_issue_time(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/times", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// DeleteTime delete a specific tracked time by id
    pub async fn delete_time(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        time_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/times/{time_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_stopwatch.go ────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_stopwatches_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/stopwatches"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([stopwatch_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (stopwatches, resp) = client
            .issues()
            .list_my_stopwatches(Default::default())
            .await
            .unwrap();
        assert_eq!(stopwatches.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_stopwatches_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/stopwatches"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_my_stopwatches(Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_stopwatch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/delete",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_stopwatch("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_stopwatch_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/delete",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_stopwatch("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_issue_stopwatch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/start",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .start_issue_stopwatch("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_start_issue_stopwatch_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/start",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .start_issue_stopwatch("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stop_issue_stopwatch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/stop",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .stop_issue_stopwatch("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_stop_issue_stopwatch_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/stop",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .stop_issue_stopwatch("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    // ── issue_tracked_time.go ─────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_tracked_times_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/times"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([tracked_time_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (times, resp) = client
            .issues()
            .list_repo_tracked_times("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_tracked_times_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_tracked_times("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_my_tracked_times_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/times"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([tracked_time_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (times, resp) = client
            .issues()
            .list_my_tracked_times(Default::default())
            .await
            .unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_tracked_times_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_my_tracked_times(Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_time_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tracked_time_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = AddTimeOption {
            time: 3600,
            created: None,
            user: String::new(),
        };
        let (tt, resp) = client
            .issues()
            .add_time("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(tt.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_add_time_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = AddTimeOption {
            time: 3600,
            created: None,
            user: String::new(),
        };
        let result = client.issues().add_time("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_time_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = AddTimeOption {
            time: 0,
            created: None,
            user: String::new(),
        };
        let result = client.issues().add_time("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_issue_tracked_times_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([tracked_time_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (times, resp) = client
            .issues()
            .list_issue_tracked_times("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_tracked_times_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_tracked_times("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_issue_time_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .reset_issue_time("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_reset_issue_time_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().reset_issue_time("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_time_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_time("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_time_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().delete_time("owner", "repo", 1, 1).await;
        assert!(result.is_err());
    }
}
