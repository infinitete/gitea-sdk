// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use bytes::Bytes;

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::pull::*;
use crate::pagination::QueryEncode;
use crate::version::{VERSION_1_11_5, VERSION_1_13_0};

// ── pull.go ────────────────────────────────────────────────────

impl<'a> super::PullsApi<'a> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
}

#[cfg(test)]
mod tests {
    use crate::Client;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::create_test_client;

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
                crate::options::pull::MergePullRequestOption::default(),
            )
            .await
            .unwrap();
        assert!(merged);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_is_merged() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/pulls/1/merge"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

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
        assert_eq!(String::from_utf8(body.to_vec()).unwrap(), patch);
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
                crate::options::pull::PullRequestDiffOptions::default(),
            )
            .await
            .unwrap();
        assert_eq!(String::from_utf8(body.to_vec()).unwrap(), diff_body);
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
                crate::options::pull::PullRequestDiffOptions::default(),
            )
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
                crate::options::pull::MergePullRequestOption {
                    style: Some(crate::types::enums::MergeStyle::Squash),
                    ..Default::default()
                },
            )
            .await;
        assert!(matches!(result, Err(crate::Error::Version(_))));
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
                crate::options::pull::MergePullRequestOption::default(),
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
                crate::options::pull::MergePullRequestOption::default(),
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
                crate::options::pull::PullRequestDiffOptions::default(),
            )
            .await;
        assert!(result.is_err());
    }
}
