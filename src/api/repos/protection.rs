// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;
use crate::version::{VERSION_1_12_0, VERSION_1_23_0};

impl<'a> super::ReposApi<'a> {
    // ── repo_branch_protection.go (5 methods) ─────────────────────

    /// ListBranchProtections list branch protections
    pub async fn list_branch_protections(
        &self,
        owner: &str,
        repo: &str,
        opt: ListBranchProtectionsOptions,
    ) -> crate::Result<(Vec<BranchProtection>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/branch_protections?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetBranchProtection get a branch protection
    pub async fn get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<(BranchProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/branch_protections/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateBranchProtection create a branch protection
    pub async fn create_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateBranchProtectionOption,
    ) -> crate::Result<(BranchProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/branch_protections", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditBranchProtection edit a branch protection
    pub async fn edit_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        opt: EditBranchProtectionOption,
    ) -> crate::Result<(BranchProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/branch_protections/{}",
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

    /// DeleteBranchProtection delete a branch protection
    pub async fn delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/branch_protections/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_tag_protection.go (5 methods) ────────────────────────

    /// ListTagProtections list tag protections
    pub async fn list_tag_protections(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoTagProtectionsOptions,
    ) -> crate::Result<(Vec<TagProtection>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_23_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/tag_protections?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTagProtection get a tag protection
    pub async fn get_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateTagProtection create a tag protection
    pub async fn create_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateTagProtectionOption,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tag_protections", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditTagProtection edit a tag protection
    pub async fn edit_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditTagProtectionOption,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteTagProtection delete a tag protection
    pub async fn delete_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
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
    async fn test_list_branch_protections_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_branch_protection_json()])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_branch_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (protections, resp) = result.unwrap();
        assert_eq!(protections.len(), 1);
        assert_eq!(protections[0].branch_name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_branch_protections_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_branch_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_branch_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(minimal_branch_protection_json()),
            )
            .mount(&server)
            .await;
        let opt = CreateBranchProtectionOption {
            branch_name: "main".to_string(),
            rule_name: "main".to_string(),
            enable_push: false,
            enable_push_whitelist: false,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: false,
            enable_merge_whitelist: false,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: false,
            status_check_contexts: vec![],
            required_approvals: 0,
            enable_approvals_whitelist: false,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: false,
            block_on_official_review_requests: false,
            block_on_outdated_branch: false,
            dismiss_stale_approvals: false,
            require_signed_commits: false,
            protected_file_patterns: String::new(),
            unprotected_file_patterns: String::new(),
        };
        let result = client
            .repos()
            .create_branch_protection("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        let (bp, resp) = result.unwrap();
        assert_eq!(bp.branch_name, "main");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_branch_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateBranchProtectionOption {
            branch_name: "main".to_string(),
            rule_name: "main".to_string(),
            enable_push: false,
            enable_push_whitelist: false,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: false,
            enable_merge_whitelist: false,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: false,
            status_check_contexts: vec![],
            required_approvals: 0,
            enable_approvals_whitelist: false,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: false,
            block_on_official_review_requests: false,
            block_on_outdated_branch: false,
            dismiss_stale_approvals: false,
            require_signed_commits: false,
            protected_file_patterns: String::new(),
            unprotected_file_patterns: String::new(),
        };
        let result = client
            .repos()
            .create_branch_protection("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_branch_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_branch_protection_json()),
            )
            .mount(&server)
            .await;
        let opt = EditBranchProtectionOption {
            enable_push: None,
            enable_push_whitelist: None,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: None,
            enable_merge_whitelist: None,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: None,
            status_check_contexts: vec![],
            required_approvals: None,
            enable_approvals_whitelist: None,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: None,
            block_on_official_review_requests: None,
            block_on_outdated_branch: None,
            dismiss_stale_approvals: None,
            require_signed_commits: None,
            protected_file_patterns: None,
            unprotected_file_patterns: None,
        };
        let result = client
            .repos()
            .edit_branch_protection("owner", "repo", "main", opt)
            .await;
        assert!(result.is_ok());
        let (bp, resp) = result.unwrap();
        assert_eq!(bp.branch_name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_branch_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditBranchProtectionOption {
            enable_push: None,
            enable_push_whitelist: None,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: None,
            enable_merge_whitelist: None,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: None,
            status_check_contexts: vec![],
            required_approvals: None,
            enable_approvals_whitelist: None,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: None,
            block_on_official_review_requests: None,
            block_on_outdated_branch: None,
            dismiss_stale_approvals: None,
            require_signed_commits: None,
            protected_file_patterns: None,
            unprotected_file_patterns: None,
        };
        let result = client
            .repos()
            .edit_branch_protection("owner", "repo", "main", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_tag_protections_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_tag_protection_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_tag_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (protections, resp) = result.unwrap();
        assert_eq!(protections.len(), 1);
        assert_eq!(protections[0].name_pattern, "v*");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_tag_protections_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_tag_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let result = client.repos().get_tag_protection("owner", "repo", 1).await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(tp.name_pattern, "v*");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_tag_protection("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let opt = CreateTagProtectionOption {
            name_pattern: "v*".to_string(),
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .create_tag_protection("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateTagProtectionOption {
            name_pattern: "v*".to_string(),
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .create_tag_protection("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let opt = EditTagProtectionOption {
            name_pattern: None,
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .edit_tag_protection("owner", "repo", 1, opt)
            .await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditTagProtectionOption {
            name_pattern: None,
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .edit_tag_protection("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_tag_protection("owner", "repo", 1)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_tag_protection("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }
}
