// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::pull::*;
use crate::pagination::QueryEncode;
use crate::types::{ChangedFile, PullRequest, PullReview, PullReviewComment};

pub struct PullsApi<'a> {
    client: &'a Client,
}

fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

impl<'a> PullsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    // ── pull.go ────────────────────────────────────────────────────

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
        // TODO: version gate for fixPullHeadSha (Gitea >= 1.14)
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
        // TODO: version gate for fixPullHeadSha (Gitea >= 1.14)
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

    /// MergePullRequest merge a PR to repository by PR id.
    /// Returns `(merged: bool, Response)`. `merged` is true when status is 200.
    /// TODO: version gate for MergeStyleSquash (Gitea >= 1.11.5)
    pub async fn merge(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: MergePullRequestOption,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/pulls/{index}/merge", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if status / 100 != 2 && status != 405 {
            // Non-success, non-merge-conflict status → read body for error
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("merge failed with status {status}"),
            });
        }
        Ok((status == 200, response))
    }

    /// IsPullRequestMerged test if one PR is merged to one repository.
    /// Returns `true` when status is 204.
    pub async fn is_merged(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/pulls/{index}/merge", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        Ok((status == 204, response))
    }

    /// GetPullRequestPatch gets the git patchset of a PR as raw bytes.
    /// TODO: version gate for Gitea >= 1.13 (assumes >= 1.13)
    pub async fn patch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/pulls/{index}.patch", escaped[0], escaped[1]);
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetPullRequestDiff gets the diff of a PR as raw bytes.
    /// For Gitea >= 1.16, you must set includeBinary to get an applicable diff.
    /// TODO: version gate for Gitea >= 1.13 (assumes >= 1.13)
    pub async fn diff(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: PullRequestDiffOptions,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let qs = opt.query_encode();
        let path = format!(
            "/repos/{}/{}/pulls/{index}.diff?{qs}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
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

    // ── pull_review.go ─────────────────────────────────────────────

    /// ListPullReviews lists all reviews of a pull request.
    /// TODO: version gate for Gitea >= 1.12
    pub async fn list_reviews(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListPullReviewsOptions,
    ) -> crate::Result<(Vec<PullReview>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews?{}",
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

    /// GetPullReview gets a specific review of a pull request.
    /// TODO: version gate for Gitea >= 1.12
    pub async fn get_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<(PullReview, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews/{id}",
            escaped[0], escaped[1]
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

    /// ListPullReviewComments lists all comments of a pull request review.
    /// TODO: version gate for Gitea >= 1.12
    pub async fn list_review_comments(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<(Vec<PullReviewComment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews/{id}/comments",
            escaped[0], escaped[1]
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

    /// DeletePullReview delete a specific review from a pull request.
    /// TODO: version gate for Gitea >= 1.12
    pub async fn delete_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews/{id}",
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

    /// CreatePullReview create a review to a pull request.
    /// TODO: version gate for Gitea >= 1.12
    pub async fn create_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: CreatePullReviewOptions,
    ) -> crate::Result<(PullReview, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/pulls/{index}/reviews", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// SubmitPullReview submit a pending review to a pull request.
    /// TODO: version gate for Gitea >= 1.12
    pub async fn submit_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
        opt: SubmitPullReviewOptions,
    ) -> crate::Result<(PullReview, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/pulls/{index}/reviews/{id}",
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

    /// CreateReviewRequests create review requests to a pull request.
    /// TODO: version gate for Gitea >= 1.14
    pub async fn create_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: PullReviewRequestOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
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
    /// TODO: version gate for Gitea >= 1.14
    pub async fn delete_review_requests(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: PullReviewRequestOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
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

    /// DismissPullReview dismiss a review for a pull request.
    /// TODO: version gate for Gitea >= 1.14
    pub async fn dismiss_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
        opt: DismissPullReviewOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
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
    /// TODO: version gate for Gitea >= 1.14
    pub async fn undismiss_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
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
// NOTE: A full Commit type should live in its own module when repos are
// implemented. This is a minimal placeholder matching Go SDK's Commit struct
// fields commonly returned by the PR commits endpoint.

/// PullRequestCommit represents a commit in a pull request.
/// This is a minimal version; a full Commit type will be added with the repos module.
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
    #[serde(rename = "sha")]
    pub id: String,
    #[serde(rename = "author")]
    pub author: CommitUser,
    #[serde(rename = "committer")]
    pub committer: CommitUser,
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

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn pr_json(id: i64, number: i64, title: &str, state: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "url": "",
            "number": number,
            "user": {
                "id": 1,
                "login": "testuser",
                "login_name": "",
                "source_id": 0,
                "full_name": "",
                "email": "",
                "avatar_url": "",
                "html_url": "",
                "language": "",
                "is_admin": false,
                "restricted": false,
                "active": true,
                "prohibit_login": false,
                "location": "",
                "website": "",
                "description": "",
                "visibility": "public",
                "followers_count": 0,
                "following_count": 0,
                "starred_repos_count": 0
            },
            "title": title,
            "body": "",
            "labels": [],
            "milestone": null,
            "assignee": null,
            "assignees": [],
            "requested_reviewers": [],
            "requested_reviewers_teams": [],
            "state": state,
            "draft": false,
            "is_locked": false,
            "comments": 0,
            "html_url": "",
            "diff_url": "",
            "patch_url": "",
            "mergeable": false,
            "merged": false,
            "merged_at": null,
            "merge_commit_sha": null,
            "merged_by": null,
            "allow_maintainer_edit": false,
            "base": {
                "label": "main",
                "ref": "main",
                "sha": "abc123",
                "repo_id": 1,
                "repo": {
                    "id": 1,
                    "name": "testrepo",
                    "full_name": "testowner/testrepo",
                    "owner": {
                        "id": 1,
                        "login": "testowner"
                    }
                }
            },
            "head": {
                "label": "feature",
                "ref": "feature",
                "sha": "def456",
                "repo_id": 2,
                "repo": null
            },
            "merge_base": "",
            "due_date": null,
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T10:00:00Z",
            "closed_at": null,
            "pin_order": 0
        })
    }

    fn review_json(id: i64, state: &str, body: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "user": {
                "id": 1,
                "login": "reviewer",
                "login_name": "",
                "source_id": 0,
                "full_name": "",
                "email": "",
                "avatar_url": "",
                "html_url": "",
                "language": "",
                "is_admin": false,
                "restricted": false,
                "active": true,
                "prohibit_login": false,
                "location": "",
                "website": "",
                "description": "",
                "visibility": "public",
                "followers_count": 0,
                "following_count": 0,
                "starred_repos_count": 0
            },
            "team": null,
            "state": state,
            "body": body,
            "commit_id": "abc123",
            "stale": false,
            "official": false,
            "dismissed": false,
            "comments_count": 0,
            "submitted_at": "2024-01-15T10:00:00Z",
            "html_url": "",
            "pull_request_url": ""
        })
    }

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
        let opt = CreatePullRequestOption {
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
    async fn test_merge_pull_request() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/merge"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (merged, resp) = client
            .pulls()
            .merge(
                "testowner",
                "testrepo",
                1,
                MergePullRequestOption::default(),
            )
            .await
            .unwrap();
        assert!(merged);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_is_merged() {
        let server = MockServer::start().await;

        // Merged PR → 204
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/merge"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        // Not merged PR → 404
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/2/merge"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "pull request not merged"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);

        let (merged, _) = client
            .pulls()
            .is_merged("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert!(merged);

        let (not_merged, _) = client
            .pulls()
            .is_merged("testowner", "testrepo", 2)
            .await
            .unwrap();
        assert!(!not_merged);
    }

    #[tokio::test]
    async fn test_list_reviews() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            review_json(1, "APPROVED", "LGTM"),
            review_json(2, "COMMENT", "Looks good"),
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (reviews, resp) = client
            .pulls()
            .list_reviews("testowner", "testrepo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(reviews.len(), 2);
        assert_eq!(reviews[0].body, "LGTM");
        assert_eq!(reviews[1].body, "Looks good");
        assert_eq!(resp.status, 200);
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
}
