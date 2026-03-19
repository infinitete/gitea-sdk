// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::json_header;
use crate::options::repo::*;
use crate::pagination::QueryEncode;

impl<'a> super::ReposApi<'a> {
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
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

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
