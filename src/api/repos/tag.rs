// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;
use crate::version::{VERSION_1_14_0, VERSION_1_15_0};

impl<'a> super::ReposApi<'a> {
    // ── repo_tag.go (5 methods) ───────────────────────────────────

    /// `ListTags` list a repository's tags
    pub async fn list_tags(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoTagsOptions,
    ) -> crate::Result<(Vec<Tag>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/tags?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetTag` get a single tag of a repository
    pub async fn get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::Result<(Tag, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, tag])?;
        let path = format!("/repos/{}/{}/tags/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetAnnotatedTag` get an annotated tag of a repository
    pub async fn get_annotated_tag(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::Result<(AnnotatedTag, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/tags/{}", escaped[0], escaped[1], sha);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateTag` create a tag in a repository
    pub async fn create_tag(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateTagOption,
    ) -> crate::Result<(Tag, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tags", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteTag` delete a tag from a repository
    pub async fn delete_tag(&self, owner: &str, repo: &str, tag: &str) -> crate::Result<Response> {
        self.client()
            .check_server_version_ge(&VERSION_1_14_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, tag])?;
        let path = format!("/repos/{}/{}/tags/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
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
    async fn test_list_tags_happy() {
        let server = MockServer::start().await;
        let body = json!([{
            "name": "v1.0",
            "message": "release 1.0",
            "id": "abc123",
            "commit": null,
            "zipball_url": "",
            "tarball_url": ""
        }]);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tags, resp) = client
            .repos()
            .list_tags("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "v1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_tags_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_tags("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tag_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "v1.0",
            "message": "release",
            "id": "sha123",
            "commit": null,
            "zipball_url": "",
            "tarball_url": ""
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags/v1%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tag, resp) = client
            .repos()
            .get_tag("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(tag.name, "v1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags/nonexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_tag("owner", "repo", "nonexist").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_annotated_tag_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "tag": "v1.0",
            "sha": "abc123",
            "url": "https://example.com",
            "message": "annotated tag",
            "tagger": null,
            "object": null,
            "verification": null
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/tags/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tag, resp) = client
            .repos()
            .get_annotated_tag("owner", "repo", "abc123")
            .await
            .unwrap();
        assert_eq!(tag.tag, "v1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_annotated_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/tags/badsha"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_annotated_tag("owner", "repo", "badsha")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_tag_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "name": "v2.0",
            "message": "release 2",
            "id": "sha456",
            "commit": null,
            "zipball_url": "",
            "tarball_url": ""
        });
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tag, resp) = client
            .repos()
            .create_tag(
                "owner",
                "repo",
                CreateTagOption {
                    tag_name: "v2.0".to_string(),
                    message: "release 2".to_string(),
                    target: "sha456".to_string(),
                },
            )
            .await
            .unwrap();
        assert_eq!(tag.name, "v2.0");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(409).set_body_json(json!({"message": "Conflict"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .create_tag(
                "owner",
                "repo",
                CreateTagOption {
                    tag_name: "v2.0".to_string(),
                    message: "release 2".to_string(),
                    target: "sha456".to_string(),
                },
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_tag_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tags/v1%2E0"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .repos()
            .delete_tag("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tags/v1.0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_tag("owner", "repo", "v1.0").await;
        assert!(result.is_err());
    }
}
