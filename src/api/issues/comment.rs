// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::*;
use crate::pagination::QueryEncode;
use crate::types::Comment;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_comment.go ──────────────────────────────────────────
    // CRUD methods

    /// ListIssueComments list comments on an issue
    pub async fn list_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueCommentOptions,
    ) -> crate::Result<(Vec<Comment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/comments?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListRepoIssueComments list comments for a given repo
    pub async fn list_repo_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        opt: ListIssueCommentOptions,
    ) -> crate::Result<(Vec<Comment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssueComment get a comment for a given repo by id
    pub async fn get_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Comment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/comments/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateIssueComment create comment on an issue
    pub async fn create_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: CreateIssueCommentOption,
    ) -> crate::Result<(Comment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/comments",
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

    /// EditIssueComment edits an issue comment
    pub async fn edit_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        opt: EditIssueCommentOption,
    ) -> crate::Result<(Comment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssueComment deletes an issue comment
    pub async fn delete_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_comment.go ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_comments_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/comments"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([comment_json(1, "Nice")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comments, resp) = client
            .issues()
            .list_issue_comments("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_comments_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/comments"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_comments("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_issue_comments_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/comments"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([comment_json(1, "comment")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comments, resp) = client
            .issues()
            .list_repo_issue_comments("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_issue_comments_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/comments"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_issue_comments("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(comment_json(1, "hello")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comment, resp) = client
            .issues()
            .get_issue_comment("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(comment.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_comment("testowner", "testrepo", 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(comment_json(1, "Nice issue")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueCommentOption {
            body: "Nice issue".to_string(),
        };
        let (comment, resp) = client
            .issues()
            .create_issue_comment("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(comment.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1/comments"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueCommentOption {
            body: "body".to_string(),
        };
        let result = client
            .issues()
            .create_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_comment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateIssueCommentOption {
            body: String::new(),
        };
        let result = client
            .issues()
            .create_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(comment_json(1, "updated")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueCommentOption {
            body: "updated".to_string(),
        };
        let (comment, resp) = client
            .issues()
            .edit_issue_comment("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(comment.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueCommentOption {
            body: "updated".to_string(),
        };
        let result = client
            .issues()
            .edit_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = EditIssueCommentOption {
            body: String::new(),
        };
        let result = client
            .issues()
            .edit_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_comment("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_comment("testowner", "testrepo", 1)
            .await;
        assert!(result.is_err());
    }
}
