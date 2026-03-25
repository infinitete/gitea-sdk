// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! `OAuth2` API endpoints for managing Gitea `OAuth2` applications and grants.

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::oauth2::*;
use crate::pagination::QueryEncode;
use crate::types::Oauth2;

/// API methods for `OAuth2` applications. Access via [`Client::oauth2()`](crate::Client::oauth2).
pub struct Oauth2Api<'a> {
    client: &'a Client,
}

impl<'a> Oauth2Api<'a> {
    /// Create a new `Oauth2Api` view.
    #[must_use]
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// `ListOauth2` all of your Oauth2 Applications
    pub async fn list_applications(
        &self,
        opt: ListOauth2Option,
    ) -> crate::Result<(Vec<Oauth2>, Response)> {
        let path = format!("/user/applications/oauth2?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// `CreateOauth2` create an Oauth2 Application and returns a completed Oauth2 object
    pub async fn create_application(
        &self,
        opt: CreateOauth2Option,
    ) -> crate::Result<(Oauth2, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/applications/oauth2",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `GetOauth2` a specific Oauth2 Application by ID
    pub async fn get_application(&self, id: i64) -> crate::Result<(Oauth2, Response)> {
        let path = format!("/user/applications/oauth2/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `UpdateOauth2` a specific Oauth2 Application by ID and return a completed Oauth2 object
    pub async fn update_application(
        &self,
        id: i64,
        opt: CreateOauth2Option,
    ) -> crate::Result<(Oauth2, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/user/applications/oauth2/{id}");
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteOauth2` delete an Oauth2 application by ID
    pub async fn delete_application(&self, id: i64) -> crate::Result<Response> {
        let path = format!("/user/applications/oauth2/{id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn oauth2_json(id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "client_id": "abc123",
            "client_secret": "secret456",
            "redirect_uris": ["https://example.com/callback"],
            "confidential_client": true,
            "created": "2024-01-15T10:00:00Z"
        })
    }

    #[tokio::test]
    async fn test_list_applications() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/applications/oauth2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(vec![oauth2_json(1, "My App")]))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (apps, resp) = client
            .oauth2()
            .list_applications(Default::default())
            .await
            .unwrap();
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].name, "My App");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_and_delete_application() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/applications/oauth2"))
            .respond_with(ResponseTemplate::new(201).set_body_json(oauth2_json(2, "New App")))
            .mount(&server)
            .await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/applications/oauth2/2"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);

        let opt = CreateOauth2Option {
            name: "New App".to_string(),
            confidential_client: true,
            redirect_uris: vec!["https://example.com/callback".to_string()],
        };
        let (app, resp) = client.oauth2().create_application(opt).await.unwrap();
        assert_eq!(app.id, 2);
        assert_eq!(resp.status, 201);

        let resp = client.oauth2().delete_application(2).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_list_applications_empty() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/applications/oauth2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(Vec::<serde_json::Value>::new()))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (apps, resp) = client
            .oauth2()
            .list_applications(Default::default())
            .await
            .unwrap();
        assert!(apps.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_applications_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/applications/oauth2"))
            .respond_with(ResponseTemplate::new(500).set_body_json(serde_json::json!({
                "message": "Internal Server Error"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.oauth2().list_applications(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_application_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/user/applications/oauth2"))
            .respond_with(ResponseTemplate::new(422).set_body_json(serde_json::json!({
                "message": "Validation failed"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateOauth2Option {
            name: "Valid App".to_string(),
            confidential_client: false,
            redirect_uris: vec![],
        };
        let result = client.oauth2().create_application(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_application() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/applications/oauth2/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(oauth2_json(42, "Fetched App")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (app, resp) = client.oauth2().get_application(42).await.unwrap();
        assert_eq!(app.id, 42);
        assert_eq!(app.name, "Fetched App");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_application_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/applications/oauth2/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.oauth2().get_application(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_application() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/applications/oauth2/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(oauth2_json(5, "Updated App")))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateOauth2Option {
            name: "Updated App".to_string(),
            confidential_client: true,
            redirect_uris: vec!["https://example.com/new-callback".to_string()],
        };
        let (app, resp) = client.oauth2().update_application(5, opt).await.unwrap();
        assert_eq!(app.id, 5);
        assert_eq!(app.name, "Updated App");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_application_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/user/applications/oauth2/5"))
            .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
                "message": "Forbidden"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateOauth2Option {
            name: "Updated App".to_string(),
            confidential_client: true,
            redirect_uris: vec![],
        };
        let result = client.oauth2().update_application(5, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_application_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/applications/oauth2/99"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.oauth2().delete_application(99).await;
        assert!(result.is_err());
    }
}
