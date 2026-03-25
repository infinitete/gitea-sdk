// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use bytes::Bytes;

use std::collections::HashMap;

use crate::Response;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    pub async fn mirror_sync(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/mirror-sync", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// `GetRepoLanguages` get languages of a repository
    pub async fn get_repo_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(HashMap<String, i64>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/languages", escaped[0], escaped[1]);
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        let langs: HashMap<String, i64> = serde_json::from_slice(&data)?;
        Ok((langs, resp))
    }

    /// `GetArchive` get an archive of a repository
    pub async fn get_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        archive: &str,
    ) -> crate::Result<(Bytes, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/archive/{}.{}",
            escaped[0], escaped[1], ref_, archive
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetArchiveReader` get an archive streaming reader of a repository
    pub async fn get_archive_reader(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        archive: &str,
    ) -> crate::Result<(Bytes, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/archive/{}.{}",
            escaped[0], escaped[1], ref_, archive
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `UpdateRepoAvatar` update the avatar of a repository
    pub async fn update_repo_avatar(
        &self,
        owner: &str,
        repo: &str,
        file_content: &[u8],
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/avatar", escaped[0], escaped[1]);
        let form = reqwest::multipart::Form::new().part(
            "avatar",
            reqwest::multipart::Part::bytes(file_content.to_vec()).file_name("avatar".to_string()),
        );
        self.client()
            .get_parsed_response_multipart(reqwest::Method::PUT, &path, None, form)
            .await
    }

    /// `DeleteRepoAvatar` delete the avatar of a repository
    pub async fn delete_repo_avatar(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/avatar", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
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
    async fn test_get_repo_languages() {
        let server = MockServer::start().await;
        let body = serde_json::json!({"Go": 1000, "Rust": 500});

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/languages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (langs, resp) = client
            .repos()
            .get_repo_languages("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(langs.get("Go"), Some(&1000));
        assert_eq!(langs.get("Rust"), Some(&500));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_mirror_sync_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/mirror-sync"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.repos().mirror_sync("owner", "repo").await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_mirror_sync_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/mirror-sync"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().mirror_sync("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_languages_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/languages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"Go": 1000})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (langs, resp) = client
            .repos()
            .get_repo_languages("owner", "repo")
            .await
            .unwrap();
        assert_eq!(langs.get("Go"), Some(&1000));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_languages_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/languages"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_repo_languages("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_archive_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.tar.gz"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake-archive-data"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_archive("owner", "repo", "main", "tar.gz")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_archive_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.tar.gz"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_archive("owner", "repo", "main", "tar.gz")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_archive_reader_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.zip"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake-zip-data"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_archive_reader("owner", "repo", "main", "zip")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_archive_reader_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.zip"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_archive_reader("owner", "repo", "main", "zip")
            .await;
        assert!(result.is_err());
    }
}
