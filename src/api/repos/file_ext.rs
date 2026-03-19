// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use bytes::Bytes;

use crate::Response;
use crate::options::repo::*;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_file_ext.go (4 methods) ──────────────────────────────

    /// GetContentsExt get extended contents of a repository
    pub async fn get_contents_ext(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
        opt: GetContentsExtOptions,
    ) -> crate::Result<(ContentsExtResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let mut qs = format!(
            "ref={}&includes={}",
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC),
            percent_encoding::utf8_percent_encode(
                &opt.includes,
                percent_encoding::NON_ALPHANUMERIC
            ),
        );
        if !opt.r#ref.is_empty() {
            qs = format!(
                "ref={}&includes={}",
                percent_encoding::utf8_percent_encode(
                    &opt.r#ref,
                    percent_encoding::NON_ALPHANUMERIC
                ),
                percent_encoding::utf8_percent_encode(
                    &opt.includes,
                    percent_encoding::NON_ALPHANUMERIC
                ),
            );
        }
        let path = format!(
            "/repos/{}/{}/contents/{}?{}",
            escaped[0], escaped[1], escaped_path, qs
        );
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        let mut ext: ContentsExtResponse = serde_json::from_slice(&data)?;
        if ext.file_contents.is_none() && ext.dir_contents.is_none() {
            if let Ok(file_contents) = serde_json::from_slice::<ContentsResponse>(&data) {
                ext.file_contents = Some(file_contents);
            } else if let Ok(dir_contents) = serde_json::from_slice::<Vec<ContentsResponse>>(&data)
            {
                ext.dir_contents = Some(dir_contents);
            }
        }
        Ok((ext, resp))
    }

    /// GetEditorConfig get the editorconfig of a repository
    pub async fn get_editor_config(
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
            "/repos/{}/{}/editorconfig/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRawFileOrLFS get raw file from a repository, following LFS redirects
    pub async fn get_raw_file_or_lfs(
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

    /// GetRawFile get raw file from a repository
    pub async fn get_raw_file(
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
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_file_reader_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/README%2Emd"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"Hello World"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_file_reader("owner", "repo", "README.md", "main")
            .await
            .unwrap();
        assert_eq!(&*data, b"Hello World");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_file_reader_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_file_reader("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_contents_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "README.md",
            "path": "README.md",
            "sha": "abc",
            "type": "file",
            "size": 5,
            "last_commit_sha": "def"
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/README%2Emd"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (content, resp) = client
            .repos()
            .get_contents("owner", "repo", "README.md", "main")
            .await
            .unwrap();
        assert_eq!(content.name, "README.md");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_contents_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_contents("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_contents_happy() {
        let server = MockServer::start().await;
        let body = json!([{
            "name": "src",
            "path": "src",
            "sha": "abc",
            "type": "dir",
            "size": 0,
            "last_commit_sha": ""
        }]);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/src"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (items, resp) = client
            .repos()
            .list_contents("owner", "repo", "src", "main")
            .await
            .unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "src");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_contents_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/nonexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_contents("owner", "repo", "nonexist", "main")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_contents_ext_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "file_contents": {
                "name": "file.txt",
                "path": "file.txt",
                "sha": "abc",
                "type": "file",
                "size": 5,
                "last_commit_sha": ""
            }
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = GetContentsExtOptions::default();
        let (ext, resp) = client
            .repos()
            .get_contents_ext("owner", "repo", "file.txt", "main", opt)
            .await
            .unwrap();
        assert!(ext.file_contents.is_some());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_contents_ext_flat_file_payload() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "file.txt",
            "path": "file.txt",
            "sha": "abc",
            "type": "file",
            "size": 5,
            "last_commit_sha": ""
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = GetContentsExtOptions::default();
        let (ext, resp) = client
            .repos()
            .get_contents_ext("owner", "repo", "file.txt", "main", opt)
            .await
            .unwrap();
        assert_eq!(
            ext.file_contents
                .expect("flat file payload should map")
                .path,
            "file.txt"
        );
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_contents_ext_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = GetContentsExtOptions::default();
        let result = client
            .repos()
            .get_contents_ext("owner", "repo", "missing.txt", "main", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_editor_config_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/editorconfig/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"root = true\n"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_editor_config("owner", "repo", "file.txt", "main")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_editor_config_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/editorconfig/file%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_editor_config("owner", "repo", "file.txt", "main")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_raw_file_or_lfs_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"raw content"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_raw_file_or_lfs("owner", "repo", "file.txt", "main")
            .await
            .unwrap();
        assert_eq!(&*data, b"raw content");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_raw_file_or_lfs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_raw_file_or_lfs("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_raw_file_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"raw data"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_raw_file("owner", "repo", "file.txt", "main")
            .await
            .unwrap();
        assert_eq!(&*data, b"raw data");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_raw_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_raw_file("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }
}
