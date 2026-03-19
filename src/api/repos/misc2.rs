// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_tree.go (1 method) ───────────────────────────────────

    /// GetTrees get a git tree of a repository
    pub async fn get_trees(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        opt: ListTreeOptions,
    ) -> crate::Result<(GitTreeResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/git/trees/{}?{}",
            escaped[0],
            escaped[1],
            sha,
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_migrate.go (1 method) ────────────────────────────────

    /// MigrateRepo migrate a repository from an external service
    pub async fn migrate_repo(
        &self,
        opt: MigrateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/repos/migrate",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── repo_transfer.go (3 methods) ──────────────────────────────

    /// TransferRepo transfer a repository to a new owner
    pub async fn transfer_repo(
        &self,
        owner: &str,
        repo: &str,
        opt: TransferRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/transfer", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// AcceptRepoTransfer accept a repository transfer
    pub async fn accept_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/transfer/accept", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// RejectRepoTransfer reject a repository transfer
    pub async fn reject_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/transfer/reject", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    // ── fork.go (2 methods) ───────────────────────────────────────

    /// ListForks list repository's forks
    pub async fn list_forks(
        &self,
        owner: &str,
        repo: &str,
        opt: ListForksOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/forks?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateFork create a fork of a repository
    pub async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateForkOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/forks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use crate::options::repo::{CreateForkOption, MigrateRepoOption, TransferRepoOption};
    use serde_json::json;

    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_trees_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/trees/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "sha": "abc123",
                "url": "https://gitea.example.com/api/v1/repos/owner/repo/git/trees/abc123",
                "tree": [
                    {
                        "path": "README.md",
                        "mode": "100644",
                        "type": "blob",
                        "size": 10,
                        "sha": "def456",
                        "url": ""
                    }
                ],
                "truncated": false,
                "page": 1,
                "total_count": 1
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tree, resp) = client
            .repos()
            .get_trees("owner", "repo", "main", Default::default())
            .await
            .unwrap();
        assert_eq!(tree.sha, "abc123");
        assert_eq!(tree.tree.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_trees_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/trees/main"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_trees("owner", "repo", "main", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_migrate_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/migrate"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(minimal_repo_json(10, "migrated", "owner")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = MigrateRepoOption {
            repo_name: "migrated".to_string(),
            repo_owner: "owner".to_string(),
            uid: 0,
            clone_addr: "https://github.com/example/repo.git".to_string(),
            service: crate::types::enums::GitServiceType::Github,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: "ghp_test_token".to_string(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: true,
            milestones: true,
            labels: true,
            issues: true,
            pull_requests: true,
            releases: true,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        let (repo, resp) = client.repos().migrate_repo(opt).await.unwrap();
        assert_eq!(repo.id, 10);
        assert_eq!(repo.name, "migrated");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_migrate_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/migrate"))
            .respond_with(
                ResponseTemplate::new(409)
                    .set_body_json(json!({"message": "Repository already exists"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = MigrateRepoOption {
            repo_name: "migrated".to_string(),
            repo_owner: "owner".to_string(),
            uid: 0,
            clone_addr: "https://github.com/example/repo.git".to_string(),
            service: crate::types::enums::GitServiceType::Github,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: "ghp_test_token".to_string(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: true,
            milestones: true,
            labels: true,
            issues: true,
            pull_requests: true,
            releases: true,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        let result = client.repos().migrate_repo(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_transfer_repo_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "newowner")),
            )
            .mount(&server)
            .await;
        let opt = TransferRepoOption {
            new_owner: "newowner".to_string(),
            team_ids: None,
        };
        let result = client.repos().transfer_repo("owner", "repo", opt).await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_transfer_repo_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = TransferRepoOption {
            new_owner: "newowner".to_string(),
            team_ids: None,
        };
        let result = client.repos().transfer_repo("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_accept_repo_transfer_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/accept"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "owner")),
            )
            .mount(&server)
            .await;
        let result = client.repos().accept_repo_transfer("owner", "repo").await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_accept_repo_transfer_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/accept"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let result = client.repos().accept_repo_transfer("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reject_repo_transfer_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/reject"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "owner")),
            )
            .mount(&server)
            .await;
        let result = client.repos().reject_repo_transfer("owner", "repo").await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_reject_repo_transfer_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/reject"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let result = client.repos().reject_repo_transfer("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_forks_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                minimal_repo_json(2, "fork-repo", "user")
            ])))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_forks("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (forks, resp) = result.unwrap();
        assert_eq!(forks.len(), 1);
        assert_eq!(forks[0].name, "fork-repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_forks_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_forks("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_fork_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(202).set_body_json(minimal_repo_json(
                2,
                "fork-repo",
                "user",
            )))
            .mount(&server)
            .await;
        let opt = CreateForkOption {
            organization: None,
            name: Some("fork-repo".to_string()),
        };
        let result = client.repos().create_fork("owner", "repo", opt).await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "fork-repo");
        assert_eq!(resp.status, 202);
    }

    #[tokio::test]
    async fn test_create_fork_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateForkOption {
            organization: None,
            name: Some("fork-repo".to_string()),
        };
        let result = client.repos().create_fork("owner", "repo", opt).await;
        assert!(result.is_err());
    }
}
