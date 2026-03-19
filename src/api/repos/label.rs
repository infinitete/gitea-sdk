// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::Label;

impl<'a> super::ReposApi<'a> {
    // ── repo_label.go (5 methods) ─────────────────────────────────

    /// ListLabels list repository's labels
    pub async fn list_labels(
        &self,
        owner: &str,
        repo: &str,
        opt: ListLabelsOptions,
    ) -> crate::Result<(Vec<Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/labels?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetLabel get a single label
    pub async fn get_label(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/labels/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateLabel create a label
    pub async fn create_label(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/labels", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditLabel edit a label
    pub async fn edit_label(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/labels/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteLabel delete a label
    pub async fn delete_label(&self, owner: &str, repo: &str, id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/labels/{id}", escaped[0], escaped[1]);
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
    async fn test_list_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_label_json(1, "bug")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (labels, resp) = client
            .repos()
            .list_labels("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_labels("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_label_json(1, "bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (label, resp) = client.repos().get_label("owner", "repo", 1).await.unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_label("owner", "repo", 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(minimal_label_json(2, "feature")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        let (label, resp) = client
            .repos()
            .create_label("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(label.id, 2);
        assert_eq!(label.name, "feature");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        let result = client.repos().create_label("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_label_json(1, "bugfix")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditLabelOption {
            name: Some("bugfix".to_string()),
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
        };
        let (label, resp) = client
            .repos()
            .edit_label("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(label.name, "bugfix");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditLabelOption {
            name: Some("bugfix".to_string()),
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
        };
        let result = client.repos().edit_label("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_label("owner", "repo", 1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_label("owner", "repo", 1).await;
        assert!(result.is_err());
    }
}
