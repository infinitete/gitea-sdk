// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::notification::*;
use crate::pagination::QueryEncode;
use crate::types::NotificationThread;

pub struct NotificationsApi<'a> {
    client: &'a Client,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct NewNotificationCount {
    #[serde(rename = "new")]
    pub new: i64,
}

impl<'a> NotificationsApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    /// CheckNotifications list users's notification threads
    pub async fn check_notifications(&self) -> crate::Result<(i64, Response)> {
        let (count, resp) = self
            .client()
            .get_parsed_response::<NewNotificationCount, _>(
                reqwest::Method::GET,
                "/notifications/new",
                None,
                None::<&str>,
            )
            .await?;
        Ok((count.new, resp))
    }

    /// GetNotification get notification thread by ID
    pub async fn get_notification(&self, id: i64) -> crate::Result<(NotificationThread, Response)> {
        let path = format!("/notifications/threads/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ReadNotification mark notification thread as read by ID
    pub async fn read_notification(
        &self,
        id: i64,
    ) -> crate::Result<(NotificationThread, Response)> {
        let path = format!("/notifications/threads/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::PATCH, &path, None, None::<&str>)
            .await
    }

    /// ListNotifications list users's notification threads
    pub async fn list_notifications(
        &self,
        opt: ListNotificationOptions,
    ) -> crate::Result<(Vec<NotificationThread>, Response)> {
        let path = format!("/notifications?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ReadNotifications mark notification threads as read
    pub async fn read_notifications(
        &self,
        opt: MarkNotificationOptions,
    ) -> crate::Result<(Vec<NotificationThread>, Response)> {
        let path = format!("/notifications?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// ListRepoNotifications list users's notification threads on a specific repo
    pub async fn list_repo_notifications(
        &self,
        owner: &str,
        repo: &str,
        opt: ListNotificationOptions,
    ) -> crate::Result<(Vec<NotificationThread>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/notifications?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ReadRepoNotifications mark notification threads as read on a specific repo
    pub async fn read_repo_notifications(
        &self,
        owner: &str,
        repo: &str,
        opt: MarkNotificationOptions,
    ) -> crate::Result<(Vec<NotificationThread>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/notifications?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn notification_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "repository": null,
            "subject": null,
            "unread": true,
            "pinned": false,
            "updated_at": "2024-01-15T10:00:00Z",
            "url": "https://gitea.example.com/notifications/1"
        })
    }

    #[tokio::test]
    async fn test_check_notifications() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/notifications/new"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"new": 5})))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (count, resp) = client.notifications().check_notifications().await.unwrap();
        assert_eq!(count, 5);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_notifications() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/notifications"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(vec![notification_json(1), notification_json(2)]),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (threads, resp) = client
            .notifications()
            .list_notifications(Default::default())
            .await
            .unwrap();
        assert_eq!(threads.len(), 2);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_read_notification() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/notifications/threads/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(notification_json(1)))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (thread, resp) = client.notifications().read_notification(1).await.unwrap();
        assert_eq!(thread.id, 1);
        assert_eq!(resp.status, 200);
    }
}
