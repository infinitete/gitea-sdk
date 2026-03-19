// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::*;
use crate::pagination::QueryEncode;
use crate::types::{Issue, IssueMeta};

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue.go ──────────────────────────────────────────────────
    // 6 methods

    /// ListIssues returns all issues assigned the authenticated user
    pub async fn list_issues(&self, opt: ListIssueOption) -> crate::Result<(Vec<Issue>, Response)> {
        let path = format!("/repos/issues/search?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListRepoIssues returns all issues for a given repository
    pub async fn list_repo_issues(
        &self,
        owner: &str,
        repo: &str,
        opt: ListIssueOption,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssue returns a single issue for a given repository
    pub async fn get_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateIssue create a new issue for a given repository
    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateIssueOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditIssue modify an existing issue for a given repository
    pub async fn edit_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: EditIssueOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssue delete an issue from a repository
    pub async fn delete_issue(&self, owner: &str, repo: &str, id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── issue_ext.go ──────────────────────────────────────────────
    // 9 methods

    /// ListIssueBlocks lists issues that are blocked by the specified issue
    pub async fn list_issue_blocks(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueBlocksOptions,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/blocks?{}",
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

    /// CreateIssueBlocking blocks an issue with another issue
    pub async fn create_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/blocks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// RemoveIssueBlocking removes an issue block
    pub async fn remove_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/blocks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListIssueDependencies lists issues that block the specified issue (its dependencies)
    pub async fn list_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueDependenciesOptions,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/dependencies?{}",
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

    /// CreateIssueDependency creates a new issue dependency
    pub async fn create_issue_dependency(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/dependencies",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// RemoveIssueDependency removes an issue dependency
    pub async fn remove_issue_dependency(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/dependencies",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// LockIssue locks an issue
    pub async fn lock_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: LockIssueOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/lock", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
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
        Ok(resp)
    }

    /// UnlockIssue unlocks an issue
    pub async fn unlock_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/lock", escaped[0], escaped[1]);
        let (status, resp) = self
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
        Ok(resp)
    }

    /// UpdateIssueDeadline updates an issue's deadline
    pub async fn update_issue_deadline(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: EditDeadlineOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/deadline",
            escaped[0], escaped[1]
        );
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if let Ok(issue) = serde_json::from_slice::<Issue>(&data) {
            return Ok((issue, resp));
        }
        if serde_json::from_slice::<serde_json::Value>(&data)
            .ok()
            .and_then(|value| value.get("due_date").cloned())
            .is_some()
        {
            let (issue, _) = self.get_issue(owner, repo, index).await?;
            return Ok((issue, resp));
        }
        Err(crate::Error::Json(
            serde_json::from_slice::<Issue>(&data).unwrap_err(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue.go ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issues_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/issues/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(1, 1, "Issue 1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_issues(Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issues_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/issues/search"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().list_issues(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_issues_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(1, 1, "Issue 1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_repo_issues("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_issues_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_issues("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(1, 1, "Bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issue, resp) = client
            .issues()
            .get_issue("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue("testowner", "testrepo", 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(1, 1, "Bug fix")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueOption {
            title: "Bug fix".to_string(),
            body: "Fix the bug".to_string(),
            r#ref: String::new(),
            assignees: vec![],
            deadline: None,
            milestone: 0,
            labels: vec![],
            closed: false,
        };
        let (issue, resp) = client
            .issues()
            .create_issue("testowner", "testrepo", opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueOption {
            title: "Bug fix".to_string(),
            body: String::new(),
            r#ref: String::new(),
            assignees: vec![],
            deadline: None,
            milestone: 0,
            labels: vec![],
            closed: false,
        };
        let result = client
            .issues()
            .create_issue("testowner", "testrepo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateIssueOption {
            title: "  ".to_string(),
            body: String::new(),
            r#ref: String::new(),
            assignees: vec![],
            deadline: None,
            milestone: 0,
            labels: vec![],
            closed: false,
        };
        let result = client
            .issues()
            .create_issue("testowner", "testrepo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(1, 1, "Updated")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("Updated".to_string()),
            body: None,
            r#ref: None,
            assignees: vec![],
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        let (issue, resp) = client
            .issues()
            .edit_issue("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.title, "Updated");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("Updated".to_string()),
            body: None,
            r#ref: None,
            assignees: vec![],
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        let result = client
            .issues()
            .edit_issue("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("   ".to_string()),
            body: None,
            r#ref: None,
            assignees: vec![],
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        let result = client
            .issues()
            .edit_issue("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue("testowner", "testrepo", 1)
            .await;
        assert!(result.is_err());
    }

    // ── issue_ext.go ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_blocks_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(2, 2, "Blocker")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_issue_blocks("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_blocks_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_blocks("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_blocking_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(2, 2, "Blocker")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let (issue, resp) = client
            .issues()
            .create_issue_blocking("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 2);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_blocking_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let result = client
            .issues()
            .create_issue_blocking("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_issue_blocking_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(2, 2, "Blocker")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let (issue, resp) = client
            .issues()
            .remove_issue_blocking("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 2);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_remove_issue_blocking_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let result = client
            .issues()
            .remove_issue_blocking("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_issue_dependencies_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([issue_json(
                    3,
                    3,
                    "Dependency"
                )])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_issue_dependencies("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_dependencies_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_dependencies("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_dependency_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(3, 3, "Dep")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let (issue, resp) = client
            .issues()
            .create_issue_dependency("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 3);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_dependency_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let result = client
            .issues()
            .create_issue_dependency("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_issue_dependency_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(3, 3, "Dep")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let (issue, resp) = client
            .issues()
            .remove_issue_dependency("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_remove_issue_dependency_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let result = client
            .issues()
            .remove_issue_dependency("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lock_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = LockIssueOption {
            lock_reason: String::new(),
        };
        let resp = client
            .issues()
            .lock_issue("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_lock_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = LockIssueOption {
            lock_reason: String::new(),
        };
        let result = client.issues().lock_issue("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlock_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .unlock_issue("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unlock_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().unlock_issue("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_issue_deadline_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/deadline"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(1, 1, "Bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditDeadlineOption {
            deadline: Some(time::OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            )),
        };
        let (issue, resp) = client
            .issues()
            .update_issue_deadline("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_update_issue_deadline_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/deadline"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditDeadlineOption { deadline: None };
        let result = client
            .issues()
            .update_issue_deadline("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }
}
