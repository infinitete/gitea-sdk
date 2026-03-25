// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::Label;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    // ── org_label.go ──────────────────────────────────────────────────────

    /// `ListOrgLabels` returns the labels defined at the org level
    pub async fn list_org_labels(
        &self,
        org: &str,
        opt: ListOrgLabelsOptions,
    ) -> crate::Result<(Vec<Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/labels?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// `CreateOrgLabel` creates a new label under an organization
    pub async fn create_org_label(
        &self,
        org: &str,
        opt: CreateOrgLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/labels", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `GetOrgLabel` get one label of organization by org id
    pub async fn get_org_label(
        &self,
        org: &str,
        label_id: i64,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/labels/{label_id}", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `EditOrgLabel` edits an existing org-level label by ID
    pub async fn edit_org_label(
        &self,
        org: &str,
        label_id: i64,
        opt: EditOrgLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/labels/{label_id}", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteOrgLabel` deletes an org label by ID
    pub async fn delete_org_label(&self, org: &str, label_id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/labels/{label_id}", escaped[0]);
        let (_, response) = self
            .client()
            .get_response(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_org_labels ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_labels_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([label_json(1, "bug")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (labels, resp) = client
            .orgs()
            .list_org_labels("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_labels("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── create_org_label ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(201).set_body_json(label_json(10, "feature")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: None,
            exclusive: None,
        };
        let (label, resp) = client
            .orgs()
            .create_org_label("testorg", opt)
            .await
            .unwrap();
        assert_eq!(label.name, "feature");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: None,
            exclusive: None,
        };
        let result = client.orgs().create_org_label("testorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_label_validation_invalid_color() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgLabelOption {
            name: "badlabel".to_string(),
            color: "not-a-color".to_string(),
            description: None,
            exclusive: None,
        };
        let result = client.orgs().create_org_label("testorg", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid color format")
        );
    }

    // ── get_org_label ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(label_json(42, "bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (label, resp) = client.orgs().get_org_label("testorg", 42).await.unwrap();
        assert_eq!(label.id, 42);
        assert_eq!(label.name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org_label("testorg", 999).await;
        assert!(result.is_err());
    }

    // ── edit_org_label ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(label_json(42, "updated-bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgLabelOption {
            name: Some("updated-bug".to_string()),
            color: Some("0000ff".to_string()),
            ..Default::default()
        };
        let (label, resp) = client
            .orgs()
            .edit_org_label("testorg", 42, opt)
            .await
            .unwrap();
        assert_eq!(label.name, "updated-bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgLabelOption {
            name: Some("updated".to_string()),
            ..Default::default()
        };
        let result = client.orgs().edit_org_label("testorg", 42, opt).await;
        assert!(result.is_err());
    }

    // ── delete_org_label ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_org_label("testorg", 42).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_org_label("testorg", 42).await;
        assert!(result.is_err());
    }
}
