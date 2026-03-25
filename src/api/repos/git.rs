// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── git_blob.go (1 method) ────────────────────────────────────

    /// `GetBlob` get a blob of a repository
    pub async fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::Result<(GitBlobResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/blobs/{}", escaped[0], escaped[1], sha);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── git_hook.go (4 methods) ───────────────────────────────────

    /// `ListGitHooks` list git hooks
    pub async fn list_git_hooks(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoGitHooksOptions,
    ) -> crate::Result<(Vec<GitHook>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/hooks/git?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetGitHook` get a git hook
    pub async fn get_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::Result<(GitHook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/hooks/git/{}", escaped[0], escaped[1], id);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `EditGitHook` edit a git hook
    pub async fn edit_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        opt: EditGitHookOption,
    ) -> crate::Result<(GitHook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/hooks/git/{}", escaped[0], escaped[1], id);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteGitHook` delete a git hook
    pub async fn delete_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/hooks/git/{}", escaped[0], escaped[1], id);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_refs.go (3 methods) ──────────────────────────────────

    /// `GetRepoRef` get one ref's information of one repository
    pub async fn get_repo_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Reference, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let ref_trimmed = ref_.trim_start_matches("refs/");
        let ref_escaped = crate::internal::escape::path_escape_segments(ref_trimmed);
        let path = format!(
            "/repos/{}/{}/git/refs/{}",
            escaped[0], escaped[1], ref_escaped
        );
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        if let Ok(single) = serde_json::from_slice::<Reference>(&data) {
            Ok((single, resp))
        } else {
            let mut refs: Vec<Reference> = serde_json::from_slice(&data)?;
            refs.pop()
                .map(|reference| (reference, resp))
                .ok_or_else(|| {
                    crate::Error::Json(serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "empty ref array response",
                    )))
                })
        }
    }

    /// `GetRepoRefs` get list of ref's information of one repository
    pub async fn get_repo_refs(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<Reference>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let ref_trimmed = ref_.trim_start_matches("refs/");
        let ref_escaped = crate::internal::escape::path_escape_segments(ref_trimmed);
        let path = format!(
            "/repos/{}/{}/git/refs/{}",
            escaped[0], escaped[1], ref_escaped
        );
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        // Try array first, then single object
        if let Ok(refs) = serde_json::from_slice::<Vec<Reference>>(&data) {
            Ok((refs, resp))
        } else {
            let single: Reference = serde_json::from_slice(&data)?;
            Ok((vec![single], resp))
        }
    }

    /// `ListAllGitRefs` get all refs from a repository
    pub async fn list_all_git_refs(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<Reference>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/refs", escaped[0], escaped[1]);
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
    async fn test_get_blob_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/blobs/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_git_blob_json()))
            .mount(&server)
            .await;
        let result = client.repos().get_blob("owner", "repo", "abc123").await;
        assert!(result.is_ok());
        let (blob, resp) = result.unwrap();
        assert_eq!(blob.sha, "abc123");
        assert_eq!(blob.encoding, "base64");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_blob_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/blobs/abc123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_blob("owner", "repo", "abc123").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_git_hooks_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_git_hook_json()])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_git_hooks("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (hooks, resp) = result.unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "pre-receive");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_git_hooks_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_git_hooks("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_git_hook_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_git_hook_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_ok());
        let (hook, resp) = result.unwrap();
        assert_eq!(hook.name, "pre-receive");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_git_hook_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_git_hook_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_git_hook_json()))
            .mount(&server)
            .await;
        let opt = EditGitHookOption {
            content: "#!/bin/sh\necho hello".to_string(),
        };
        let result = client
            .repos()
            .edit_git_hook("owner", "repo", "pre-receive", opt)
            .await;
        assert!(result.is_ok());
        let (hook, resp) = result.unwrap();
        assert_eq!(hook.name, "pre-receive");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_git_hook_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditGitHookOption {
            content: "#!/bin/sh\necho hello".to_string(),
        };
        let result = client
            .repos()
            .edit_git_hook("owner", "repo", "pre-receive", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_git_hook_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_git_hook_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_ref_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_reference_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_ref("owner", "repo", "refs/heads/main")
            .await;
        assert!(result.is_ok());
        let (ref_, resp) = result.unwrap();
        assert_eq!(ref_.ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_ref_happy_with_array_payload() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads/main"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_reference_json()])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_ref("owner", "repo", "refs/heads/main")
            .await;
        assert!(result.is_ok());
        let (ref_, resp) = result.unwrap();
        assert_eq!(ref_.ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_ref_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads/nonexistent"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_ref("owner", "repo", "refs/heads/nonexistent")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_refs_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_reference_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_refs("owner", "repo", "refs/heads")
            .await;
        assert!(result.is_ok());
        let (refs, resp) = result.unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_refs_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_refs("owner", "repo", "refs/heads")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_all_git_refs_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_reference_json()])),
            )
            .mount(&server)
            .await;
        let result = client.repos().list_all_git_refs("owner", "repo").await;
        assert!(result.is_ok());
        let (refs, resp) = result.unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_all_git_refs_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().list_all_git_refs("owner", "repo").await;
        assert!(result.is_err());
    }
}
