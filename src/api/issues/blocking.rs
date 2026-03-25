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
    // ── issue_ext.go ──────────────────────────────────────────────
    // blocks & dependencies

    /// `ListIssueBlocks` lists issues that are blocked by the specified issue
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

    /// `CreateIssueBlocking` blocks an issue with another issue
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

    /// `RemoveIssueBlocking` removes an issue block
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

    /// `ListIssueDependencies` lists issues that block the specified issue (its dependencies)
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

    /// `CreateIssueDependency` creates a new issue dependency
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

    /// `RemoveIssueDependency` removes an issue dependency
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
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

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
}
