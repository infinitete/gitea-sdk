// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::*;
use crate::pagination::QueryEncode;
use crate::types::Milestone;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_milestone.go ────────────────────────────────────────
    // 8 methods

    /// `ListRepoMilestones` list all the milestones of one repository
    pub async fn list_repo_milestones(
        &self,
        owner: &str,
        repo: &str,
        opt: ListMilestoneOption,
    ) -> crate::Result<(Vec<Milestone>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/milestones?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetMilestone` get one milestone by repo name and milestone id
    pub async fn get_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/milestones/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetMilestoneByName` get one milestone by repo and milestone name
    pub async fn get_milestone_by_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `CreateMilestone` create one milestone with options
    pub async fn create_milestone(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateMilestoneOption,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/milestones", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `EditMilestone` modify milestone with options
    pub async fn edit_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditMilestoneOption,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/milestones/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `EditMilestoneByName` modify milestone with options
    pub async fn edit_milestone_by_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        opt: EditMilestoneOption,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteMilestone` delete one milestone by id
    pub async fn delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/milestones/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// `DeleteMilestoneByName` delete one milestone by name
    pub async fn delete_milestone_by_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_milestone.go ────────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_milestones_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([milestone_json(1, "v1.0")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (milestones, resp) = client
            .issues()
            .list_repo_milestones("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(milestones.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_milestones_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_milestones("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v1.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (ms, resp) = client
            .issues()
            .get_milestone("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(ms.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().get_milestone("owner", "repo", 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_milestone_by_name_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v1.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (ms, resp) = client
            .issues()
            .get_milestone_by_name("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(ms.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_milestone_by_name_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_milestone_by_name("owner", "repo", "v1.0")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(ResponseTemplate::new(201).set_body_json(milestone_json(1, "v1.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateMilestoneOption {
            title: "v1.0".to_string(),
            description: String::new(),
            state: crate::StateType::Open,
            deadline: None,
        };
        let (ms, resp) = client
            .issues()
            .create_milestone("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(ms.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateMilestoneOption {
            title: "v1.0".to_string(),
            description: String::new(),
            state: crate::StateType::Open,
            deadline: None,
        };
        let result = client.issues().create_milestone("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_milestone_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateMilestoneOption {
            title: String::new(),
            description: String::new(),
            state: crate::StateType::Open,
            deadline: None,
        };
        let result = client.issues().create_milestone("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v2.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let (ms, resp) = client
            .issues()
            .edit_milestone("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(ms.title, "v2.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let result = client
            .issues()
            .edit_milestone("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_milestone_by_name_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v2.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let (ms, resp) = client
            .issues()
            .edit_milestone_by_name("owner", "repo", "v1.0", opt)
            .await
            .unwrap();
        assert_eq!(ms.title, "v2.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_milestone_by_name_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let result = client
            .issues()
            .edit_milestone_by_name("owner", "repo", "v1.0", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_milestone("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().delete_milestone("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_milestone_by_name_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_milestone_by_name("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_milestone_by_name_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_milestone_by_name("owner", "repo", "v1.0")
            .await;
        assert!(result.is_err());
    }
}
