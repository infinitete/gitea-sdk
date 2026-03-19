// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::User;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_watch.go (5 methods) ─────────────────────────────────

    /// GetWatchedRepos list all the watched repos of user
    pub async fn get_watched_repos(
        &self,
        user: &str,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/subscriptions", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetMyWatchedRepos list repositories watched by the authenticated user
    pub async fn get_my_watched_repos(&self) -> crate::Result<(Vec<Repository>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/user/subscriptions",
                None,
                None::<&str>,
            )
            .await
    }

    /// CheckRepoWatch check if the current user is watching a repo
    pub async fn check_repo_watch(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/subscription", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        match status {
            200 => Ok((true, resp)),
            404 => Ok((false, resp)),
            _ => Err(crate::Error::Api {
                status,
                message: format!("unexpected status: {status}"),
                body: vec![],
            }),
        }
    }

    /// WatchRepo start to watch a repository
    pub async fn watch_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/subscription", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::PUT, &path, None, None::<&str>)
            .await?;
        if status == 200 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status: {status}"),
                body: vec![],
            })
        }
    }

    /// UnWatchRepo stop to watch a repository
    pub async fn unwatch_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/subscription", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await?;
        if status == 204 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status: {status}"),
                body: vec![],
            })
        }
    }

    // ── repo_stars.go (6 methods) ─────────────────────────────────

    /// ListStargazers list a repository's stargazers
    pub async fn list_stargazers(
        &self,
        owner: &str,
        repo: &str,
        opt: ListStargazersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/stargazers?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetStarredRepos list repos starred by a given user
    pub async fn get_starred_repos(
        &self,
        user: &str,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/starred", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetMyStarredRepos list repos starred by the authenticated user
    pub async fn get_my_starred_repos(&self) -> crate::Result<(Vec<Repository>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/user/starred",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// IsRepoStarring check if the authenticated user has starred the repo
    pub async fn is_repo_starring(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/user/starred/{}/{}", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        match status {
            204 => Ok((true, resp)),
            404 => Ok((false, resp)),
            _ => Err(crate::Error::Api {
                status,
                message: format!("unexpected status code '{status}'"),
                body: vec![],
            }),
        }
    }

    /// StarRepo star a repository as the authenticated user
    pub async fn star_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/user/starred/{}/{}", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status == 204 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status code '{status}'"),
                body: vec![],
            })
        }
    }

    /// UnstarRepo remove star from a repository as the authenticated user
    pub async fn unstar_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/user/starred/{}/{}", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status == 204 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status code '{status}'"),
                body: vec![],
            })
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
    async fn test_star_repo() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/user/starred/testowner/testrepo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .repos()
            .star_repo("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_error_case() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/nonrepo"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({"message": "Repository not found"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.repos().get_repo("testowner", "nonrepo").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Repository not found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
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
    async fn test_get_watched_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/subscriptions"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_watched_repos("testuser").await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_watched_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/subscriptions"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_watched_repos("testuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_my_watched_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/subscriptions"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_my_watched_repos().await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_my_watched_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/subscriptions"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_my_watched_repos().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_repo_watch_true() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (watching, resp) = client
            .repos()
            .check_repo_watch("owner", "repo")
            .await
            .unwrap();
        assert!(watching);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_check_repo_watch_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (watching, resp) = client
            .repos()
            .check_repo_watch("owner", "repo")
            .await
            .unwrap();
        assert!(!watching);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_watch_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().watch_repo("owner", "repo").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[tokio::test]
    async fn test_watch_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().watch_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unwatch_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unwatch_repo("owner", "repo").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_unwatch_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unwatch_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_stargazers_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/stargazers"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_user_json(1, "stargazer1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (stargazers, resp) = client
            .repos()
            .list_stargazers("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(stargazers.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_stargazers_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/stargazers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_stargazers("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_starred_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/starred"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_starred_repos("testuser").await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_starred_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/starred"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_starred_repos("testuser").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_my_starred_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_my_starred_repos().await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_my_starred_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_my_starred_repos().await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_repo_starring_true() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (starring, resp) = client
            .repos()
            .is_repo_starring("owner", "repo")
            .await
            .unwrap();
        assert!(starring);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_is_repo_starring_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (starring, resp) = client
            .repos()
            .is_repo_starring("owner", "repo")
            .await
            .unwrap();
        assert!(!starring);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_star_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().star_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unstar_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unstar_repo("owner", "repo").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_unstar_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unstar_repo("owner", "repo").await;
        assert!(result.is_err());
    }
}
