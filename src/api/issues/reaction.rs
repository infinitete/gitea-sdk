// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::ListIssueReactionsOptions;
use crate::pagination::QueryEncode;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_reaction.go ─────────────────────────────────────────
    // 6 methods (excluding deprecated GetIssueReactions)

    /// `ListIssueReactions` get a list of reactions for an issue with pagination
    pub async fn list_issue_reactions(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueReactionsOptions,
    ) -> crate::Result<(Vec<crate::Reaction>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/reactions?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `GetIssueCommentReactions` get a list of reactions from a comment of an issue
    pub async fn get_issue_comment_reactions(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> crate::Result<(Vec<crate::Reaction>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// `PostIssueReaction` add a reaction to an issue
    pub async fn post_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        reaction: &str,
    ) -> crate::Result<(crate::Reaction, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteIssueReaction` remove a reaction from an issue
    pub async fn delete_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        reaction: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `PostIssueCommentReaction` add a reaction to a comment of an issue
    pub async fn post_issue_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction: &str,
    ) -> crate::Result<(crate::Reaction, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// `DeleteIssueCommentReaction` remove a reaction from a comment of an issue
    pub async fn delete_issue_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
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
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_reaction.go ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_reactions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([reaction_json(":+1:")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reactions, resp) = client
            .issues()
            .list_issue_reactions("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(reactions.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_reactions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_reactions("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_comment_reactions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([reaction_json(":+1:")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reactions, resp) = client
            .issues()
            .get_issue_comment_reactions("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(reactions.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_comment_reactions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_comment_reactions("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_post_issue_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(reaction_json(":+1:")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reaction, resp) = client
            .issues()
            .post_issue_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(reaction.reaction, ":+1:");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_post_issue_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .post_issue_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_post_issue_comment_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(reaction_json(":+1:")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reaction, resp) = client
            .issues()
            .post_issue_comment_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(reaction.reaction, ":+1:");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_post_issue_comment_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .post_issue_comment_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_comment_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_comment_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_comment_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_comment_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }
}
