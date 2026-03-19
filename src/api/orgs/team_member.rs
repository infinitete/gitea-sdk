// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::User;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    /// ListTeamMembers lists all members of a team
    pub async fn list_team_members(
        &self,
        id: i64,
        opt: ListTeamMembersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/teams/{}/members?{}", id, opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTeamMember gets a member of a team
    pub async fn get_team_member(&self, id: i64, user: &str) -> crate::Result<(User, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/teams/{}/members/{}", id, escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddTeamMember adds a member to a team
    pub async fn add_team_member(&self, id: i64, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/teams/{}/members/{}", id, escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// RemoveTeamMember removes a member from a team
    pub async fn remove_team_member(&self, id: i64, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/teams/{}/members/{}", id, escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_team_members ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_team_members_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "user1"), user_json(2, "user2")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (members, resp) = client
            .orgs()
            .list_team_members(5, Default::default())
            .await
            .unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].user_name, "user1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_team_members_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_team_members(5, Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_team_member ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_team_member_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json(1, "testuser")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (user, resp) = client.orgs().get_team_member(5, "testuser").await.unwrap();
        assert_eq!(user.user_name, "testuser");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_team_member_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_team_member(5, "testuser").await;
        assert!(result.is_err());
    }

    // ── add_team_member ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_add_team_member_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().add_team_member(5, "testuser").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_team_member_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().add_team_member(5, "testuser").await;
        assert!(result.is_err());
    }

    // ── remove_team_member ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_remove_team_member_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .remove_team_member(5, "testuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_remove_team_member_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().remove_team_member(5, "testuser").await;
        assert!(result.is_err());
    }
}
