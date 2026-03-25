// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;
use crate::version::{VERSION_1_13_0, VERSION_1_23_0};

impl<'a> super::ReposApi<'a> {
    // ── repo_branch.go (5 methods) ────────────────────────────────

    /// `ListBranches` list a repository's branches
    pub async fn list_branches(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoBranchesOptions,
    ) -> crate::Result<(Vec<Branch>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/branches?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetBranch` get a single branch of a repository
    pub async fn get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::Result<(Branch, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, branch])?;
        let path = format!(
            "/repos/{}/{}/branches/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `DeleteBranch` delete a branch from a repository
    pub async fn delete_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, branch])?;
        let path = format!(
            "/repos/{}/{}/branches/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `UpdateBranch` rename a branch in a repository
    pub async fn update_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        opt: UpdateRepoBranchOption,
    ) -> crate::Result<(Branch, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_23_0)
            .await?;
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, branch])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/branches/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;

        if data.is_empty() {
            let (updated, _) = self.get_branch(owner, repo, &opt.name).await?;
            return Ok((updated, resp));
        }

        let updated: Branch = serde_json::from_slice(&data)?;
        Ok((updated, resp))
    }

    /// `CreateBranch` create a branch in a repository
    pub async fn create_branch(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateBranchOption,
    ) -> crate::Result<(Branch, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_13_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/branches", escaped[0], escaped[1]);
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
    use crate::options::repo::{CreateBranchOption, UpdateRepoBranchOption};

    use crate::Client;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_list_branches() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "name": "main",
                "protected": false,
                "required_approvals": 0,
                "enable_status_check": false,
                "status_check_contexts": [],
                "user_can_push": true,
                "user_can_merge": true,
                "effective_branch_protection_name": ""
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/branches"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (branches, resp) = client
            .repos()
            .list_branches("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_branch_version_gated() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "1.12.0"
            })))
            .mount(&server)
            .await;

        let client = Client::builder(&server.uri())
            .token("test-token")
            .build()
            .unwrap();
        let result = client
            .repos()
            .create_branch(
                "testowner",
                "testrepo",
                CreateBranchOption {
                    branch_name: "feature".to_string(),
                    old_branch_name: "main".to_string(),
                },
            )
            .await;

        assert!(matches!(result, Err(crate::Error::Version { .. })));
    }

    #[tokio::test]
    async fn test_list_branches_happy() {
        let server = MockServer::start().await;
        let body = json!([{
            "name": "main",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        }]);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branches, resp) = client
            .repos()
            .list_branches("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_branches_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_branches("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_branch_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "develop",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches/develop"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branch, resp) = client
            .repos()
            .get_branch("owner", "repo", "develop")
            .await
            .unwrap();
        assert_eq!(branch.name, "develop");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_branch_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches/nonexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_branch("owner", "repo", "nonexist").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_branch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/branches/old%2Dbranch"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .repos()
            .delete_branch("owner", "repo", "old-branch")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_branch_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/branches/old-branch"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_branch("owner", "repo", "old-branch")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_branch_happy() {
        let server = MockServer::start().await;
        // version check
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.23.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "name": "new-name",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        });
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branches/old%2Dname"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branch, resp) = client
            .repos()
            .update_branch(
                "owner",
                "repo",
                "old-name",
                UpdateRepoBranchOption {
                    name: "new-name".to_string(),
                },
            )
            .await
            .unwrap();
        assert_eq!(branch.name, "new-name");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_branch_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.23.0"})))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branches/old-name"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .update_branch(
                "owner",
                "repo",
                "old-name",
                UpdateRepoBranchOption {
                    name: "new-name".to_string(),
                },
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_branch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "name": "feature",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        });
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/branches"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branch, resp) = client
            .repos()
            .create_branch(
                "owner",
                "repo",
                CreateBranchOption {
                    branch_name: "feature".to_string(),
                    old_branch_name: "main".to_string(),
                },
            )
            .await
            .unwrap();
        assert_eq!(branch.name, "feature");
        assert_eq!(resp.status, 201);
    }
}
