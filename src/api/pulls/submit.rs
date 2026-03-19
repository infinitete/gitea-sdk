// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::pull::*;
use crate::types::PullReview;
use crate::version::VERSION_1_12_0;

impl<'a> super::PullsApi<'a> {
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
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, review_json};

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
        let opt = crate::options::pull::CreatePullReviewOptions {
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

        let opt = crate::options::pull::CreatePullReviewOptions {
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

        let opt = crate::options::pull::CreatePullReviewOptions {
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
        let opt = crate::options::pull::SubmitPullReviewOptions {
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

        let opt = crate::options::pull::SubmitPullReviewOptions {
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

        let opt = crate::options::pull::SubmitPullReviewOptions {
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
        let opt = crate::options::pull::CreatePullReviewOptions {
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
        let opt = crate::options::pull::SubmitPullReviewOptions {
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
