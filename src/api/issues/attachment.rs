// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::types::Attachment;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_comment.go ──────────────────────────────────────────
    // attachment methods

    /// ListIssueCommentAttachments lists all attachments for a comment
    pub async fn list_issue_comment_attachments(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> crate::Result<(Vec<Attachment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssueCommentAttachment gets a comment attachment
    pub async fn get_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        attachment_id: i64,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets/{attachment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// EditIssueCommentAttachment updates a comment attachment
    pub async fn edit_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        attachment_id: i64,
        form: crate::options::release::EditAttachmentOption,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        form.validate()?;
        let body = json_body(&form)?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets/{attachment_id}",
            escaped[0], escaped[1]
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

    /// DeleteIssueCommentAttachment deletes a comment attachment
    pub async fn delete_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        attachment_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets/{attachment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_comment.go ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_comment_attachments_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([attachment_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (attachments, resp) = client
            .issues()
            .list_issue_comment_attachments("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_comment_attachments_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_comment_attachments("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_comment_attachment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(attachment_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (att, resp) = client
            .issues()
            .get_issue_comment_attachment("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(att.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_comment_attachment_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_comment_attachment("owner", "repo", 1, 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_attachment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(attachment_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let form = crate::options::release::EditAttachmentOption {
            name: "new_name.txt".to_string(),
        };
        let (att, resp) = client
            .issues()
            .edit_issue_comment_attachment("owner", "repo", 1, 1, form)
            .await
            .unwrap();
        assert_eq!(att.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_issue_comment_attachment_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let form = crate::options::release::EditAttachmentOption {
            name: "new_name.txt".to_string(),
        };
        let result = client
            .issues()
            .edit_issue_comment_attachment("owner", "repo", 1, 1, form)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_attachment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let form = crate::options::release::EditAttachmentOption {
            name: "  ".to_string(),
        };
        let result = client
            .issues()
            .edit_issue_comment_attachment("owner", "repo", 1, 1, form)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_comment_attachment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_comment_attachment("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_comment_attachment_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_comment_attachment("owner", "repo", 1, 1)
            .await;
        assert!(result.is_err());
    }
}
