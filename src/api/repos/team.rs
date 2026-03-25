// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::types::Team;
use crate::version::VERSION_1_15_0;

impl<'a> super::ReposApi<'a> {
    // ── repo_team.go (4 methods) ──────────────────────────────────

    /// `GetRepoTeams` get teams from a repository
    pub async fn get_repo_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<Team>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/teams", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `AddRepoTeam` add a team to a repository
    pub async fn add_repo_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::Result<Response> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, team])?;
        let path = format!("/repos/{}/{}/teams/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// `RemoveRepoTeam` remove a team from a repository
    pub async fn remove_repo_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::Result<Response> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, team])?;
        let path = format!("/repos/{}/{}/teams/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `CheckRepoTeam` check if a team is assigned to a repository
    pub async fn check_repo_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::Result<(Option<Team>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, team])?;
        let path = format!("/repos/{}/{}/teams/{}", escaped[0], escaped[1], escaped[2]);
        match self
            .client()
            .get_parsed_response::<Team, _>(reqwest::Method::GET, &path, None, None::<&str>)
            .await
        {
            Ok((t, resp)) => Ok((Some(t), resp)),
            Err(e) => {
                if let crate::Error::Api { status, .. } = &e
                    && *status == 404
                {
                    return Ok((
                        None,
                        Response {
                            status: *status,
                            headers: reqwest::header::HeaderMap::new(),
                            page_links: None,
                        },
                    ));
                }
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_repo_teams_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([minimal_team_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client.repos().get_repo_teams("owner", "repo").await;
        assert!(result.is_ok());
        let (teams, resp) = result.unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "developers");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_teams_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_repo_teams("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_repo_team_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .add_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_add_repo_team_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .add_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_repo_team_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .remove_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_remove_repo_team_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .remove_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_repo_team_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_team_json(1)))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .check_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        let (team, resp) = result.unwrap();
        assert!(team.is_some());
        assert_eq!(team.unwrap().name, "developers");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_check_repo_team_not_found() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .check_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        let (team, resp) = result.unwrap();
        assert!(team.is_none());
        assert_eq!(resp.status, 404);
    }
}
