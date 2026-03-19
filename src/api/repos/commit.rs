// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;
use crate::version::{VERSION_1_16_0, VERSION_1_22_0};
use bytes::Bytes;

impl<'a> super::ReposApi<'a> {
    // ── repo_commit.go (4 methods) ────────────────────────────────

    /// GetSingleCommit get a single commit of a repository
    pub async fn get_single_commit(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Commit, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/commits/{}", escaped[0], escaped[1], ref_);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListCommits list commits of a repository
    pub async fn list_commits(
        &self,
        owner: &str,
        repo: &str,
        opt: ListCommitOptions,
    ) -> crate::Result<(Vec<Commit>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/commits?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetCommitDiff get the diff of a commit
    pub async fn get_commit_diff(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Bytes, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_16_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/git/commits/{}.diff",
            escaped[0], escaped[1], ref_
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetCommitPatch get the patch of a commit
    pub async fn get_commit_patch(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Bytes, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_16_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/git/commits/{}.patch",
            escaped[0], escaped[1], ref_
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_compare.go (1 method) ────────────────────────────────

    /// CompareCommits compare two commits
    pub async fn compare_commits(
        &self,
        owner: &str,
        repo: &str,
        before: &str,
        after: &str,
    ) -> crate::Result<(Compare, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_22_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/compare/{}...{}",
            escaped[0], escaped[1], before, after
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRepoNote get a note for a specific commit
    pub async fn get_repo_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        opt: GetRepoNoteOptions,
    ) -> crate::Result<(Note, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, sha])?;
        let mut qs = String::new();
        if let Some(v) = &opt.verification {
            qs.push_str(&format!(
                "verification={}",
                percent_encoding::utf8_percent_encode(
                    if *v { "true" } else { "false" },
                    percent_encoding::NON_ALPHANUMERIC,
                )
            ));
        }
        if let Some(v) = &opt.files {
            if !qs.is_empty() {
                qs.push('&');
            }
            qs.push_str(&format!(
                "files={}",
                percent_encoding::utf8_percent_encode(
                    if *v { "true" } else { "false" },
                    percent_encoding::NON_ALPHANUMERIC,
                )
            ));
        }
        let mut path = format!(
            "/repos/{}/{}/git/notes/{}",
            escaped[0], escaped[1], escaped[2]
        );
        if !qs.is_empty() {
            path = format!("{}?{}", path, qs);
        }
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        let note: Note = serde_json::from_slice(&data)?;
        Ok((note, resp))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use crate::options::repo::*;

    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_single_commit_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_commit_json("abc123")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (commit, resp) = client
            .repos()
            .get_single_commit("owner", "repo", "abc123")
            .await
            .unwrap();
        assert_eq!(commit.commit_meta.sha, "abc123");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_single_commit_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_single_commit("owner", "repo", "abc123")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_commits_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/commits"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_commit_json("sha1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (commits, resp) = client
            .repos()
            .list_commits("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_commits_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/commits"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_commits("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_commit_diff_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.diff"))
            .respond_with(ResponseTemplate::new(200).set_body_string("diff --git a/file b/file"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (diff, resp) = client
            .repos()
            .get_commit_diff("owner", "repo", "abc123")
            .await
            .unwrap();
        assert!(!diff.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_commit_diff_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.diff"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_commit_diff("owner", "repo", "abc123")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_commit_patch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.patch"))
            .respond_with(ResponseTemplate::new(200).set_body_string("patch content"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (patch, resp) = client
            .repos()
            .get_commit_patch("owner", "repo", "abc123")
            .await
            .unwrap();
        assert!(!patch.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_commit_patch_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.patch"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_commit_patch("owner", "repo", "abc123")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_compare_commits_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/compare/abc123...def456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_compare_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .compare_commits("owner", "repo", "abc123", "def456")
            .await;
        assert!(result.is_ok());
        let (compare, resp) = result.unwrap();
        assert_eq!(compare.total_commits, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_compare_commits_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/compare/abc123...def456"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .compare_commits("owner", "repo", "abc123", "def456")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_note_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/notes/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_note_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_note(
                "owner",
                "repo",
                "abc123",
                GetRepoNoteOptions {
                    verification: None,
                    files: None,
                },
            )
            .await;
        assert!(result.is_ok());
        let (note, resp) = result.unwrap();
        assert_eq!(note.message, "Test note");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_note_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/notes/abc123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_note(
                "owner",
                "repo",
                "abc123",
                GetRepoNoteOptions {
                    verification: None,
                    files: None,
                },
            )
            .await;
        assert!(result.is_err());
    }
}
