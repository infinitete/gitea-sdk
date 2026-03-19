// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::admin::AdminListOrgsOptions;
use crate::options::org::CreateOrgOption;
use crate::options::repo::CreateRepoOption;
use crate::pagination::QueryEncode;
use crate::types::{Organization, Repository};

use super::AdminApi;

impl<'a> AdminApi<'a> {
    // ── admin_org.go ─────────────────────────────────────────────────

    /// List all organizations
    pub async fn list_orgs(
        &self,
        opt: AdminListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let path = format!("/admin/orgs?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// Create an organization for an existing user
    pub async fn create_org_for_user(
        &self,
        user: &str,
        opt: CreateOrgOption,
    ) -> crate::Result<(Organization, Response)> {
        opt.validate()?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}/orgs", escaped[0]);
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// Create a repository for a user
    pub async fn create_repo_for_user(
        &self,
        user: &str,
        opt: CreateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        opt.validate()?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/admin/users/{}/repos", escaped[0]);
        let body = json_body(&opt)?;
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
    use crate::options::org::CreateOrgOption;
    use crate::options::repo::CreateRepoOption;
    #[allow(unused_imports)]
    use crate::{Repository, TrustModel};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::super::test_helpers::{create_test_client, org_json, repo_json};

    #[tokio::test]
    async fn test_list_orgs() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "id": 1,
                "name": "myorg",
                "username": "myorg",
                "full_name": "My Org",
                "email": "",
                "avatar_url": "",
                "description": "",
                "website": "",
                "location": "",
                "visibility": "public",
                "repo_admin_change_team_access": false
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/admin/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (orgs, resp) = client.admin().list_orgs(Default::default()).await.unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].user_name, "myorg");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_org_for_user() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/janedoe/orgs"))
            .respond_with(ResponseTemplate::new(201).set_body_json(org_json()))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            ..Default::default()
        };
        let (org, resp) = client
            .admin()
            .create_org_for_user("janedoe", opt)
            .await
            .unwrap();
        assert_eq!(org.user_name, "neworg");
        assert!(org.repo_admin_change_team_access);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_repo_for_user() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/admin/users/janedoe/repos"))
            .respond_with(ResponseTemplate::new(201).set_body_json(repo_json()))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: "newrepo".to_string(),
            description: "desc".to_string(),
            private: false,
            issue_labels: "".to_string(),
            auto_init: false,
            template: false,
            gitignores: "".to_string(),
            license: "".to_string(),
            readme: "".to_string(),
            default_branch: "main".to_string(),
            trust_model: TrustModel::Default,
            object_format_name: "".to_string(),
        };
        let (repo, resp) = client
            .admin()
            .create_repo_for_user("janedoe", opt)
            .await
            .unwrap();
        assert_eq!(repo.name, "newrepo");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_list_orgs_server_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/admin/orgs"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.admin().list_orgs(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_for_user_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "".to_string(),
            ..Default::default()
        };
        let result = client.admin().create_org_for_user("janedoe", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_for_user_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            ..Default::default()
        };
        let result = client.admin().create_org_for_user("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    #[tokio::test]
    async fn test_create_repo_for_user_validation_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: String::new(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        let result = client.admin().create_repo_for_user("janedoe", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_repo_for_user_empty_user() {
        let server = MockServer::start().await;
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
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        };
        let result = client.admin().create_repo_for_user("", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }
}
