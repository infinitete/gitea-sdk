// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Release API endpoints for managing Gitea repository releases and assets.

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::release::*;
use crate::pagination::QueryEncode;
use crate::types::{Attachment, Release};

/// API methods for releases. Access via [`Client::releases()`](crate::Client::releases).
pub struct ReleasesApi<'a> {
    client: &'a Client,
}

impl<'a> ReleasesApi<'a> {
    /// Create a new `ReleasesApi` for the given client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get a reference to the underlying client.
    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    // ── release.go ────────────────────────────────────────────────────

    /// ListReleases list releases of a repository
    pub async fn list(
        &self,
        owner: &str,
        repo: &str,
        opt: ListReleasesOptions,
    ) -> crate::Result<(Vec<Release>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/releases?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRelease get a release of a repository by id
    pub async fn get(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Release, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/releases/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetLatestRelease get the latest release of a repository
    pub async fn get_latest(&self, owner: &str, repo: &str) -> crate::Result<(Release, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/releases/latest", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetReleaseByTag get a release of a repository by tag
    pub async fn get_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::Result<(Release, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, tag])?;
        let path = format!(
            "/repos/{}/{}/releases/tags/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateRelease create a release
    pub async fn create(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateReleaseOption,
    ) -> crate::Result<(Release, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/releases", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditRelease edit a release
    pub async fn edit(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        form: EditReleaseOption,
    ) -> crate::Result<(Release, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&form)?;
        let path = format!("/repos/{}/{}/releases/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteRelease delete a release from a repository, keeping its tag
    pub async fn delete(&self, owner: &str, repo: &str, id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/releases/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// DeleteReleaseByTag delete a release from a repository by tag
    pub async fn delete_by_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, tag])?;
        let path = format!(
            "/repos/{}/{}/releases/tags/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── attachment.go ─────────────────────────────────────────────────

    /// ListReleaseAttachments list release's attachments
    pub async fn list_attachments(
        &self,
        owner: &str,
        repo: &str,
        release: i64,
        opt: ListReleaseAttachmentsOptions,
    ) -> crate::Result<(Vec<Attachment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/releases/{release}/assets?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetReleaseAttachment returns the requested attachment
    pub async fn get_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: i64,
        id: i64,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/releases/{release}/assets/{id}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateReleaseAttachment creates an attachment for the given release.
    /// `data` is the raw file content to upload; `filename` is the name the
    /// attachment will have on the server.
    pub async fn create_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: i64,
        data: Vec<u8>,
        filename: &str,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/releases/{release}/assets",
            escaped[0], escaped[1]
        );
        let form = reqwest::multipart::Form::new().part(
            "attachment",
            reqwest::multipart::Part::bytes(data).file_name(filename.to_string()),
        );
        self.client()
            .get_parsed_response_multipart(reqwest::Method::POST, &path, None, form)
            .await
    }

    /// EditReleaseAttachment updates the given attachment with the given options
    pub async fn edit_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: i64,
        attachment: i64,
        form: EditAttachmentOption,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        form.validate()?;
        let body = json_body(&form)?;
        let path = format!(
            "/repos/{}/{}/releases/{release}/assets/{attachment}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteReleaseAttachment deletes the given attachment including the uploaded file
    pub async fn delete_attachment(
        &self,
        owner: &str,
        repo: &str,
        release: i64,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/releases/{release}/assets/{id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn error_response(status: u16, message: &str) -> ResponseTemplate {
        ResponseTemplate::new(status).set_body_json(serde_json::json!({"message": message}))
    }

    fn attachment_json(id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "size": 2048,
            "download_count": 3,
            "created": "2024-01-15T10:00:00Z",
            "uuid": "def456",
            "browser_download_url": "https://example.com/attachments/def456"
        })
    }

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn release_json(id: i64, tag: &str, title: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "tag_name": tag,
            "target": "main",
            "title": title,
            "note": "Release notes",
            "draft": false,
            "prerelease": false,
            "url": "",
            "html_url": "",
            "tarball_url": "",
            "zipball_url": "",
            "created_at": "2024-01-15T10:00:00Z",
            "published_at": "2024-01-15T10:00:00Z",
            "assets": []
        })
    }

    #[tokio::test]
    async fn test_list_releases() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            release_json(1, "v1.0.0", "v1.0.0"),
            release_json(2, "v1.1.0", "v1.1.0"),
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/releases"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (releases, resp) = client
            .releases()
            .list("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(releases.len(), 2);
        assert_eq!(releases[0].id, 1);
        assert_eq!(releases[0].tag_name, "v1.0.0");
        assert_eq!(releases[1].id, 2);
        assert_eq!(releases[1].tag_name, "v1.1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_release() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/releases"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(release_json(3, "v2.0.0", "v2.0.0")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateReleaseOption {
            tag_name: "v2.0.0".to_string(),
            target: Some("main".to_string()),
            title: Some("v2.0.0".to_string()),
            note: Some("New release".to_string()),
            is_draft: false,
            is_prerelease: false,
        };
        let (release, resp) = client
            .releases()
            .create("testowner", "testrepo", opt)
            .await
            .unwrap();
        assert_eq!(release.id, 3);
        assert_eq!(release.tag_name, "v2.0.0");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_get_release() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/releases/1"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(release_json(1, "v1.0.0", "v1.0.0")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (release, resp) = client
            .releases()
            .get("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(release.id, 1);
        assert_eq!(release.tag_name, "v1.0.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_error_case() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/releases/999"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({"message": "Release not found"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.releases().get("testowner", "testrepo", 999).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Release not found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[tokio::test]
    async fn test_delete_release() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/releases/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .releases()
            .delete("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_create_release_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);

        let opt = CreateReleaseOption {
            tag_name: "".to_string(),
            target: None,
            title: None,
            note: None,
            is_draft: false,
            is_prerelease: false,
        };
        let result = client.releases().create("testowner", "testrepo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_attachment() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/releases/1/assets"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 10,
                "name": "binary.zip",
                "size": 1024,
                "download_count": 0,
                "created": "2024-01-15T10:00:00Z",
                "uuid": "abc123",
                "browser_download_url": "https://example.com/attachments/abc123"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (attachment, resp) = client
            .releases()
            .create_attachment(
                "testowner",
                "testrepo",
                1,
                b"file content".to_vec(),
                "binary.zip",
            )
            .await
            .unwrap();
        assert_eq!(attachment.id, 10);
        assert_eq!(attachment.name, "binary.zip");
        assert_eq!(resp.status, 201);
    }

    // ── list: error path ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_releases_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases"))
            .respond_with(error_response(500, "internal error"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .releases()
            .list("testowner", "testrepo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_latest: happy path ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_latest_release() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/releases/latest"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(release_json(5, "v2.0.0", "v2.0.0")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (release, resp) = client
            .releases()
            .get_latest("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(release.id, 5);
        assert_eq!(release.tag_name, "v2.0.0");
        assert_eq!(resp.status, 200);
    }

    // ── get_latest: error path ────────────────────────────────────────

    #[tokio::test]
    async fn test_get_latest_release_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/releases/latest"))
            .respond_with(error_response(404, "not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.releases().get_latest("testowner", "testrepo").await;
        assert!(result.is_err());
    }

    // ── get_by_tag: happy path ────────────────────────────────────────

    #[tokio::test]
    async fn test_get_release_by_tag() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/tags/[^/]+"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(release_json(4, "v1.5.0", "v1.5.0")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (release, resp) = client
            .releases()
            .get_by_tag("testowner", "testrepo", "v1.5.0")
            .await
            .unwrap();
        assert_eq!(release.id, 4);
        assert_eq!(release.tag_name, "v1.5.0");
        assert_eq!(resp.status, 200);
    }

    // ── get_by_tag: error path ────────────────────────────────────────

    #[tokio::test]
    async fn test_get_release_by_tag_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/tags/[^/]+"))
            .respond_with(error_response(404, "release not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .releases()
            .get_by_tag("testowner", "testrepo", "nonexistent")
            .await;
        assert!(result.is_err());
    }

    // ── create: error path (server error) ─────────────────────────────

    #[tokio::test]
    async fn test_create_release_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/releases"))
            .respond_with(error_response(500, "server error"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateReleaseOption {
            tag_name: "v3.0.0".to_string(),
            target: None,
            title: None,
            note: None,
            is_draft: false,
            is_prerelease: false,
        };
        let result = client.releases().create("testowner", "testrepo", opt).await;
        assert!(result.is_err());
    }

    // ── edit: happy path ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_release() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/\d+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(release_json(
                1,
                "v1.0.0",
                "Updated Title",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let form = EditReleaseOption {
            tag_name: None,
            target: None,
            title: Some("Updated Title".to_string()),
            note: None,
            is_draft: None,
            is_prerelease: None,
        };
        let (release, resp) = client
            .releases()
            .edit("testowner", "testrepo", 1, form)
            .await
            .unwrap();
        assert_eq!(release.id, 1);
        assert_eq!(release.title, "Updated Title");
        assert_eq!(resp.status, 200);
    }

    // ── edit: error path ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_release_error() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/\d+"))
            .respond_with(error_response(404, "release not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let form = EditReleaseOption {
            tag_name: None,
            target: None,
            title: Some("New Title".to_string()),
            note: None,
            is_draft: None,
            is_prerelease: None,
        };
        let result = client
            .releases()
            .edit("testowner", "testrepo", 999, form)
            .await;
        assert!(result.is_err());
    }

    // ── delete: error path ────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_release_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/\d+"))
            .respond_with(error_response(404, "release not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.releases().delete("testowner", "testrepo", 999).await;
        assert!(result.is_err());
    }

    // ── delete_by_tag: happy path ─────────────────────────────────────

    #[tokio::test]
    async fn test_delete_release_by_tag() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/tags/[^/]+"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .releases()
            .delete_by_tag("testowner", "testrepo", "v1.0.0")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    // ── delete_by_tag: error path ─────────────────────────────────────

    #[tokio::test]
    async fn test_delete_release_by_tag_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/tags/[^/]+"))
            .respond_with(error_response(404, "release not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .releases()
            .delete_by_tag("testowner", "testrepo", "nonexistent")
            .await;
        assert!(result.is_err());
    }

    // ── list_attachments: happy path ──────────────────────────────────

    #[tokio::test]
    async fn test_list_attachments() {
        let server = MockServer::start().await;

        let body = serde_json::json!([
            attachment_json(1, "binary.zip"),
            attachment_json(2, "checksum.txt"),
        ]);

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (attachments, resp) = client
            .releases()
            .list_attachments("testowner", "testrepo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(attachments.len(), 2);
        assert_eq!(attachments[0].id, 1);
        assert_eq!(attachments[0].name, "binary.zip");
        assert_eq!(attachments[1].id, 2);
        assert_eq!(attachments[1].name, "checksum.txt");
        assert_eq!(resp.status, 200);
    }

    // ── list_attachments: error path ──────────────────────────────────

    #[tokio::test]
    async fn test_list_attachments_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets"))
            .respond_with(error_response(500, "internal error"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .releases()
            .list_attachments("testowner", "testrepo", 999, Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_attachment: happy path ────────────────────────────────────

    #[tokio::test]
    async fn test_get_attachment() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets/\d+",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(attachment_json(10, "binary.zip")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (attachment, resp) = client
            .releases()
            .get_attachment("testowner", "testrepo", 1, 10)
            .await
            .unwrap();
        assert_eq!(attachment.id, 10);
        assert_eq!(attachment.name, "binary.zip");
        assert_eq!(resp.status, 200);
    }

    // ── get_attachment: error path ────────────────────────────────────

    #[tokio::test]
    async fn test_get_attachment_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets/\d+",
            ))
            .respond_with(error_response(404, "attachment not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .releases()
            .get_attachment("testowner", "testrepo", 1, 999)
            .await;
        assert!(result.is_err());
    }

    // ── create_attachment: error path ─────────────────────────────────

    #[tokio::test]
    async fn test_create_attachment_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets"))
            .respond_with(error_response(500, "upload failed"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .releases()
            .create_attachment(
                "testowner",
                "testrepo",
                1,
                b"file content".to_vec(),
                "binary.zip",
            )
            .await;
        assert!(result.is_err());
    }

    // ── edit_attachment: happy path ───────────────────────────────────

    #[tokio::test]
    async fn test_edit_attachment() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets/\d+",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(attachment_json(10, "renamed.zip")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let form = EditAttachmentOption {
            name: "renamed.zip".to_string(),
        };
        let (attachment, resp) = client
            .releases()
            .edit_attachment("testowner", "testrepo", 1, 10, form)
            .await
            .unwrap();
        assert_eq!(attachment.id, 10);
        assert_eq!(attachment.name, "renamed.zip");
        assert_eq!(resp.status, 200);
    }

    // ── edit_attachment: error path ───────────────────────────────────

    #[tokio::test]
    async fn test_edit_attachment_error() {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/releases/\d+/assets/\d+",
            ))
            .respond_with(error_response(404, "attachment not found"))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let form = EditAttachmentOption {
            name: "new-name.zip".to_string(),
        };
        let result = client
            .releases()
            .edit_attachment("testowner", "testrepo", 1, 999, form)
            .await;
        assert!(result.is_err());
    }

    // ── edit_attachment: validation error ─────────────────────────────

    #[tokio::test]
    async fn test_edit_attachment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);

        let form = EditAttachmentOption {
            name: "   ".to_string(),
        };
        let result = client
            .releases()
            .edit_attachment("testowner", "testrepo", 1, 10, form)
            .await;
        assert!(result.is_err());
    }
}
