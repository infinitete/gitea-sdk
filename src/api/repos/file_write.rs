// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_file.go: write operations ────────────────────────────

    /// `CreateFile` create a file in a repository
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

    /// `UpdateFile` update a file in a repository
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

    /// `DeleteFile` delete a file from a repository
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
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use crate::options::repo::*;
    use crate::types::repository::{CommitDateOptions, Identity};
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn test_file_options() -> FileOptions {
        FileOptions {
            message: "test".to_string(),
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
        }
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
            file_options: test_file_options(),
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
            file_options: test_file_options(),
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
            file_options: test_file_options(),
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
            file_options: test_file_options(),
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
            file_options: test_file_options(),
            sha: "abc123".to_string(),
        };
        let result = client
            .repos()
            .delete_file("owner", "repo", "file.txt", opt)
            .await;
        assert!(result.is_err());
    }
}
