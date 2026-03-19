// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::pull::*;
use crate::pagination::QueryEncode;
use crate::types::{PullReview, PullReviewComment};
use crate::version::VERSION_1_12_0;

// ── pull_review.go ─────────────────────────────────────────────

impl<'a> super::PullsApi<'a> {
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
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, review_json};

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
        let comment1 = serde_json::json!({"id": 100, "body": "fix this line", "pull_request_review_id": 10, "path": "src/main.rs", "commit_id": "abc123", "original_commit_id": "abc123", "diff_hunk": "@@ -1,3 +1,4 @@", "position": 5, "original_position": 5, "html_url": "", "pull_request_url": "", "created_at": "2024-01-15T10:00:00Z", "updated_at": "2024-01-15T10:00:00Z"});
        let comment2 = serde_json::json!({"id": 101, "body": "nit: spacing", "pull_request_review_id": 10, "path": "src/lib.rs", "commit_id": "abc123", "original_commit_id": "abc123", "diff_hunk": "@@ -10,3 +10,3 @@", "position": 12, "original_position": 12, "html_url": "", "pull_request_url": "", "created_at": "2024-01-15T10:00:00Z", "updated_at": "2024-01-15T10:00:00Z"});
        let body = serde_json::json!([&comment1, &comment2]);

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
}
