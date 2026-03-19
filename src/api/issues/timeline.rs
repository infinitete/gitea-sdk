// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::options::issue::ListIssueCommentOptions;
use crate::pagination::QueryEncode;
use crate::types::TimelineComment;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_timeline.go ─────────────────────────────────────────
    // 1 method

    /// ListIssueTimeline list timeline on an issue
    pub async fn list_issue_timeline(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueCommentOptions,
    ) -> crate::Result<(Vec<TimelineComment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/timeline?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_timeline.go ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_timeline_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/timeline"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([timeline_comment_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comments, resp) = client
            .issues()
            .list_issue_timeline("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_timeline_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/timeline"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_timeline("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }
}
