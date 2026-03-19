// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::options::issue::ListIssueSubscribersOptions;
use crate::pagination::QueryEncode;
use crate::types::WatchInfo;

use super::IssuesApi;

impl<'a> IssuesApi<'a> {
    // ── issue_subscription.go ─────────────────────────────────────
    // 7 methods (excluding deprecated GetIssueSubscribers)

    /// ListIssueSubscribers get list of users who subscribed on an issue with pagination
    pub async fn list_issue_subscribers(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueSubscribersOptions,
    ) -> crate::Result<(Vec<crate::User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddIssueSubscription subscribe user to issue
    pub async fn add_issue_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        user: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, user])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::PUT, &path, None, None::<&str>)
            .await?;
        if status == 201 {
            return Ok(resp);
        }
        if status == 200 {
            return Err(crate::Error::Validation("already subscribed".to_string()));
        }
        Err(crate::Error::UnknownApi {
            status,
            body: format!("unexpected status: {status}"),
        })
    }

    /// DeleteIssueSubscription unsubscribe user from issue
    pub async fn delete_issue_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        user: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, user])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await?;
        if status == 201 {
            return Ok(resp);
        }
        if status == 200 {
            return Err(crate::Error::Validation("already unsubscribed".to_string()));
        }
        Err(crate::Error::UnknownApi {
            status,
            body: format!("unexpected status: {status}"),
        })
    }

    /// CheckIssueSubscription check if current user is subscribed to an issue
    pub async fn check_issue_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(WatchInfo, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions/check",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// IssueSubscribe subscribe current user to an issue
    pub async fn issue_subscribe(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let (user, _) = self.client().users().get_my_info().await?;
        self.add_issue_subscription(owner, repo, index, &user.user_name)
            .await
    }

    /// IssueUnsubscribe unsubscribe current user from an issue
    pub async fn issue_unsubscribe(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let (user, _) = self.client().users().get_my_info().await?;
        self.delete_issue_subscription(owner, repo, index, &user.user_name)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    // ── issue_subscription.go ─────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_subscribers_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([user_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (users, resp) = client
            .issues()
            .list_issue_subscribers("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_subscribers_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_subscribers("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_issue_subscription_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .add_issue_subscription("owner", "repo", 1, "testuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_add_issue_subscription_already_subscribed() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .add_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_issue_subscription_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .add_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_subscription_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_subscription("owner", "repo", 1, "testuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_delete_issue_subscription_already_unsubscribed() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_subscription_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_issue_subscription_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/check",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(watch_info_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (info, resp) = client
            .issues()
            .check_issue_subscription("owner", "repo", 1)
            .await
            .unwrap();
        assert!(info.subscribed);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_check_issue_subscription_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/check",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .check_issue_subscription("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_issue_subscribe_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .issue_subscribe("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_issue_subscribe_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().issue_subscribe("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_issue_unsubscribe_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .issue_unsubscribe("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_issue_unsubscribe_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().issue_unsubscribe("owner", "repo", 1).await;
        assert!(result.is_err());
    }
}
