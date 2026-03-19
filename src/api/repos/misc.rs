// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::repo::*;
use crate::pagination::QueryEncode;
use crate::types::repository::*;

impl<'a> super::ReposApi<'a> {
    // ── repo_mirror.go (4 methods) ────────────────────────────────

    /// CreatePushMirror create a push mirror for a repository
    pub async fn create_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        opt: CreatePushMirrorOption,
    ) -> crate::Result<(PushMirrorResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/push_mirrors", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListPushMirrors list push mirrors of a repository
    pub async fn list_push_mirrors(
        &self,
        owner: &str,
        repo: &str,
        opt: ListPushMirrorOptions,
    ) -> crate::Result<(Vec<PushMirrorResponse>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/push_mirrors?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetPushMirror get a push mirror of a repository
    pub async fn get_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        remote_name: &str,
    ) -> crate::Result<(PushMirrorResponse, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, remote_name])?;
        let path = format!(
            "/repos/{}/{}/push_mirrors/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeletePushMirror delete a push mirror of a repository
    pub async fn delete_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        remote_name: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, remote_name])?;
        let path = format!(
            "/repos/{}/{}/push_mirrors/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_template.go (1 method) ───────────────────────────────

    /// CreateRepoFromTemplate create a repository using a template
    pub async fn create_repo_from_template(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateRepoFromTemplateOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/generate", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── repo_topics.go (4 methods) ────────────────────────────────

    /// ListTopics list all repository's topics
    pub async fn list_topics(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoTopicsOptions,
    ) -> crate::Result<(Vec<String>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/topics?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        #[derive(serde::Deserialize)]
        struct TopicsList {
            topics: Vec<String>,
        }
        let list: TopicsList = serde_json::from_slice(&data)?;
        Ok((list.topics, resp))
    }

    /// SetTopics replace the list of a repository's topics
    pub async fn set_topics(
        &self,
        owner: &str,
        repo: &str,
        topics: Vec<String>,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = serde_json::to_string(&serde_json::json!({"topics": topics}))?;
        let path = format!("/repos/{}/{}/topics", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// AddTopic add a topic to a repository
    pub async fn add_topic(&self, owner: &str, repo: &str, topic: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, topic])?;
        let path = format!("/repos/{}/{}/topics/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// DeleteTopic delete a topic from a repository
    pub async fn delete_topic(
        &self,
        owner: &str,
        repo: &str,
        topic: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, topic])?;
        let path = format!("/repos/{}/{}/topics/{}", escaped[0], escaped[1], escaped[2]);
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
    async fn test_create_push_mirror_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_push_mirror_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreatePushMirrorOption {
            interval: "8h".to_string(),
            remote_address: "https://example.com/repo.git".to_string(),
            remote_password: String::new(),
            remote_username: String::new(),
            sync_on_commit: false,
        };
        let (mirror, resp) = client
            .repos()
            .create_push_mirror("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(mirror.remote_address, "https://example.com/repo.git");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_push_mirror_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreatePushMirrorOption {
            interval: "8h".to_string(),
            remote_address: "https://example.com/repo.git".to_string(),
            remote_password: String::new(),
            remote_username: String::new(),
            sync_on_commit: false,
        };
        let result = client
            .repos()
            .create_push_mirror("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_push_mirrors_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_push_mirror_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (mirrors, resp) = client
            .repos()
            .list_push_mirrors("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(mirrors.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_push_mirrors_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_push_mirrors("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_push_mirror_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_push_mirror_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (mirror, resp) = client
            .repos()
            .get_push_mirror("owner", "repo", "origin")
            .await
            .unwrap();
        assert_eq!(mirror.interval, "8h");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_push_mirror_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_push_mirror("owner", "repo", "origin")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_push_mirror_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_push_mirror("owner", "repo", "origin")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_push_mirror_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_push_mirror("owner", "repo", "origin")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_repo_from_template_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/template/generate"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(minimal_repo_json(3, "newrepo", "newowner")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateRepoFromTemplateOption {
            owner: "newowner".to_string(),
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            git_content: true,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        let (repo, resp) = client
            .repos()
            .create_repo_from_template("owner", "template", opt)
            .await
            .unwrap();
        assert_eq!(repo.id, 3);
        assert_eq!(repo.name, "newrepo");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_repo_from_template_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/template/generate"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateRepoFromTemplateOption {
            owner: "newowner".to_string(),
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            git_content: true,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        let result = client
            .repos()
            .create_repo_from_template("owner", "template", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_topics_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/topics"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "topics": ["rust", "gitea"]
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (topics, resp) = client
            .repos()
            .list_topics("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(topics, vec!["rust", "gitea"]);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_topics_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/topics"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_topics("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_topics_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/topics"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .set_topics("owner", "repo", vec!["rust".to_string()])
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_set_topics_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/topics"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .set_topics("owner", "repo", vec!["rust".to_string()])
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_topic_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/topics/rust"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().add_topic("owner", "repo", "rust").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_add_topic_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/topics/rust"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().add_topic("owner", "repo", "rust").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_topic_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/topics/rust"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_topic("owner", "repo", "rust").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_topic_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/topics/rust"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_topic("owner", "repo", "rust").await;
        assert!(result.is_err());
    }
}
