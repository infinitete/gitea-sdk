// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;

pub struct ActivityPubApi<'a> {
    client: &'a Client,
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

impl<'a> ActivityPubApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    pub async fn get_person(&self, user_id: i64) -> crate::Result<(serde_json::Value, Response)> {
        let path = format!("/activitypub/user-id/{user_id}");
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    pub async fn get_repository(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(serde_json::Value, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/activitypub/{}/{}/repository", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    pub async fn get_followers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(serde_json::Value, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/activitypub/{}/{}/followers", escaped[0], escaped[1]);
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

    #[tokio::test]
    async fn test_get_person() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/activitypub/user-id/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "@context": "https://www.w3.org/ns/activitystreams",
                "type": "Person",
                "id": "https://gitea.example.com/api/v1/activitypub/user-id/1",
                "preferredUsername": "testuser"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (person, resp) = client.activitypub().get_person(1).await.unwrap();
        assert_eq!(person["type"], "Person");
        assert_eq!(person["preferredUsername"], "testuser");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_person_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/activitypub/user-id/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "Not Found"
            })))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.activitypub().get_person(1).await;
        assert!(result.is_err());
    }
}
