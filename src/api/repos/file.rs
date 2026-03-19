// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use bytes::Bytes;

use crate::Response;
use crate::internal::request::json_header;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_file.go (7 methods) ──────────────────────────────────

    /// GetFile download a file from a repository
    pub async fn get_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Bytes, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/contents/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetFileReader get a streaming reader for a file from a repository
    pub async fn get_file_reader(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Bytes, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/raw/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetContents get the metadata and contents of a file in a repository
    pub async fn get_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(ContentsResponse, Response)> {
        let (data, resp) = self.get_contents_raw(owner, repo, filepath, ref_).await?;
        let cr: ContentsResponse = serde_json::from_slice(&data)?;
        Ok((cr, resp))
    }

    /// ListContents get a list of entries in a directory
    pub async fn list_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<ContentsResponse>, Response)> {
        let (data, resp) = self.get_contents_raw(owner, repo, filepath, ref_).await?;
        let crl: Vec<ContentsResponse> = serde_json::from_slice(&data)?;
        Ok((crl, resp))
    }

    // ── Internal helpers ──────────────────────────────────────────

    /// Internal: get raw bytes for contents endpoint
    async fn get_contents_raw(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Bytes, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/contents/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_file() {
        use wiremock::matchers::query_param;

        let server = MockServer::start().await;
        let body = serde_json::json!({
            "name": "README.md",
            "path": "README.md",
            "sha": "abc123",
            "type": "file",
            "size": 10,
            "encoding": "base64",
            "content": "SGVsbG8=",
            "last_commit_sha": "def456"
        });

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/contents/README%2Emd",
            ))
            .and(query_param("ref", "main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_file("testowner", "testrepo", "README.md", "main")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_file_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "README.md",
            "path": "README.md",
            "sha": "abc123",
            "type": "file",
            "size": 10,
            "encoding": "base64",
            "content": "SGVsbG8=",
            "last_commit_sha": "def456"
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/README%2Emd"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_file("owner", "repo", "README.md", "main")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_file("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }
}
