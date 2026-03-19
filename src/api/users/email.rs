// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::user::*;
use crate::pagination::QueryEncode;
use crate::types::Email;

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_email.go ──────────────────────────────────────────────────

    /// ListEmails all the email addresses of user
    pub async fn list_emails(
        &self,
        opt: ListEmailsOptions,
    ) -> crate::Result<(Vec<Email>, Response)> {
        let path = format!("/user/emails?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddEmail add one email to current user with options
    pub async fn add_email(&self, opt: CreateEmailOption) -> crate::Result<(Vec<Email>, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/emails",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteEmail delete one email of current users
    pub async fn delete_email(&self, opt: DeleteEmailOption) -> crate::Result<Response> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                "/user/emails",
                Some(&json_header()),
                Some(body),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::create_test_client;

    #[tokio::test]
    async fn test_list_emails() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"email": "test@example.com", "verified": true, "primary": true},
            {"email": "alt@example.com", "verified": false, "primary": false}
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (emails, resp) = client
            .users()
            .list_emails(Default::default())
            .await
            .unwrap();
        assert_eq!(emails.len(), 2);
        assert_eq!(emails[0].email, "test@example.com");
        assert!(emails[0].verified);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_add_email_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {"email": "new@example.com", "verified": false, "primary": false}
        ]);

        Mock::given(method("POST"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateEmailOption {
            emails: vec!["new@example.com".to_string()],
        };
        let (emails, resp) = client.users().add_email(opt).await.unwrap();
        assert_eq!(emails.len(), 1);
        assert_eq!(emails[0].email, "new@example.com");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_add_email_validation_empty() {
        let client = create_test_client(&MockServer::start().await);
        let opt = CreateEmailOption { emails: vec![] };
        let result = client.users().add_email(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_email_error() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/emails"))
            .respond_with(
                ResponseTemplate::new(422)
                    .set_body_json(serde_json::json!({"message": "invalid email"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateEmailOption {
            emails: vec!["bad-email".to_string()],
        };
        let result = client.users().add_email(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_email_happy() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = DeleteEmailOption {
            emails: vec!["old@example.com".to_string()],
        };
        let resp = client.users().delete_email(opt).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_email_validation_empty() {
        let client = create_test_client(&MockServer::start().await);
        let opt = DeleteEmailOption { emails: vec![] };
        let result = client.users().delete_email(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_email_error() {
        let server = MockServer::start().await;

        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = DeleteEmailOption {
            emails: vec!["nonexistent@example.com".to_string()],
        };
        let result = client.users().delete_email(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_emails_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/user/emails"))
            .respond_with(ResponseTemplate::new(401))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().list_emails(Default::default()).await;
        assert!(result.is_err());
    }
}
