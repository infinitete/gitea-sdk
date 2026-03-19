// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use bytes::Bytes;

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
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

    /// CreateFile create a file in a repository
    pub async fn create_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        opt: CreateFileOptions,
    ) -> crate::Result<(FileResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/contents/{}",
            escaped[0], escaped[1], escaped_path
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

    /// UpdateFile update a file in a repository
    pub async fn update_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        opt: UpdateFileOptions,
    ) -> crate::Result<(FileResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/contents/{}",
            escaped[0], escaped[1], escaped_path
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteFile delete a file from a repository
    pub async fn delete_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        opt: DeleteFileOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/contents/{}",
            escaped[0], escaped[1], escaped_path
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
    use super::*;
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

    #[tokio::test]
    async fn test_create_file_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "content": {
                "name": "newfile.txt",
                "path": "newfile.txt",
                "sha": "abc",
                "type": "file",
                "size": 5,
                "last_commit_sha": "def"
            },
            "commit": {
                "sha": "commit123",
                "url": "https://example.com",
                "html_url": "https://example.com/commit123",
                "created": "2024-01-01T00:00:00Z",
                "message": "create file",
                "parents": []
            }
        });
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/contents/newfile%2Etxt"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateFileOptions {
            file_options: FileOptions {
                message: "create file".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            content: "SGVsbG8=".to_string(),
        };
        let (fr, resp) = client
            .repos()
            .create_file("owner", "repo", "newfile.txt", opt)
            .await
            .unwrap();
        assert!(fr.content.is_some());
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/contents/newfile%2Etxt"))
            .respond_with(
                ResponseTemplate::new(422).set_body_json(json!({"message": "Validation error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateFileOptions {
            file_options: FileOptions {
                message: "create file".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            content: "SGVsbG8=".to_string(),
        };
        let result = client
            .repos()
            .create_file("owner", "repo", "newfile.txt", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_file_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "content": {
                "name": "file.txt",
                "path": "file.txt",
                "sha": "newsha",
                "type": "file",
                "size": 10,
                "last_commit_sha": "commit2"
            },
            "commit": {
                "sha": "commit2",
                "url": "https://example.com",
                "html_url": "https://example.com/commit2",
                "created": "2024-01-01T00:00:00Z",
                "message": "update file",
                "parents": []
            }
        });
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateFileOptions {
            file_options: FileOptions {
                message: "update file".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "oldsha".to_string(),
            content: "bmV3IGNvbnRlbnQ=".to_string(),
            from_path: String::new(),
        };
        let (fr, resp) = client
            .repos()
            .update_file("owner", "repo", "file.txt", opt)
            .await
            .unwrap();
        assert!(fr.content.is_some());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(
                ResponseTemplate::new(422).set_body_json(json!({"message": "SHA mismatch"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateFileOptions {
            file_options: FileOptions {
                message: "update".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "oldsha".to_string(),
            content: "bmV3".to_string(),
            from_path: String::new(),
        };
        let result = client
            .repos()
            .update_file("owner", "repo", "file.txt", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_file_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = DeleteFileOptions {
            file_options: FileOptions {
                message: "delete".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "abc123".to_string(),
        };
        let resp = client
            .repos()
            .delete_file("owner", "repo", "file.txt", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_delete_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = DeleteFileOptions {
            file_options: FileOptions {
                message: "delete".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "abc123".to_string(),
        };
        let result = client
            .repos()
            .delete_file("owner", "repo", "file.txt", opt)
            .await;
        assert!(result.is_err());
    }
}
