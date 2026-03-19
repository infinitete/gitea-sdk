// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::pull::*;
use crate::pagination::QueryEncode;
use crate::types::PullRequest;
use crate::version::VERSION_1_12_0;

impl<'a> super::PullsApi<'a> {
    /// ListRepoPullRequests list PRs of one repository
    pub async fn list(
        &self,
        owner: &str,
        repo: &str,
        opt: ListPullRequestsOptions,
    ) -> crate::Result<(Vec<PullRequest>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetPullRequest get information of one PR
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(PullRequest, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/pulls/{index}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreatePullRequest create pull request with options
    pub async fn create(
        &self,
        owner: &str,
        repo: &str,
        opt: CreatePullRequestOption,
    ) -> crate::Result<(PullRequest, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/pulls", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditPullRequest modify pull request with PR id and options
    pub async fn edit(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: EditPullRequestOption,
    ) -> crate::Result<(PullRequest, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        if opt.base.is_some() {
            self.client()
                .check_server_version_ge(&VERSION_1_12_0)
                .await?;
        }
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/pulls/{index}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, pr_json};

    #[tokio::test]
    async fn test_list_pull_requests() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            pr_json(1, 1, "Fix bug", "open"),
            pr_json(2, 2, "Add feature", "open"),
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (prs, resp) = client
            .pulls()
            .list("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(prs.len(), 2);
        assert_eq!(prs[0].title, "Fix bug");
        assert_eq!(prs[1].title, "Add feature");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_pull_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls"))
            .respond_with(ResponseTemplate::new(201).set_body_json(pr_json(3, 3, "New PR", "open")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::CreatePullRequestOption {
            head: "feature".to_string(),
            base: "main".to_string(),
            title: "New PR".to_string(),
            body: Some("Description".to_string()),
            assignee: None,
            assignees: vec![],
            reviewers: vec![],
            team_reviewers: vec![],
            milestone: 0,
            labels: vec![],
            deadline: None,
        };
        let (pr, resp) = client
            .pulls()
            .create("testowner", "testrepo", opt)
            .await
            .unwrap();
        assert_eq!(pr.title, "New PR");
        assert_eq!(pr.index, 3);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_error_case() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/999"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({"message": "Pull request not found"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.pulls().get("testowner", "testrepo", 999).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Pull request not found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_get_pull_request() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(pr_json(1, 1, "Fix bug", "open")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (pr, resp) = client
            .pulls()
            .get("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(pr.title, "Fix bug");
        assert_eq!(pr.index, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_pull_request() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(pr_json(
                1,
                1,
                "Updated title",
                "open",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::EditPullRequestOption {
            title: Some("Updated title".to_string()),
            body: Some("New description".to_string()),
            ..Default::default()
        };
        let (pr, resp) = client
            .pulls()
            .edit("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(pr.title, "Updated title");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_pull_request_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);

        let opt = crate::options::pull::EditPullRequestOption {
            title: Some("   ".to_string()),
            ..Default::default()
        };
        let result = client.pulls().edit("testowner", "testrepo", 1, opt).await;
        assert!(matches!(result, Err(crate::Error::Validation(_))));
    }

    #[tokio::test]
    async fn test_edit_base_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .edit(
                "testowner",
                "testrepo",
                1,
                crate::options::pull::EditPullRequestOption {
                    base: Some("stable".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_list_pull_requests_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "message": "internal error"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .list("testowner", "testrepo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_pull_request_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "message": "validation failed"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::CreatePullRequestOption {
            head: "feature".to_string(),
            base: "main".to_string(),
            title: "New PR".to_string(),
            body: None,
            assignee: None,
            assignees: vec![],
            reviewers: vec![],
            team_reviewers: vec![],
            milestone: 0,
            labels: vec![],
            deadline: None,
        };
        let result = client.pulls().create("testowner", "testrepo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_pull_request_error() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "pull request not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::EditPullRequestOption {
            title: Some("Updated".to_string()),
            ..Default::default()
        };
        let result = client.pulls().edit("testowner", "testrepo", 1, opt).await;
        assert!(result.is_err());
    }
}
