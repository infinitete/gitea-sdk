// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::IssueLabelsOption;
use crate::pagination::QueryEncode;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_label.go ────────────────────────────────────────────
    // 5 methods

    /// `GetIssueLabels` get labels of one issue via issue id
    pub async fn get_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: crate::ListOptions,
    ) -> crate::Result<(Vec<crate::Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/labels?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `AddIssueLabels` add one or more labels to one issue
    pub async fn add_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueLabelsOption,
    ) -> crate::Result<(Vec<crate::Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/labels", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `ReplaceIssueLabels` replace old labels of issue with new labels
    pub async fn replace_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueLabelsOption,
    ) -> crate::Result<(Vec<crate::Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/labels", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteIssueLabel` delete one label of one issue by issue id and label id
    pub async fn delete_issue_label(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        label: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/labels/{label}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `ClearIssueLabels` delete all the labels of one issue
    pub async fn clear_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/labels", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_label.go ────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([label_json(1, "bug")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (labels, resp) = client
            .issues()
            .get_issue_labels("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_labels("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([label_json(1, "bug")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![1] };
        let (labels, resp) = client
            .issues()
            .add_issue_labels("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_add_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![1] };
        let result = client
            .issues()
            .add_issue_labels("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_replace_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([label_json(2, "feature")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![2] };
        let (labels, resp) = client
            .issues()
            .replace_issue_labels("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].name, "feature");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_replace_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![2] };
        let result = client
            .issues()
            .replace_issue_labels("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_label("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_label("owner", "repo", 1, 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clear_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .clear_issue_labels("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_clear_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().clear_issue_labels("owner", "repo", 1).await;
        assert!(result.is_err());
    }
}
