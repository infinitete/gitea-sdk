// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::query::build_query_string;
use crate::options::user::SearchUsersOption;
use crate::pagination::QueryEncode;
use crate::types::User;

use super::UsersApi;

#[derive(serde::Deserialize)]
pub(crate) struct SearchUsersResponse {
    #[serde(default)]
    data: Vec<User>,
}

impl<'a> UsersApi<'a> {
    // ── user_search.go ─────────────────────────────────────────────────

    /// `SearchUsers` finds users by query
    pub async fn search(&self, opt: SearchUsersOption) -> crate::Result<(Vec<User>, Response)> {
        let query = opt.query_encode();
        let path = build_query_string("/users/search", &query);
        let (search_resp, response) = self
            .client()
            .get_parsed_response::<SearchUsersResponse, _>(
                reqwest::Method::GET,
                &path,
                None,
                None::<&str>,
            )
            .await?;
        Ok((search_resp.data, response))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::{create_test_client, user_json};

    #[tokio::test]
    async fn test_search_users() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": true,
            "data": [user_json(1, "testuser")]
        });

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = SearchUsersOption {
            key_word: "testuser".to_string(),
            ..Default::default()
        };
        let (users, resp) = client.users().search(opt).await.unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_name, "testuser");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_users_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/search"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().search(SearchUsersOption::default()).await;
        assert!(result.is_err());
    }
}
