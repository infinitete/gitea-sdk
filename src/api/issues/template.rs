// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::types::IssueTemplate;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_template.go ─────────────────────────────────────────
    // 1 method

    /// GetIssueTemplates lists all issue templates of the repository
    pub async fn get_issue_templates(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<IssueTemplate>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issue_templates", escaped[0], escaped[1]);
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

    // ── issue_template.go ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_issue_templates_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issue_templates"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_template_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (templates, resp) = client
            .issues()
            .get_issue_templates("owner", "repo")
            .await
            .unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].name, "Bug Report");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_templates_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issue_templates"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().get_issue_templates("owner", "repo").await;
        assert!(result.is_err());
    }
}
