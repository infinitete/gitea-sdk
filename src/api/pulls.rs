// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::pull::*;
use crate::pagination::QueryEncode;
use crate::types::{ChangedFile, PullRequest, PullReview, PullReviewComment};
use crate::version::{VERSION_1_11_5, VERSION_1_12_0, VERSION_1_13_0, VERSION_1_14_0};

/// API methods for pull request resources.
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
    /// Create a new `PullsApi` view.
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

    /// MergePullRequest merge a PR to repository by PR id.
    /// Returns `(merged: bool, Response)`. `merged` is true when status is 200.
    pub async fn merge(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: MergePullRequestOption,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        if matches!(opt.style, Some(crate::types::enums::MergeStyle::Squash)) {
            self.client()
                .check_server_version_ge(&VERSION_1_11_5)
                .await?;
        }
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
    pub async fn patch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_13_0)
            .await?;
        let path = format!("/repos/{}/{}/pulls/{index}.patch", escaped[0], escaped[1]);
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetPullRequestDiff gets the diff of a PR as raw bytes.
    /// For Gitea >= 1.16, you must set includeBinary to get an applicable diff.
    pub async fn diff(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: PullRequestDiffOptions,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_13_0)
            .await?;
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
    pub async fn list_reviews(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListPullReviewsOptions,
    ) -> crate::Result<(Vec<PullReview>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
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
    pub async fn get_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<(PullReview, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
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
    pub async fn list_review_comments(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<(Vec<PullReviewComment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
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
    pub async fn delete_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
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
    pub async fn create_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: CreatePullReviewOptions,
    ) -> crate::Result<(PullReview, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
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
    pub async fn submit_review(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        id: i64,
        opt: SubmitPullReviewOptions,
    ) -> crate::Result<(PullReview, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
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
// NOTE: A full Commit type should live in its own module when repos are
// implemented. This is a minimal placeholder matching Go SDK's Commit struct
// fields commonly returned by the PR commits endpoint.

/// PullRequestCommit represents a commit in a pull request.
/// This is a minimal version; a full Commit type will be added with the repos module.
#[derive(Debug, Clone, serde::Deserialize)]
/// Pull Request Commit payload type.
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
/// Commit Meta payload type.
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
/// Commit User payload type.
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
    async fn test_get_pull_request_patch() {
        let server = MockServer::start().await;
        let patch = "diff --git a/file.txt b/file.txt\n+hello\n";

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1.patch"))
            .respond_with(ResponseTemplate::new(200).set_body_string(patch))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (body, resp) = client
            .pulls()
            .patch("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(String::from_utf8(body).unwrap(), patch);
        assert_eq!(resp.status, 200);
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
    async fn test_patch_requires_gitea_1_13() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.12.3")
            .build()
            .unwrap();

        let result = client.pulls().patch("testowner", "testrepo", 1).await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_list_reviews_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .list_reviews("testowner", "testrepo", 1, Default::default())
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
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
    async fn test_merge_squash_requires_gitea_1_11_5() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.0")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .merge(
                "testowner",
                "testrepo",
                1,
                MergePullRequestOption {
                    style: Some(crate::types::enums::MergeStyle::Squash),
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
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
                EditPullRequestOption {
                    base: Some("stable".to_string()),
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
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
    async fn test_get_pull_request_diff() {
        let server = MockServer::start().await;
        let diff_body = "diff --git a/readme.md b/readme.md\nindex abc..def 100644\n--- a/readme.md\n+++ b/readme.md\n@@ -1 +1 @@\n-hello\n+world\n";

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1.diff"))
            .respond_with(ResponseTemplate::new(200).set_body_string(diff_body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (body, resp) = client
            .pulls()
            .diff(
                "testowner",
                "testrepo",
                1,
                PullRequestDiffOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(String::from_utf8(body).unwrap(), diff_body);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_diff_requires_gitea_1_13() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.12.3")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .diff(
                "testowner",
                "testrepo",
                1,
                PullRequestDiffOptions::default(),
            )
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_list_pull_request_commits() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "sha": "abc123",
                "commit": {
                    "sha": "abc123",
                    "author": { "name": "alice", "email": "alice@example.com" },
                    "committer": { "name": "alice", "email": "alice@example.com" },
                    "message": "fix typo"
                },
                "author": { "id": 1, "login": "alice", "login_name": "", "source_id": 0, "full_name": "", "email": "", "avatar_url": "", "html_url": "", "language": "", "is_admin": false, "restricted": false, "active": true, "prohibit_login": false, "location": "", "website": "", "description": "", "visibility": "public", "followers_count": 0, "following_count": 0, "starred_repos_count": 0 },
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
        let body = serde_json::json!([
            {
                "filename": "readme.md",
                "previous_filename": "",
                "status": "modified",
                "additions": 1,
                "deletions": 1,
                "changes": 2,
                "html_url": "",
                "contents_url": "",
                "raw_url": ""
            },
            {
                "filename": "newfile.txt",
                "previous_filename": "",
                "status": "added",
                "additions": 5,
                "deletions": 0,
                "changes": 5,
                "html_url": "",
                "contents_url": "",
                "raw_url": ""
            }
        ]);

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
        let opt = EditPullRequestOption {
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

        let opt = EditPullRequestOption {
            title: Some("   ".to_string()),
            ..Default::default()
        };
        let result = client.pulls().edit("testowner", "testrepo", 1, opt).await;
        assert!(matches!(result, Err(crate::Error::Validation(_))));
    }

    #[tokio::test]
    async fn test_get_pull_review() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews/10"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(review_json(10, "APPROVED", "LGTM")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (review, resp) = client
            .pulls()
            .get_review("testowner", "testrepo", 1, 10)
            .await
            .unwrap();
        assert_eq!(review.id, 10);
        assert_eq!(review.body, "LGTM");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_review_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .get_review("testowner", "testrepo", 1, 10)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_list_review_comments() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "id": 100,
                "body": "fix this line",
                "pull_request_review_id": 10,
                "path": "src/main.rs",
                "commit_id": "abc123",
                "original_commit_id": "abc123",
                "diff_hunk": "@@ -1,3 +1,4 @@",
                "position": 5,
                "original_position": 5,
                "html_url": "",
                "pull_request_url": "",
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z"
            },
            {
                "id": 101,
                "body": "nit: spacing",
                "pull_request_review_id": 10,
                "path": "src/lib.rs",
                "commit_id": "abc123",
                "original_commit_id": "abc123",
                "diff_hunk": "@@ -10,3 +10,3 @@",
                "position": 12,
                "original_position": 12,
                "html_url": "",
                "pull_request_url": "",
                "created_at": "2024-01-15T10:00:00Z",
                "updated_at": "2024-01-15T10:00:00Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/pulls/1/reviews/10/comments",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (comments, resp) = client
            .pulls()
            .list_review_comments("testowner", "testrepo", 1, 10)
            .await
            .unwrap();
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].body, "fix this line");
        assert_eq!(comments[0].path, "src/main.rs");
        assert_eq!(comments[1].body, "nit: spacing");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_review_comments_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .list_review_comments("testowner", "testrepo", 1, 10)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_delete_review() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews/10"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .pulls()
            .delete_review("testowner", "testrepo", 1, 10)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_review_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let result = client
            .pulls()
            .delete_review("testowner", "testrepo", 1, 10)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_create_review() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews"))
            .respond_with(ResponseTemplate::new(200).set_body_json(review_json(
                20,
                "APPROVED",
                "Looks great",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreatePullReviewOptions {
            state: Some(crate::types::enums::ReviewStateType::Approved),
            body: None,
            commit_id: None,
            comments: vec![],
        };
        let (review, resp) = client
            .pulls()
            .create_review("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(review.id, 20);
        assert_eq!(review.body, "Looks great");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_review_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);

        let opt = CreatePullReviewOptions {
            state: None,
            body: Some("   ".to_string()),
            commit_id: None,
            comments: vec![],
        };
        let result = client
            .pulls()
            .create_review("testowner", "testrepo", 1, opt)
            .await;
        assert!(matches!(result, Err(crate::Error::Validation(_))));
    }

    #[tokio::test]
    async fn test_create_review_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let opt = CreatePullReviewOptions {
            state: Some(crate::types::enums::ReviewStateType::Approved),
            body: None,
            commit_id: None,
            comments: vec![],
        };
        let result = client
            .pulls()
            .create_review("testowner", "testrepo", 1, opt)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

    #[tokio::test]
    async fn test_submit_review() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews/10"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(review_json(10, "APPROVED", "Ship it")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = SubmitPullReviewOptions {
            state: Some(crate::types::enums::ReviewStateType::Approved),
            body: Some("Ship it".to_string()),
        };
        let (review, resp) = client
            .pulls()
            .submit_review("testowner", "testrepo", 1, 10, opt)
            .await
            .unwrap();
        assert_eq!(review.id, 10);
        assert_eq!(review.body, "Ship it");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_submit_review_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);

        let opt = SubmitPullReviewOptions {
            state: None,
            body: Some("   ".to_string()),
        };
        let result = client
            .pulls()
            .submit_review("testowner", "testrepo", 1, 10, opt)
            .await;
        assert!(matches!(result, Err(crate::Error::Validation(_))));
    }

    #[tokio::test]
    async fn test_submit_review_requires_gitea_1_12() {
        let server = MockServer::start().await;
        let client = Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("1.11.5")
            .build()
            .unwrap();

        let opt = SubmitPullReviewOptions {
            state: Some(crate::types::enums::ReviewStateType::Approved),
            body: None,
        };
        let result = client
            .pulls()
            .submit_review("testowner", "testrepo", 1, 10, opt)
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
    }

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
        let opt = PullReviewRequestOptions {
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
        let opt = PullReviewRequestOptions {
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
        let opt = DismissPullReviewOptions {
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

        let opt = DismissPullReviewOptions { message: None };
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
        let opt = CreatePullRequestOption {
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
    async fn test_merge_pull_request_conflict() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/merge"))
            .respond_with(ResponseTemplate::new(405).set_body_json(serde_json::json!({
                "message": "merge conflict"
            })))
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
        assert!(!merged);
        assert_eq!(resp.status, 405);
    }

    #[tokio::test]
    async fn test_merge_pull_request_unexpected_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/merge"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "message": "internal error"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .merge(
                "testowner",
                "testrepo",
                1,
                MergePullRequestOption::default(),
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_patch_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1.patch"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "pull request not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.pulls().patch("testowner", "testrepo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_diff_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1.diff"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "pull request not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .diff(
                "testowner",
                "testrepo",
                1,
                PullRequestDiffOptions::default(),
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_review_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews/10"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "review not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .delete_review("testowner", "testrepo", 1, 10)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_review_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews/10"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "review not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .pulls()
            .get_review("testowner", "testrepo", 1, 10)
            .await;
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
        let opt = EditPullRequestOption {
            title: Some("Updated".to_string()),
            ..Default::default()
        };
        let result = client.pulls().edit("testowner", "testrepo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_review_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "message": "invalid review"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreatePullReviewOptions {
            state: Some(crate::types::enums::ReviewStateType::Approved),
            body: None,
            commit_id: None,
            comments: vec![],
        };
        let result = client
            .pulls()
            .create_review("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_submit_review_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/reviews/10"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "review not found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = SubmitPullReviewOptions {
            state: Some(crate::types::enums::ReviewStateType::Approved),
            body: None,
        };
        let result = client
            .pulls()
            .submit_review("testowner", "testrepo", 1, 10, opt)
            .await;
        assert!(result.is_err());
    }
}
