// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::pull::*;
use crate::pagination::QueryEncode;
use crate::types::ChangedFile;
use crate::version::VERSION_1_14_0;

impl<'a> super::PullsApi<'a> {
    /// CreateReviewRequests create review requests to a pull request.
    pub async fn create_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: PullReviewRequestOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_14_0)
            .await?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/requested_reviewers",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteReviewRequests delete review requests to a pull request.
    pub async fn delete_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: PullReviewRequestOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_14_0)
            .await?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/requested_reviewers",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListPullRequestCommits list commits for a pull request
    pub async fn list_commits(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListPullRequestCommitsOptions,
    ) -> crate::Result<(Vec<PullRequestCommit>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/commits?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListPullRequestFiles list changed files for a pull request
    pub async fn list_files(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListPullRequestFilesOptions,
    ) -> crate::Result<(Vec<ChangedFile>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/files?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DismissPullReview dismiss a review for a pull request.
    pub async fn dismiss_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
        opt: DismissPullReviewOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_14_0)
            .await?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews/{id}/dismissals",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// UnDismissPullReview cancel to dismiss a review for a pull request.
    pub async fn undismiss_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_14_0)
            .await?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews/{id}/undismissals",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

// ── Minimal commit type for ListPullRequestCommits ──────────────

#[derive(Debug, Clone, serde::Deserialize)]
pub struct PullRequestCommit {
    #[serde(rename = "sha")]
    pub id: String,
    #[serde(rename = "commit")]
    pub commit: CommitMeta,
    #[serde(default)]
    pub author: Option<crate::types::User>,
    #[serde(default)]
    pub committer: Option<crate::types::User>,
    #[serde(default)]
    pub parents: Vec<CommitMeta>,
}

/// CommitMeta contains meta information of a commit in Gitea format
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitMeta {
    #[serde(rename = "sha", default)]
    pub id: String,
    #[serde(rename = "author", default)]
    pub author: Option<CommitUser>,
    #[serde(rename = "committer", default)]
    pub committer: Option<CommitUser>,
    #[serde(default)]
    pub message: String,
}

/// CommitUser contains information of a user in a commit
#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitUser {
    pub name: String,
    pub email: String,
    #[serde(
        rename = "date",
        default,
        with = "crate::types::serde_helpers::nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub date: Option<time::OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::create_test_client;

    #[tokio::test]
    async fn test_create_review_requests() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/pulls/1/requested_reviewers",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::PullReviewRequestOptions {
            reviewers: vec!["reviewer1".to_string()],
            team_reviewers: vec![],
        };
        let resp = client
            .pulls()
            .create_review_requests("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_delete_review_requests() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/pulls/1/requested_reviewers",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::PullReviewRequestOptions {
            reviewers: vec!["reviewer1".to_string()],
            team_reviewers: vec![],
        };
        let resp = client
            .pulls()
            .delete_review_requests("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_create_review_requests_requires_gitea_1_14() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.13.0")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .create_review_requests("testowner", "testrepo", 1, Default::default())
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_delete_review_requests_requires_gitea_1_14() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.13.0")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .delete_review_requests("testowner", "testrepo", 1, Default::default())
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_list_pull_request_commits() {
        let server = MockServer::start().await;
        let user_json = serde_json::json!({"id": 1, "login": "alice", "login_name": "", "source_id": 0, "full_name": "", "email": "", "avatar_url": "", "html_url": "", "language": "", "is_admin": false, "restricted": false, "active": true, "prohibit_login": false, "location": "", "website": "", "description": "", "visibility": "public", "followers_count": 0, "following_count": 0, "starred_repos_count": 0});
        let body = serde_json::json!([
            {
                "sha": "abc123",
                "commit": {
                    "sha": "abc123",
                    "author": { "name": "alice", "email": "alice@example.com" },
                    "committer": { "name": "alice", "email": "alice@example.com" },
                    "message": "fix typo"
                },
                "author": &user_json,
                "parents": []
            },
            {
                "sha": "def456",
                "commit": {
                    "sha": "def456",
                    "author": { "name": "bob", "email": "bob@example.com" },
                    "committer": { "name": "bob", "email": "bob@example.com" },
                    "message": "add feature"
                },
                "author": null,
                "parents": [
                    { "sha": "abc123", "author": { "name": "alice", "email": "alice@example.com" }, "committer": { "name": "alice", "email": "alice@example.com" } }
                ]
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/commits"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (commits, resp) = client
            .pulls()
            .list_commits("testowner", "testrepo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(commits.len(), 2);
        assert_eq!(commits[0].id, "abc123");
        assert_eq!(commits[0].commit.message, "fix typo");
        assert_eq!(commits[1].id, "def456");
        assert_eq!(commits[1].parents.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_commits_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/commits"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "message": "internal error"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .list_commits("testowner", "testrepo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_pull_request_files() {
        let server = MockServer::start().await;
        let file1 = serde_json::json!({"filename": "readme.md", "previous_filename": "", "status": "modified", "additions": 1, "deletions": 1, "changes": 2, "html_url": "", "contents_url": "", "raw_url": ""});
        let file2 = serde_json::json!({"filename": "newfile.txt", "previous_filename": "", "status": "added", "additions": 5, "deletions": 0, "changes": 5, "html_url": "", "contents_url": "", "raw_url": ""});
        let body = serde_json::json!([&file1, &file2]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/files"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (files, resp) = client
            .pulls()
            .list_files("testowner", "testrepo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].filename, "readme.md");
        assert_eq!(files[0].status, "modified");
        assert_eq!(files[0].changes, 2);
        assert_eq!(files[1].filename, "newfile.txt");
        assert_eq!(files[1].status, "added");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_files_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/files"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "pull request not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .list_files("testowner", "testrepo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dismiss_review() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/pulls/1/reviews/10/dismissals",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = crate::options::pull::DismissPullReviewOptions {
            message: Some("dismissed due to stale".to_string()),
        };
        let resp = client
            .pulls()
            .dismiss_review("testowner", "testrepo", 1, 10, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_dismiss_review_requires_gitea_1_14() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.13.0")
            .build()
            .unwrap();

        let opt = crate::options::pull::DismissPullReviewOptions { message: None };
        let result = client
            .pulls()
            .dismiss_review("testowner", "testrepo", 1, 10, opt)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_undismiss_review() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/pulls/1/reviews/10/undismissals",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .pulls()
            .undismiss_review("testowner", "testrepo", 1, 10)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_undismiss_review_requires_gitea_1_14() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.13.0")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .undismiss_review("testowner", "testrepo", 1, 10)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }
}
