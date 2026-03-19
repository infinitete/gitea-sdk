// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::admin::{ListAdminEmailsOptions, SearchAdminEmailsOptions};
use crate::pagination::QueryEncode;
use crate::types::Email;

use super::AdminApi;

impl<'a> AdminApi<'a> {
    // ── admin_email.go ───────────────────────────────────────────────

    /// List all email addresses
    pub async fn list_emails(
        &self,
        opt: ListAdminEmailsOptions,
    ) -> crate::Result<(Vec<Email>, Response)> {
        let path = format!("/admin/emails?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// Search email addresses
    pub async fn search_emails(
        &self,
        opt: SearchAdminEmailsOptions,
    ) -> crate::Result<(Vec<Email>, Response)> {
        let path = format!("/admin/emails/search?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
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
    use crate::options::admin::SearchAdminEmailsOptions;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, email_json};

    #[tokio::test]
    async fn test_list_emails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                email_json("a@example.com"),
                email_json("b@example.com")
            ])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (emails, resp) = client
            .admin()
            .list_emails(Default::default())
            .await
            .unwrap();
        assert_eq!(emails.len(), 2);
        assert_eq!(emails[0].email, "a@example.com");
        assert_eq!(emails[1].email, "b@example.com");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_emails_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_emails(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_emails() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([email_json("search@example.com")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchAdminEmailsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let (emails, resp) = client.admin().search_emails(opt).await.unwrap();
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].email, "search@example.com");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_emails_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/emails/search"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().search_emails(Default::default()).await;
        assert!(result.is_err());
    }
}
