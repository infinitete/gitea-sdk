// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::options::user::ListUserActivityFeedsOptions;
use crate::pagination::QueryEncode;
use crate::types::{Activity, UserHeatmapData};

use super::UsersApi;

impl<'a> UsersApi<'a> {
    // ── user_heatmap.go ───────────────────────────────────────────────────

    /// GetUserHeatmap gets a user's heatmap data.
    pub async fn get_user_heatmap(
        &self,
        username: &str,
    ) -> crate::Result<(Vec<UserHeatmapData>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let path = format!("/users/{}/heatmap", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── user_activity_feeds.go ───────────────────────────────────────────

    /// ListUserActivityFeeds lists a user's activity feeds.
    pub async fn list_user_activity_feeds(
        &self,
        username: &str,
        opt: ListUserActivityFeedsOptions,
    ) -> crate::Result<(Vec<Activity>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[username])?;
        let mut path = format!("/users/{}/activities/feeds", escaped[0]);
        let query = opt.query_encode();
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query);
        }
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::api::users::test_helpers::create_test_client;

    #[tokio::test]
    async fn test_get_user_heatmap() {
        let server = MockServer::start().await;
        let heatmap = serde_json::json!([
            {"timestamp": 1, "contributions": 3},
            {"timestamp": 2, "contributions": 0}
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/heatmap"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&heatmap))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (data, resp) = client.users().get_user_heatmap("testuser").await.unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].contributions, 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_activity_feeds() {
        let server = MockServer::start().await;
        let activities = serde_json::json!([{
            "id": 1,
            "act_user_id": 2,
            "act_user": null,
            "op_type": "create_repo",
            "content": "",
            "repo_id": 3,
            "repo": null,
            "comment_id": 0,
            "comment": null,
            "ref_name": "",
            "is_private": false,
            "user_id": 2,
            "created": "2024-01-15T10:00:00Z"
        }]);

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/activities/feeds"))
            .and(query_param("only-performed-by", "true"))
            .and(query_param("date", "2024-01-15"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&activities))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = ListUserActivityFeedsOptions {
            list_options: Default::default(),
            only_performed_by: true,
            date: "2024-01-15".to_string(),
        };
        let (list, resp) = client
            .users()
            .list_user_activity_feeds("testuser", opt)
            .await
            .unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].op_type, "create_repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_activity_feeds_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/activities/feeds"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .users()
            .list_user_activity_feeds("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_user_activity_feeds_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client
            .users()
            .list_user_activity_feeds("", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_heatmap_error() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/heatmap"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.users().get_user_heatmap("testuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_heatmap_validation() {
        let client = create_test_client(&MockServer::start().await);
        let result = client.users().get_user_heatmap("").await;
        assert!(result.is_err());
    }
}
