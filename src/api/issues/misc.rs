// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::*;
use crate::types::Issue;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_ext.go ──────────────────────────────────────────────
    // lock / unlock / deadline

    /// `LockIssue` locks an issue
    pub async fn lock_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: LockIssueOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/lock", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    /// `UnlockIssue` unlocks an issue
    pub async fn unlock_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/lock", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    /// `UpdateIssueDeadline` updates an issue's deadline
    pub async fn update_issue_deadline(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: EditDeadlineOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/deadline",
            escaped[0], escaped[1]
        );
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if let Ok(issue) = serde_json::from_slice::<Issue>(&data) {
            return Ok((issue, resp));
        }
        if serde_json::from_slice::<serde_json::Value>(&data)
            .ok()
            .and_then(|value| value.get("due_date").cloned())
            .is_some()
        {
            let (issue, _) = self.get_issue(owner, repo, index).await?;
            return Ok((issue, resp));
        }
        Err(crate::Error::Json(
            serde_json::from_slice::<Issue>(&data).unwrap_err(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;
    use wiremock::matchers::{method, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_ext.go ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_lock_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = LockIssueOption {
            lock_reason: String::new(),
        };
        let resp = client
            .issues()
            .lock_issue("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_lock_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = LockIssueOption {
            lock_reason: String::new(),
        };
        let result = client.issues().lock_issue("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlock_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .unlock_issue("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unlock_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().unlock_issue("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_issue_deadline_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/deadline"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(1, 1, "Bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditDeadlineOption {
            deadline: Some(time::OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            )),
        };
        let (issue, resp) = client
            .issues()
            .update_issue_deadline("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_update_issue_deadline_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/deadline"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditDeadlineOption { deadline: None };
        let result = client
            .issues()
            .update_issue_deadline("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }
}
