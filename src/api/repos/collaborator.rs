// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::User;
use crate::types::repository::*;
use crate::version::VERSION_1_15_0;

impl<'a> super::ReposApi<'a> {
    // ── repo_collaborator.go (7 methods) ──────────────────────────

    /// ListCollaborators list a repository's collaborators
    pub async fn list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        opt: ListCollaboratorsOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/collaborators?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// IsCollaborator check if a user is a collaborator of a repository
    pub async fn is_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        Ok((status == 204, resp))
    }

    /// GetCollaboratorPermission get collaborator permission of a repository
    pub async fn get_collaborator_permission(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::Result<(Option<CollaboratorPermissionResult>, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}/permission",
            escaped[0], escaped[1], escaped[2]
        );
        match self
            .client()
            .get_parsed_response::<CollaboratorPermissionResult, _>(
                reqwest::Method::GET,
                &path,
                None,
                None::<&str>,
            )
            .await
        {
            Ok((result, resp)) => Ok((Some(result), resp)),
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

    /// AddCollaborator add a user as a collaborator of a repository
    pub async fn add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
        mut opt: AddCollaboratorOption,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteCollaborator remove a collaborator from a repository
    pub async fn delete_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// GetReviewers get all users that can be requested to review in this repo
    pub async fn get_reviewers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<User>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/reviewers", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetAssignees get all users that have write access and can be assigned to issues
    pub async fn get_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<User>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/assignees", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_is_collaborator_true() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/collaborators/testuser",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (is_collab, resp) = client
            .repos()
            .is_collaborator("testowner", "testrepo", "testuser")
            .await
            .unwrap();
        assert!(is_collab);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_is_collaborator_false() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/collaborators/nonuser",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (is_collab, resp) = client
            .repos()
            .is_collaborator("testowner", "testrepo", "nonuser")
            .await
            .unwrap();
        assert!(!is_collab);
        assert_eq!(resp.status, 404);
    }

    fn minimal_user_json(id: i64, login: &str) -> serde_json::Value {
        json!({
            "id": id,
            "login": login,
            "login_name": "",
            "source_id": 0,
            "full_name": login,
            "email": "",
            "avatar_url": "",
            "html_url": "",
            "language": "",
            "is_admin": false,
            "restricted": false,
            "active": false,
            "prohibit_login": false,
            "location": "",
            "website": "",
            "description": "",
            "visibility": "public",
            "followers_count": 0,
            "following_count": 0,
            "starred_repos_count": 0,
        })
    }

    #[tokio::test]
    async fn test_list_collaborators_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/collaborators"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_user_json(1, "alice"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (collabs, resp) = client
            .repos()
            .list_collaborators("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(collabs.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_collaborators_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/collaborators"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_collaborators("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_collaborator_permission_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "permission": "write",
            "role_name": "Write",
            "user": minimal_user_json(1, "alice")
        });
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/owner/repo/collaborators/alice/permission",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (perm, resp) = client
            .repos()
            .get_collaborator_permission("owner", "repo", "alice")
            .await
            .unwrap();
        assert!(perm.is_some());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_collaborator_permission_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/owner/repo/collaborators/nobody/permission",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (perm, resp) = client
            .repos()
            .get_collaborator_permission("owner", "repo", "nobody")
            .await
            .unwrap();
        assert!(perm.is_none());
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_add_collaborator_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/collaborators/alice"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let mut opt = AddCollaboratorOption {
            permission: Some(crate::types::enums::AccessMode::Write),
        };
        opt.validate().unwrap();
        let resp = client
            .repos()
            .add_collaborator("owner", "repo", "alice", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_collaborator_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/collaborators/alice"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let mut opt = AddCollaboratorOption {
            permission: Some(crate::types::enums::AccessMode::Write),
        };
        opt.validate().unwrap();
        let result = client
            .repos()
            .add_collaborator("owner", "repo", "alice", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_collaborator_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/collaborators/user1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_collaborator("owner", "repo", "user1")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_collaborator_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/collaborators/user1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_collaborator("owner", "repo", "user1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_reviewers_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/reviewers"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_user_json(1, "reviewer1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reviewers, resp) = client.repos().get_reviewers("owner", "repo").await.unwrap();
        assert_eq!(reviewers.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_reviewers_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/reviewers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_reviewers("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_assignees_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/assignees"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_user_json(1, "assignee1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (assignees, resp) = client.repos().get_assignees("owner", "repo").await.unwrap();
        assert_eq!(assignees.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_assignees_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/assignees"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_assignees("owner", "repo").await;
        assert!(result.is_err());
    }
}
