// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::{Team, User};
use crate::{Deserialize, Serialize};

use super::OrgsApi;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeamSearchResults {
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    error: String,
    #[serde(default)]
    data: Vec<Team>,
}

impl<'a> OrgsApi<'a> {
    // ── org_team.go ───────────────────────────────────────────────────────

    /// ListOrgTeams lists all teams of an organization
    pub async fn list_org_teams(
        &self,
        org: &str,
        opt: ListTeamsOptions,
    ) -> crate::Result<(Vec<Team>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/teams?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyTeams lists all the teams of the current user
    pub async fn list_my_teams(
        &self,
        opt: ListTeamsOptions,
    ) -> crate::Result<(Vec<Team>, Response)> {
        let path = format!("/user/teams?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTeam gets a team by ID
    pub async fn get_team(&self, id: i64) -> crate::Result<(Team, Response)> {
        let path = format!("/teams/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// SearchOrgTeams search for teams in an org
    pub async fn search_org_teams(
        &self,
        org: &str,
        opt: SearchTeamsOptions,
    ) -> crate::Result<(Vec<Team>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/teams/search?{}", escaped[0], opt.query_encode());
        let (result, response) = self
            .client()
            .get_parsed_response::<TeamSearchResults, _>(
                reqwest::Method::GET,
                &path,
                None,
                None::<&str>,
            )
            .await?;
        if !result.ok {
            return Err(crate::Error::UnknownApi {
                status: response.status,
                body: result.error,
            });
        }
        Ok((result.data, response))
    }

    /// CreateTeam creates a team for an organization
    pub async fn create_team(
        &self,
        org: &str,
        opt: CreateTeamOption,
    ) -> crate::Result<(Team, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/teams", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditTeam edits a team of an organization
    pub async fn edit_team(&self, id: i64, opt: EditTeamOption) -> crate::Result<Response> {
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/teams/{id}");
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteTeam deletes a team of an organization
    pub async fn delete_team(&self, id: i64) -> crate::Result<Response> {
        let path = format!("/teams/{id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

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

    /// AddTeamRepository adds a repository to a team
    pub async fn add_team_repo(&self, id: i64, org: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, repo])?;
        let path = format!("/teams/{}/repos/{}/{}", id, escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// RemoveTeamRepository removes a repository from a team
    pub async fn remove_team_repo(
        &self,
        id: i64,
        org: &str,
        repo: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, repo])?;
        let path = format!("/teams/{}/repos/{}/{}", id, escaped[0], escaped[1]);
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
    use wiremock::matchers::method;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_org_teams ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_teams_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([team_json(1, "owners"), team_json(2, "devs")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (teams, resp) = client
            .orgs()
            .list_org_teams("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(teams.len(), 2);
        assert_eq!(teams[0].name, "owners");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_teams_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_teams("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_my_teams ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_teams_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([team_json(1, "myteam")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/user/teams"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (teams, resp) = client
            .orgs()
            .list_my_teams(Default::default())
            .await
            .unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "myteam");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_teams_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/teams"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_my_teams(Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_team ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(team_json(5, "devs")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (team, resp) = client.orgs().get_team(5).await.unwrap();
        assert_eq!(team.id, 5);
        assert_eq!(team.name, "devs");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_team(999).await;
        assert!(result.is_err());
    }

    // ── search_org_teams ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_search_org_teams_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": true,
            "error": "",
            "data": [team_json(1, "search-team")]
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchTeamsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let (teams, resp) = client
            .orgs()
            .search_org_teams("testorg", opt)
            .await
            .unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "search-team");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_org_teams_error_not_ok() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": false,
            "error": "search failed",
            "data": []
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchTeamsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let result = client.orgs().search_org_teams("testorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_org_teams_error_http() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams/search"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(json!({"message": "Internal Server Error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchTeamsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let result = client.orgs().search_org_teams("testorg", opt).await;
        assert!(result.is_err());
    }

    // ── create_team ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(201).set_body_json(team_json(10, "newteam")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateTeamOption {
            name: "newteam".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let (team, resp) = client.orgs().create_team("testorg", opt).await.unwrap();
        assert_eq!(team.name, "newteam");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateTeamOption {
            name: "newteam".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let result = client.orgs().create_team("testorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_team_validation_empty_name() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateTeamOption {
            name: String::new(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let result = client.orgs().create_team("testorg", opt).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name required"));
    }

    // ── edit_team ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditTeamOption {
            name: "renamed".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let resp = client.orgs().edit_team(5, opt).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditTeamOption {
            name: "renamed".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let result = client.orgs().edit_team(5, opt).await;
        assert!(result.is_err());
    }

    // ── delete_team ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_team(5).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_team(5).await;
        assert!(result.is_err());
    }

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

    // ── add_team_repo ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_add_team_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .add_team_repo(5, "myorg", "myrepo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_team_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().add_team_repo(5, "myorg", "myrepo").await;
        assert!(result.is_err());
    }

    // ── remove_team_repo ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_remove_team_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .remove_team_repo(5, "myorg", "myrepo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_remove_team_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().remove_team_repo(5, "myorg", "myrepo").await;
        assert!(result.is_err());
    }
}
