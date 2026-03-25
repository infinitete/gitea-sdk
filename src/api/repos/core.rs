// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo.go (16 methods) ─────────────────────────────────────

    /// `ListMyRepos` list all repositories of the authenticated user
    pub async fn list_my_repos(
        &self,
        opt: ListReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let path = format!("/user/repos?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `ListUserRepos` list repositories of a user
    pub async fn list_user_repos(
        &self,
        user: &str,
        opt: ListReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/repos?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateRepo` create a repository
    pub async fn create_repo(
        &self,
        opt: CreateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/repos",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `GetRepo` get a repository
    pub async fn get_repo(&self, owner: &str, repo: &str) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetRepoByID` get a repository by id
    pub async fn get_repo_by_id(&self, id: i64) -> crate::Result<(Repository, Response)> {
        let path = format!("/repositories/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `EditRepo` edit repository properties
    pub async fn edit_repo(
        &self,
        owner: &str,
        repo: &str,
        opt: EditRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteRepo` delete a repository
    pub async fn delete_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}", escaped[0], escaped[1]);
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_repo() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_repo_json(
                1,
                "testrepo",
                "testowner",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (repo, resp) = client
            .repos()
            .get_repo("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "testrepo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_repo() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/repos"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(minimal_repo_json(2, "newrepo", "testuser")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: crate::types::enums::TrustModel::Default,
            object_format_name: String::new(),
        };
        let (repo, resp) = client.repos().create_repo(opt).await.unwrap();
        assert_eq!(repo.id, 2);
        assert_eq!(repo.name, "newrepo");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_list_my_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/repos"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .list_my_repos(Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "repo1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/repos"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().list_my_repos(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_user_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/someuser/repos"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "someuser"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .list_user_repos("someuser", Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/someuser/repos"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_user_repos("someuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/user/repos"))
            .respond_with(ResponseTemplate::new(422).set_body_json(json!({"message": "Invalid"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: crate::types::enums::TrustModel::Default,
            object_format_name: String::new(),
        };
        let result = client.repos().create_repo(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_repo_json(
                1,
                "testrepo",
                "testowner",
            )))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repo, resp) = client
            .repos()
            .get_repo("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(repo.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/nonrepo"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_repo("testowner", "nonrepo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_by_id_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repositories/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_repo_json(
                42,
                "some-repo",
                "owner",
            )))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repo, resp) = client.repos().get_repo_by_id(42).await.unwrap();
        assert_eq!(repo.id, 42);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_by_id_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repositories/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_repo_by_id(999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "owner")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditRepoOption {
            description: Some("new desc".to_string()),
            name: None,
            website: None,
            private: None,
            template: None,
            has_issues: None,
            internal_tracker: None,
            external_tracker: None,
            has_wiki: None,
            external_wiki: None,
            default_branch: None,
            has_pull_requests: None,
            has_projects: None,
            has_releases: None,
            has_packages: None,
            has_actions: None,
            ignore_whitespace_conflicts: None,
            allow_fast_forward_only_merge: None,
            allow_merge: None,
            allow_rebase: None,
            allow_rebase_merge: None,
            allow_squash: None,
            archived: None,
            mirror_interval: None,
            allow_manual_merge: None,
            autodetect_manual_merge: None,
            default_merge_style: None,
            projects_mode: None,
            default_delete_branch_after_merge: None,
        };
        let (repo, resp) = client
            .repos()
            .edit_repo("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(repo.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditRepoOption {
            description: Some("new desc".to_string()),
            name: None,
            website: None,
            private: None,
            template: None,
            has_issues: None,
            internal_tracker: None,
            external_tracker: None,
            has_wiki: None,
            external_wiki: None,
            default_branch: None,
            has_pull_requests: None,
            has_projects: None,
            has_releases: None,
            has_packages: None,
            has_actions: None,
            ignore_whitespace_conflicts: None,
            allow_fast_forward_only_merge: None,
            allow_merge: None,
            allow_rebase: None,
            allow_rebase_merge: None,
            allow_squash: None,
            archived: None,
            mirror_interval: None,
            allow_manual_merge: None,
            autodetect_manual_merge: None,
            default_merge_style: None,
            projects_mode: None,
            default_delete_branch_after_merge: None,
        };
        let result = client.repos().edit_repo("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.repos().delete_repo("owner", "repo").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_repo("owner", "repo").await;
        assert!(result.is_err());
    }
}
