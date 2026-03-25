// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::Activity;

use super::OrgsApi;

impl<'a> OrgsApi<'a> {
    // ── org_social.go ─────────────────────────────────────────────────────

    /// `UpdateOrgAvatar` updates the organization's avatar
    pub async fn update_org_avatar(
        &self,
        org: &str,
        opt: &crate::options::user::UpdateUserAvatarOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(opt)?;
        let path = format!("/orgs/{}/avatar", escaped[0]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
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
        Ok(response)
    }

    /// `DeleteOrgAvatar` deletes the organization's avatar
    pub async fn delete_org_avatar(&self, org: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/avatar", escaped[0]);
        let (status, response) = self
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
        Ok(response)
    }

    /// `RenameOrg` renames an organization
    pub async fn rename_org(&self, org: &str, opt: RenameOrgOption) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/rename", escaped[0]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
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
        Ok(response)
    }

    /// `ListOrgActivityFeeds` lists the organization's activity feeds
    pub async fn list_org_activity_feeds(
        &self,
        org: &str,
        opt: ListOrgActivityFeedsOptions,
    ) -> crate::Result<(Vec<Activity>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!(
            "/orgs/{}/activities/feeds?{}",
            escaped[0],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// `ListTeamActivityFeeds` lists the team's activity feeds
    pub async fn list_team_activity_feeds(
        &self,
        team_id: i64,
        opt: ListTeamActivityFeedsOptions,
    ) -> crate::Result<(Vec<Activity>, Response)> {
        let path = format!("/teams/{}/activities/feeds?{}", team_id, opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
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

    // ── update_org_avatar ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_org_avatar_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/avatar"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = crate::options::user::UpdateUserAvatarOption {
            image: "base64data".to_string(),
        };
        let resp = client
            .orgs()
            .update_org_avatar("testorg", &opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_update_org_avatar_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/avatar"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = crate::options::user::UpdateUserAvatarOption {
            image: "base64data".to_string(),
        };
        let result = client.orgs().update_org_avatar("testorg", &opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_org_avatar_validation_empty_image() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = crate::options::user::UpdateUserAvatarOption {
            image: String::new(),
        };
        let result = client.orgs().update_org_avatar("testorg", &opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("image is required")
        );
    }

    // ── delete_org_avatar ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_avatar_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/avatar"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_org_avatar("testorg").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_avatar_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/avatar"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_org_avatar("testorg").await;
        assert!(result.is_err());
    }

    // ── rename_org ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_rename_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/rename"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = RenameOrgOption {
            new_name: "new-org-name".to_string(),
        };
        let resp = client.orgs().rename_org("testorg", opt).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_rename_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/rename"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = RenameOrgOption {
            new_name: "new-org-name".to_string(),
        };
        let result = client.orgs().rename_org("testorg", opt).await;
        assert!(result.is_err());
    }

    // ── list_org_activity_feeds ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_activity_feeds_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([activity_json(1), activity_json(2)]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/activities/feeds"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (feeds, resp) = client
            .orgs()
            .list_org_activity_feeds("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(feeds.len(), 2);
        assert_eq!(feeds[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_activity_feeds_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/activities/feeds"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_activity_feeds("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_team_activity_feeds ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_team_activity_feeds_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([activity_json(1)]);
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/activities/feeds"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (feeds, resp) = client
            .orgs()
            .list_team_activity_feeds(5, Default::default())
            .await
            .unwrap();
        assert_eq!(feeds.len(), 1);
        assert_eq!(feeds[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_team_activity_feeds_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/activities/feeds"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_team_activity_feeds(5, Default::default())
            .await;
        assert!(result.is_err());
    }
}
