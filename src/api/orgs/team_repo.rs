// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::options::org::*;
use crate::pagination::QueryEncode;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    /// ListTeamRepositories lists all repositories of a team
    pub async fn list_team_repositories(
        &self,
        id: i64,
        opt: ListTeamRepositoriesOptions,
    ) -> crate::Result<(Vec<crate::types::repository::Repository>, Response)> {
        let path = format!("/teams/{}/repos?{}", id, opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
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
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── list_team_repositories ───────────────────────────────────────────

    #[tokio::test]
    async fn test_list_team_repositories_happy() {
        let server = MockServer::start().await;
        let repo_json = make_minimal_repo_json();
        let body = serde_json::json!([repo_json]);
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/repos"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .orgs()
            .list_team_repositories(5, Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_team_repositories_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/repos"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_team_repositories(5, Default::default())
            .await;
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
