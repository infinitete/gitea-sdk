// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;
use crate::version::VERSION_1_23_0;

impl<'a> super::ReposApi<'a> {
    // ── repo_tag_protection.go (5 methods) ────────────────────────

    /// `ListTagProtections` list tag protections
    pub async fn list_tag_protections(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoTagProtectionsOptions,
    ) -> crate::Result<(Vec<TagProtection>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_23_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/tag_protections?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetTagProtection` get a tag protection
    pub async fn get_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateTagProtection` create a tag protection
    pub async fn create_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateTagProtectionOption,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tag_protections", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `EditTagProtection` edit a tag protection
    pub async fn edit_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditTagProtectionOption,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteTagProtection` delete a tag protection
    pub async fn delete_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
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
    async fn test_list_tag_protections_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_tag_protection_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_tag_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (protections, resp) = result.unwrap();
        assert_eq!(protections.len(), 1);
        assert_eq!(protections[0].name_pattern, "v*");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_tag_protections_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_tag_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let result = client.repos().get_tag_protection("owner", "repo", 1).await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(tp.name_pattern, "v*");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_tag_protection("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let opt = CreateTagProtectionOption {
            name_pattern: "v*".to_string(),
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .create_tag_protection("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateTagProtectionOption {
            name_pattern: "v*".to_string(),
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .create_tag_protection("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let opt = EditTagProtectionOption {
            name_pattern: None,
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .edit_tag_protection("owner", "repo", 1, opt)
            .await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditTagProtectionOption {
            name_pattern: None,
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .edit_tag_protection("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_tag_protection("owner", "repo", 1)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_tag_protection("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }
}
