// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo.go: search ───────────────────────────────────────────

    /// `SearchRepos` search for repositories
    pub async fn search_repos(
        &self,
        opt: SearchRepoOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let path = format!("/repos/search?{}", opt.query_encode());
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;

        if let Ok(repos) = serde_json::from_slice::<Vec<Repository>>(&data) {
            return Ok((repos, resp));
        }

        #[derive(serde::Deserialize)]
        struct SearchReposEnvelope {
            #[serde(default)]
            data: Vec<Repository>,
        }

        let wrapped: SearchReposEnvelope = serde_json::from_slice(&data)?;
        Ok((wrapped.data, resp))
    }

    /// `ListOrgRepos` list repositories of an organization
    pub async fn list_org_repos(
        &self,
        org: &str,
        opt: ListOrgReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/repos?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateOrgRepo` create a repository in an organization
    pub async fn create_org_repo(
        &self,
        org: &str,
        opt: CreateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/repos", escaped[0]);
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
    use crate::options::repo::CreateRepoOption;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_search_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .search_repos(Default::default())
            .await
            .unwrap();
        assert!(!repos.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/search"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(json!({"message": "Internal Server Error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().search_repos(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_org_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "myorg"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .list_org_repos("myorg", Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_org_repos("myorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(minimal_repo_json(2, "newrepo", "myorg")),
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
        let (repo, resp) = client.repos().create_org_repo("myorg", opt).await.unwrap();
        assert_eq!(repo.id, 2);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
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
        let result = client.repos().create_org_repo("myorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_repo_avatar_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/avatar"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .repos()
            .delete_repo_avatar("owner", "repo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_repo_avatar_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/avatar"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_repo_avatar("owner", "repo").await;
        assert!(result.is_err());
    }
}
