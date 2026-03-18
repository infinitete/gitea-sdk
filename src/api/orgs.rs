// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::{Activity, Label, OrgPermissions, Organization, Secret, Team, User};
use crate::{Deserialize, Serialize};

/// API methods for organization resources.
pub struct OrgsApi<'a> {
    client: &'a Client,
}

fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TeamSearchResults {
    #[serde(default)]
    ok: bool,
    #[serde(default)]
    error: String,
    #[serde(default)]
    data: Vec<Team>,
}

impl<'a> OrgsApi<'a> {
    /// Create a new `OrgsApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    // ── org.go ────────────────────────────────────────────────────────────

    /// ListOrgs lists all public organizations
    pub async fn list_orgs(
        &self,
        opt: ListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let path = format!("/orgs?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyOrgs list all of current user's organizations
    pub async fn list_my_orgs(
        &self,
        opt: ListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let path = format!("/user/orgs?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListUserOrgs list all of some user's organizations
    pub async fn list_user_orgs(
        &self,
        user: &str,
        opt: ListOrgsOptions,
    ) -> crate::Result<(Vec<Organization>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/orgs?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetOrg get one organization by name
    pub async fn get_org(&self, org: &str) -> crate::Result<(Organization, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateOrg creates an organization
    pub async fn create_org(
        &self,
        opt: CreateOrgOption,
    ) -> crate::Result<(Organization, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/orgs",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditOrg modify one organization via options
    pub async fn edit_org(&self, org: &str, opt: EditOrgOption) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteOrg deletes an organization
    pub async fn delete_org(&self, org: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}", escaped[0]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    // ── org_team.go ───────────────────────────────────────────────────────

    /// ListOrgTeams lists all teams of an organization
    pub async fn list_org_teams(
        &self,
        org: &str,
        opt: ListTeamsOptions,
    ) -> crate::Result<(Vec<Team>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/teams?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListMyTeams lists all the teams of the current user
    pub async fn list_my_teams(
        &self,
        opt: ListTeamsOptions,
    ) -> crate::Result<(Vec<Team>, Response)> {
        let path = format!("/user/teams?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTeam gets a team by ID
    pub async fn get_team(&self, id: i64) -> crate::Result<(Team, Response)> {
        let path = format!("/teams/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// SearchOrgTeams search for teams in an org
    pub async fn search_org_teams(
        &self,
        org: &str,
        opt: SearchTeamsOptions,
    ) -> crate::Result<(Vec<Team>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/teams/search?{}", escaped[0], opt.query_encode());
        let (result, response) = self
            .client()
            .get_parsed_response::<TeamSearchResults, _>(
                reqwest::Method::GET,
                &path,
                None,
                None::<&str>,
            )
            .await?;
        if !result.ok {
            return Err(crate::Error::UnknownApi {
                status: response.status,
                body: result.error,
            });
        }
        Ok((result.data, response))
    }

    /// CreateTeam creates a team for an organization
    pub async fn create_team(
        &self,
        org: &str,
        opt: CreateTeamOption,
    ) -> crate::Result<(Team, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/teams", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditTeam edits a team of an organization
    pub async fn edit_team(&self, id: i64, opt: EditTeamOption) -> crate::Result<Response> {
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/teams/{id}");
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteTeam deletes a team of an organization
    pub async fn delete_team(&self, id: i64) -> crate::Result<Response> {
        let path = format!("/teams/{id}");
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// ListTeamMembers lists all members of a team
    pub async fn list_team_members(
        &self,
        id: i64,
        opt: ListTeamMembersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let path = format!("/teams/{}/members?{}", id, opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTeamMember gets a member of a team
    pub async fn get_team_member(&self, id: i64, user: &str) -> crate::Result<(User, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/teams/{}/members/{}", id, escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddTeamMember adds a member to a team
    pub async fn add_team_member(&self, id: i64, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/teams/{}/members/{}", id, escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// RemoveTeamMember removes a member from a team
    pub async fn remove_team_member(&self, id: i64, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/teams/{}/members/{}", id, escaped[0]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// AddTeamRepository adds a repository to a team
    pub async fn add_team_repo(&self, id: i64, org: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, repo])?;
        let path = format!("/teams/{}/repos/{}/{}", id, escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// RemoveTeamRepository removes a repository from a team
    pub async fn remove_team_repo(
        &self,
        id: i64,
        org: &str,
        repo: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, repo])?;
        let path = format!("/teams/{}/repos/{}/{}", id, escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── org_member.go ─────────────────────────────────────────────────────

    /// DeleteOrgMembership remove a member from an organization
    pub async fn delete_org_membership(&self, org: &str, user: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/members/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// ListOrgMembership list an organization's members
    pub async fn list_org_membership(
        &self,
        org: &str,
        opt: ListOrgMembershipOption,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/members?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ListPublicOrgMembership list an organization's public members
    pub async fn list_public_org_membership(
        &self,
        org: &str,
        opt: ListOrgMembershipOption,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/public_members?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CheckOrgMembership check if a user is a member of an organization
    pub async fn check_org_membership(
        &self,
        org: &str,
        user: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/members/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        match status {
            204 => Ok((true, response)),
            404 => Ok((false, response)),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// CheckPublicOrgMembership check if a user is a public member of an organization
    pub async fn check_public_org_membership(
        &self,
        org: &str,
        user: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/public_members/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        match status {
            204 => Ok((true, response)),
            404 => Ok((false, response)),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// SetPublicOrgMembership publicize or conceal a user's membership
    pub async fn set_public_org_membership(
        &self,
        org: &str,
        user: &str,
        visible: bool,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, user])?;
        let path = format!("/orgs/{}/public_members/{}", escaped[0], escaped[1]);
        let method = if visible {
            reqwest::Method::PUT
        } else {
            reqwest::Method::DELETE
        };
        let (status, response) = self
            .client()
            .get_status_code(method, &path, None, None::<&str>)
            .await?;
        match status {
            204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// GetOrgPermissions returns user permissions for specific organization
    pub async fn get_org_permissions(
        &self,
        org: &str,
        user: &str,
    ) -> crate::Result<(OrgPermissions, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user, org])?;
        let path = format!("/users/{}/orgs/{}/permissions", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    // ── org_label.go ──────────────────────────────────────────────────────

    /// ListOrgLabels returns the labels defined at the org level
    pub async fn list_org_labels(
        &self,
        org: &str,
        opt: ListOrgLabelsOptions,
    ) -> crate::Result<(Vec<Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/labels?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CreateOrgLabel creates a new label under an organization
    pub async fn create_org_label(
        &self,
        org: &str,
        opt: CreateOrgLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/labels", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// GetOrgLabel get one label of organization by org id
    pub async fn get_org_label(
        &self,
        org: &str,
        label_id: i64,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/labels/{label_id}", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// EditOrgLabel edits an existing org-level label by ID
    pub async fn edit_org_label(
        &self,
        org: &str,
        label_id: i64,
        opt: EditOrgLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/labels/{label_id}", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteOrgLabel deletes an org label by ID
    pub async fn delete_org_label(&self, org: &str, label_id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/labels/{label_id}", escaped[0]);
        let (_, response) = self
            .client()
            .get_response(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        Ok(response)
    }

    // ── org_action.go ─────────────────────────────────────────────────────

    /// ListOrgActionSecret list an organization's secrets
    pub async fn list_org_action_secrets(
        &self,
        org: &str,
        opt: ListOrgActionSecretOption,
    ) -> crate::Result<(Vec<Secret>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!(
            "/orgs/{}/actions/secrets?{}",
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

    /// ListOrgActionVariable lists an organization's action variables
    pub async fn list_org_action_variables(
        &self,
        org: &str,
        opt: ListOrgActionVariableOption,
    ) -> crate::Result<(Vec<OrgActionVariable>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!(
            "/orgs/{}/actions/variables?{}",
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

    /// GetOrgActionVariable gets a single organization's action variable by name
    pub async fn get_org_action_variable(
        &self,
        org: &str,
        name: &str,
    ) -> crate::Result<(OrgActionVariable, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, name])?;
        let path = format!("/orgs/{}/actions/variables/{}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CreateOrgActionVariable creates a variable for the specified organization
    pub async fn create_org_action_variable(
        &self,
        org: &str,
        opt: CreateOrgActionVariableOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/actions/variables/{}", escaped[0], opt.name);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        match status {
            201 | 204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
            400 => Err(crate::Error::Validation("bad request".to_string())),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// UpdateOrgActionVariable updates a variable for the specified organization
    pub async fn update_org_action_variable(
        &self,
        org: &str,
        name: &str,
        opt: UpdateOrgActionVariableOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, name])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/actions/variables/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        match status {
            200 | 204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
            400 => Err(crate::Error::Validation("bad request".to_string())),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    /// CreateOrgActionSecret creates a secret for the specified organization
    pub async fn create_org_action_secret(
        &self,
        org: &str,
        opt: CreateSecretOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/actions/secrets/{}", escaped[0], opt.name);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        match status {
            201 | 204 => Ok(response),
            404 => Err(crate::Error::Validation("forbidden".to_string())),
            400 => Err(crate::Error::Validation("bad request".to_string())),
            _ => Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            }),
        }
    }

    // ── org_block.go ──────────────────────────────────────────────────────

    /// ListOrgBlocks lists users blocked by the organization
    pub async fn list_org_blocks(
        &self,
        org: &str,
        opt: ListOrgBlocksOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/blocks?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CheckOrgBlock checks if a user is blocked by the organization
    pub async fn check_org_block(
        &self,
        org: &str,
        username: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, username])?;
        let path = format!("/orgs/{}/blocks/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        Ok((status == 204, response))
    }

    /// BlockOrgUser blocks a user from the organization
    pub async fn block_org_user(&self, org: &str, username: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, username])?;
        let path = format!("/orgs/{}/blocks/{}", escaped[0], escaped[1]);
        let (status, response) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
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

    /// UnblockOrgUser unblocks a user from the organization
    pub async fn unblock_org_user(&self, org: &str, username: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org, username])?;
        let path = format!("/orgs/{}/blocks/{}", escaped[0], escaped[1]);
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

    // ── org_social.go ─────────────────────────────────────────────────────

    /// UpdateOrgAvatar updates the organization's avatar
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

    /// DeleteOrgAvatar deletes the organization's avatar
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

    /// RenameOrg renames an organization
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

    /// ListOrgActivityFeeds lists the organization's activity feeds
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

    /// ListTeamActivityFeeds lists the team's activity feeds
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

    /// ListTeamRepositories lists all repositories of a team
    pub async fn list_team_repositories(
        &self,
        id: i64,
        opt: ListTeamRepositoriesOptions,
    ) -> crate::Result<(Vec<crate::types::repository::Repository>, Response)> {
        let path = format!("/teams/{}/repos?{}", id, opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn org_json(id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "username": name,
            "full_name": "",
            "email": "",
            "avatar_url": "",
            "description": "",
            "website": "",
            "location": "",
            "visibility": "public",
            "repo_admin_change_team_access": false
        })
    }

    fn team_json(id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "description": "",
            "organization": null,
            "permission": "read",
            "can_create_org_repo": false,
            "includes_all_repositories": false,
            "units": []
        })
    }

    fn user_json(id: i64, login: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "login": login,
            "login_name": "",
            "source_id": 0,
            "full_name": "",
            "email": "",
            "avatar_url": "",
            "html_url": "",
            "language": "",
            "is_admin": false,
            "restricted": false,
            "active": true,
            "prohibit_login": false,
            "location": "",
            "website": "",
            "description": "",
            "visibility": "public",
            "followers_count": 0,
            "following_count": 0,
            "starred_repos_count": 0
        })
    }

    fn label_json(id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "color": "ff0000",
            "description": "",
            "exclusive": false,
            "is_archived": false,
            "url": ""
        })
    }

    fn secret_json(name: &str) -> serde_json::Value {
        serde_json::json!({
            "name": name,
            "data": "secret-value",
            "description": "",
            "created": "2026-01-15T10:00:00Z"
        })
    }

    fn org_action_variable_json(owner_id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "owner_id": owner_id,
            "repo_id": 0,
            "name": name,
            "data": "var-value",
            "description": ""
        })
    }

    fn activity_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "act_user_id": 1,
            "act_user": null,
            "op_type": "create_repo",
            "content": "",
            "repo_id": 1,
            "repo": null,
            "comment_id": 0,
            "comment": null,
            "ref_name": "",
            "is_private": false,
            "user_id": 1,
            "created": "2026-01-15T10:00:00Z"
        })
    }

    fn org_permissions_json() -> serde_json::Value {
        serde_json::json!({
            "can_create_repository": true,
            "can_read": true,
            "can_write": false,
            "is_admin": false,
            "is_owner": true
        })
    }

    fn make_minimal_repo_json() -> serde_json::Value {
        let mut repo_json: serde_json::Value =
            serde_json::from_str(include_str!("../../tests/fixtures/repository.json")).unwrap();
        if let serde_json::Value::Object(map) = &mut repo_json {
            map.insert("owner".to_string(), serde_json::Value::Null);
            map.insert("template".to_string(), serde_json::Value::Bool(false));
            map.insert("mirror".to_string(), serde_json::Value::Bool(false));
            map.insert("size".to_string(), serde_json::Value::from(0));
            map.insert(
                "language".to_string(),
                serde_json::Value::String(String::new()),
            );
            map.insert(
                "languages_url".to_string(),
                serde_json::Value::String(
                    "https://example.com/api/v1/repos/test/languages".to_string(),
                ),
            );
            map.insert(
                "url".to_string(),
                serde_json::Value::String(
                    "https://example.com/api/v1/repos/testuser/test-repo".to_string(),
                ),
            );
            map.insert("link".to_string(), serde_json::Value::String(String::new()));
            map.insert(
                "original_url".to_string(),
                serde_json::Value::String(String::new()),
            );
            map.insert(
                "website".to_string(),
                serde_json::Value::String(String::new()),
            );
            map.insert("stars_count".to_string(), serde_json::Value::from(0));
            map.insert("forks_count".to_string(), serde_json::Value::from(0));
            map.insert("watchers_count".to_string(), serde_json::Value::from(0));
            map.insert("open_issues_count".to_string(), serde_json::Value::from(0));
            map.insert("open_pr_counter".to_string(), serde_json::Value::from(0));
            map.insert("release_counter".to_string(), serde_json::Value::from(0));
            map.insert("archived".to_string(), serde_json::Value::Bool(false));
            map.insert(
                "archived_at".to_string(),
                serde_json::Value::String("2026-01-01T00:00:00Z".to_string()),
            );
            map.insert(
                "mirror_interval".to_string(),
                serde_json::Value::String(String::new()),
            );
            map.insert(
                "mirror_updated".to_string(),
                serde_json::Value::String("2026-01-15T10:30:00Z".to_string()),
            );
            map.insert("repo_transfer".to_string(), serde_json::Value::Null);
            map.insert("permissions".to_string(), serde_json::Value::Null);
            map.insert("has_issues".to_string(), serde_json::Value::Bool(true));
            map.insert("has_code".to_string(), serde_json::Value::Bool(true));
            map.insert("internal_tracker".to_string(), serde_json::Value::Null);
            map.insert("external_tracker".to_string(), serde_json::Value::Null);
            map.insert("has_wiki".to_string(), serde_json::Value::Bool(true));
            map.insert("external_wiki".to_string(), serde_json::Value::Null);
            map.insert(
                "has_pull_requests".to_string(),
                serde_json::Value::Bool(true),
            );
            map.insert("has_projects".to_string(), serde_json::Value::Bool(false));
            map.insert("has_releases".to_string(), serde_json::Value::Bool(true));
            map.insert("has_packages".to_string(), serde_json::Value::Bool(false));
            map.insert("has_actions".to_string(), serde_json::Value::Bool(false));
            map.insert(
                "ignore_whitespace_conflicts".to_string(),
                serde_json::Value::Bool(false),
            );
            map.insert(
                "allow_merge_commits".to_string(),
                serde_json::Value::Bool(true),
            );
            map.insert("allow_rebase".to_string(), serde_json::Value::Bool(true));
            map.insert(
                "allow_rebase_explicit".to_string(),
                serde_json::Value::Bool(true),
            );
            map.insert(
                "allow_rebase_update".to_string(),
                serde_json::Value::Bool(true),
            );
            map.insert(
                "allow_squash_merge".to_string(),
                serde_json::Value::Bool(true),
            );
            map.insert(
                "allow_fast_forward_only_merge".to_string(),
                serde_json::Value::Bool(false),
            );
            map.insert(
                "default_allow_maintainer_edit".to_string(),
                serde_json::Value::Bool(true),
            );
            map.insert(
                "default_delete_branch_after_merge".to_string(),
                serde_json::Value::Bool(false),
            );
            map.insert(
                "default_merge_style".to_string(),
                serde_json::Value::String("merge".to_string()),
            );
            map.insert(
                "avatar_url".to_string(),
                serde_json::Value::String(String::new()),
            );
            map.insert("internal".to_string(), serde_json::Value::Bool(false));
            map.insert(
                "mirror_updated_unix".to_string(),
                serde_json::Value::from(0),
            );
            map.insert("projects_mode".to_string(), serde_json::Value::Null);
            map.insert(
                "created_at".to_string(),
                serde_json::Value::String("2026-01-01T00:00:00Z".to_string()),
            );
            map.insert(
                "updated_at".to_string(),
                serde_json::Value::String("2026-01-15T10:30:00Z".to_string()),
            );
            map.insert(
                "object_format_name".to_string(),
                serde_json::Value::String(String::new()),
            );
            map.insert("topics".to_string(), serde_json::Value::Array(vec![]));
            map.insert("licenses".to_string(), serde_json::Value::Array(vec![]));
        }
        repo_json
    }

    // ── list_orgs ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_orgs_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_json(1, "org1"), org_json(2, "org2")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (orgs, resp) = client.orgs().list_orgs(Default::default()).await.unwrap();
        assert_eq!(orgs.len(), 2);
        assert_eq!(orgs[0].name, "org1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_orgs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(json!({"message": "Internal Server Error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_orgs(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_my_orgs ─────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_orgs_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_json(1, "myorg")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/user/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (orgs, resp) = client
            .orgs()
            .list_my_orgs(Default::default())
            .await
            .unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].name, "myorg");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_orgs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/orgs"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_my_orgs(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_user_orgs ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_user_orgs_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_json(1, "userorg")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (orgs, resp) = client
            .orgs()
            .list_user_orgs("testuser", Default::default())
            .await
            .unwrap();
        assert_eq!(orgs.len(), 1);
        assert_eq!(orgs[0].name, "userorg");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_orgs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_user_orgs("testuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_user_orgs_empty_user() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.orgs().list_user_orgs("", Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_org ──────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(200).set_body_json(org_json(1, "testorg")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (org, resp) = client.orgs().get_org("testorg").await.unwrap();
        assert_eq!(org.name, "testorg");
        assert_eq!(org.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org("testorg").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_org_empty_name() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org("").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("path segment [0] is empty")
        );
    }

    // ── create_org ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs"))
            .respond_with(ResponseTemplate::new(201).set_body_json(org_json(3, "neworg")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            full_name: Some("New Org".to_string()),
            ..Default::default()
        };
        let (org, resp) = client.orgs().create_org(opt).await.unwrap();
        assert_eq!(org.name, "neworg");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs"))
            .respond_with(
                ResponseTemplate::new(422).set_body_json(json!({"message": "validation failed"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: "neworg".to_string(),
            ..Default::default()
        };
        let result = client.orgs().create_org(opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_validation_empty_name() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgOption {
            name: String::new(),
            ..Default::default()
        };
        let result = client.orgs().create_org(opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("org name is required")
        );
    }

    // ── edit_org ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgOption {
            description: Some("updated".to_string()),
            ..Default::default()
        };
        let resp = client.orgs().edit_org("testorg", opt).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgOption {
            description: Some("updated".to_string()),
            ..Default::default()
        };
        let result = client.orgs().edit_org("testorg", opt).await;
        assert!(result.is_err());
    }

    // ── delete_org ───────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_org("testorg").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_org("testorg").await;
        assert!(result.is_err());
    }

    // ── list_org_teams ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_teams_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([team_json(1, "owners"), team_json(2, "devs")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (teams, resp) = client
            .orgs()
            .list_org_teams("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(teams.len(), 2);
        assert_eq!(teams[0].name, "owners");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_teams_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_teams("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_my_teams ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_teams_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([team_json(1, "myteam")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/user/teams"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (teams, resp) = client
            .orgs()
            .list_my_teams(Default::default())
            .await
            .unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "myteam");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_teams_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/teams"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_my_teams(Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_team ─────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(200).set_body_json(team_json(5, "devs")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (team, resp) = client.orgs().get_team(5).await.unwrap();
        assert_eq!(team.id, 5);
        assert_eq!(team.name, "devs");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_team(999).await;
        assert!(result.is_err());
    }

    // ── search_org_teams ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_search_org_teams_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": true,
            "error": "",
            "data": [team_json(1, "search-team")]
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchTeamsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let (teams, resp) = client
            .orgs()
            .search_org_teams("testorg", opt)
            .await
            .unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "search-team");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_org_teams_error_not_ok() {
        let server = MockServer::start().await;
        let body = serde_json::json!({
            "ok": false,
            "error": "search failed",
            "data": []
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchTeamsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let result = client.orgs().search_org_teams("testorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_search_org_teams_error_http() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/teams/search"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(json!({"message": "Internal Server Error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = SearchTeamsOptions {
            query: "search".to_string(),
            ..Default::default()
        };
        let result = client.orgs().search_org_teams("testorg", opt).await;
        assert!(result.is_err());
    }

    // ── create_team ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(201).set_body_json(team_json(10, "newteam")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateTeamOption {
            name: "newteam".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let (team, resp) = client.orgs().create_team("testorg", opt).await.unwrap();
        assert_eq!(team.name, "newteam");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/teams"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateTeamOption {
            name: "newteam".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let result = client.orgs().create_team("testorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_team_validation_empty_name() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateTeamOption {
            name: String::new(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let result = client.orgs().create_team("testorg", opt).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name required"));
    }

    // ── edit_team ────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditTeamOption {
            name: "renamed".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let resp = client.orgs().edit_team(5, opt).await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditTeamOption {
            name: "renamed".to_string(),
            description: None,
            permission: None,
            can_create_org_repo: None,
            includes_all_repositories: None,
            units: vec![],
        };
        let result = client.orgs().edit_team(5, opt).await;
        assert!(result.is_err());
    }

    // ── delete_team ──────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_team_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_team(5).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_team_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_team(5).await;
        assert!(result.is_err());
    }

    // ── list_team_members ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_team_members_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "user1"), user_json(2, "user2")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (members, resp) = client
            .orgs()
            .list_team_members(5, Default::default())
            .await
            .unwrap();
        assert_eq!(members.len(), 2);
        assert_eq!(members[0].user_name, "user1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_team_members_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().list_team_members(5, Default::default()).await;
        assert!(result.is_err());
    }

    // ── get_team_member ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_team_member_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json(1, "testuser")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (user, resp) = client.orgs().get_team_member(5, "testuser").await.unwrap();
        assert_eq!(user.user_name, "testuser");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_team_member_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_team_member(5, "testuser").await;
        assert!(result.is_err());
    }

    // ── add_team_member ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_add_team_member_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().add_team_member(5, "testuser").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_team_member_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().add_team_member(5, "testuser").await;
        assert!(result.is_err());
    }

    // ── remove_team_member ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_remove_team_member_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .remove_team_member(5, "testuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_remove_team_member_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/members/testuser"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().remove_team_member(5, "testuser").await;
        assert!(result.is_err());
    }

    // ── add_team_repo ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_add_team_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .add_team_repo(5, "myorg", "myrepo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_team_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().add_team_repo(5, "myorg", "myrepo").await;
        assert!(result.is_err());
    }

    // ── remove_team_repo ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_remove_team_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .remove_team_repo(5, "myorg", "myrepo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_remove_team_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/teams/5/repos/myorg/myrepo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().remove_team_repo(5, "myorg", "myrepo").await;
        assert!(result.is_err());
    }

    // ── delete_org_membership ────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_membership_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/members/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .delete_org_membership("testorg", "someuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_membership_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/members/someuser"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .delete_org_membership("testorg", "someuser")
            .await;
        assert!(result.is_err());
    }

    // ── list_org_membership ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_membership_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "member1")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (members, resp) = client
            .orgs()
            .list_org_membership("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_name, "member1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_membership_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_membership("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_public_org_membership ───────────────────────────────────────

    #[tokio::test]
    async fn test_list_public_org_membership_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "pubmember")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (members, resp) = client
            .orgs()
            .list_public_org_membership("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].user_name, "pubmember");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_public_org_membership_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_public_org_membership("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── check_org_membership ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_check_org_membership_is_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/exists"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_org_membership("testorg", "exists")
            .await
            .unwrap();
        assert!(is_member);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_org_membership_not_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/notexists"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_org_membership("testorg", "notexists")
            .await
            .unwrap();
        assert!(!is_member);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_check_org_membership_error_unexpected() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .check_org_membership("testorg", "someuser")
            .await;
        assert!(result.is_err());
    }

    // ── check_public_org_membership ──────────────────────────────────────

    #[tokio::test]
    async fn test_check_public_org_membership_is_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members/exists"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_public_org_membership("testorg", "exists")
            .await
            .unwrap();
        assert!(is_member);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_public_org_membership_not_member() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members/notexists"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_member, resp) = client
            .orgs()
            .check_public_org_membership("testorg", "notexists")
            .await
            .unwrap();
        assert!(!is_member);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_check_public_org_membership_error_unexpected() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .check_public_org_membership("testorg", "someuser")
            .await;
        assert!(result.is_err());
    }

    // ── set_public_org_membership ────────────────────────────────────────

    #[tokio::test]
    async fn test_set_public_org_membership_publicize() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .set_public_org_membership("testorg", "someuser", true)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_set_public_org_membership_conceal() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .set_public_org_membership("testorg", "someuser", false)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_set_public_org_membership_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/public_members/someuser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .set_public_org_membership("testorg", "someuser", true)
            .await;
        assert!(result.is_err());
    }

    // ── get_org_permissions ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_permissions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs/testorg/permissions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(org_permissions_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (perms, resp) = client
            .orgs()
            .get_org_permissions("testorg", "testuser")
            .await
            .unwrap();
        assert!(perms.is_owner);
        assert!(perms.can_create_repository);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_permissions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/orgs/testorg/permissions"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .get_org_permissions("testorg", "testuser")
            .await;
        assert!(result.is_err());
    }

    // ── list_org_labels ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_labels_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([label_json(1, "bug")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (labels, resp) = client
            .orgs()
            .list_org_labels("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_labels("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── create_org_label ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(201).set_body_json(label_json(10, "feature")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: None,
            exclusive: None,
        };
        let (label, resp) = client
            .orgs()
            .create_org_label("testorg", opt)
            .await
            .unwrap();
        assert_eq!(label.name, "feature");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/labels"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: None,
            exclusive: None,
        };
        let result = client.orgs().create_org_label("testorg", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_label_validation_invalid_color() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateOrgLabelOption {
            name: "badlabel".to_string(),
            color: "not-a-color".to_string(),
            description: None,
            exclusive: None,
        };
        let result = client.orgs().create_org_label("testorg", opt).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("invalid color format")
        );
    }

    // ── get_org_label ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(label_json(42, "bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (label, resp) = client.orgs().get_org_label("testorg", 42).await.unwrap();
        assert_eq!(label.id, 42);
        assert_eq!(label.name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/labels/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org_label("testorg", 999).await;
        assert!(result.is_err());
    }

    // ── edit_org_label ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(label_json(42, "updated-bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgLabelOption {
            name: Some("updated-bug".to_string()),
            color: Some("0000ff".to_string()),
            ..Default::default()
        };
        let (label, resp) = client
            .orgs()
            .edit_org_label("testorg", 42, opt)
            .await
            .unwrap();
        assert_eq!(label.name, "updated-bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditOrgLabelOption {
            name: Some("updated".to_string()),
            ..Default::default()
        };
        let result = client.orgs().edit_org_label("testorg", 42, opt).await;
        assert!(result.is_err());
    }

    // ── delete_org_label ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_org_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.orgs().delete_org_label("testorg", 42).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_org_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/labels/42"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().delete_org_label("testorg", 42).await;
        assert!(result.is_err());
    }

    // ── list_org_action_secrets ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_action_secrets_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([secret_json("MY_SECRET")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/secrets"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (secrets, resp) = client
            .orgs()
            .list_org_action_secrets("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(secrets.len(), 1);
        assert_eq!(secrets[0].name, "MY_SECRET");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_action_secrets_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/secrets"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_action_secrets("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_org_action_variables ────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_action_variables_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([org_action_variable_json(1, "MY_VAR")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (vars, resp) = client
            .orgs()
            .list_org_action_variables("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "MY_VAR");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_action_variables_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_action_variables("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_org_action_variable ──────────────────────────────────────────

    #[tokio::test]
    async fn test_get_org_action_variable_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(org_action_variable_json(1, "MY_VAR")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (var, resp) = client
            .orgs()
            .get_org_action_variable("testorg", "MY_VAR")
            .await
            .unwrap();
        assert_eq!(var.name, "MY_VAR");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_org_action_variable_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MISSING"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .get_org_action_variable("testorg", "MISSING")
            .await;
        assert!(result.is_err());
    }

    // ── create_org_action_variable ───────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_action_variable_happy_201() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_action_variable_happy_204() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_create_org_action_variable_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let result = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_org_action_variable_error_bad_request() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY_VAR"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateOrgActionVariableOption {
            name: "MY_VAR".to_string(),
            value: "my-value".to_string(),
            description: None,
        };
        let result = client
            .orgs()
            .create_org_action_variable("testorg", opt)
            .await;
        assert!(result.is_err());
    }

    // ── update_org_action_variable ───────────────────────────────────────

    #[tokio::test]
    async fn test_update_org_action_variable_happy_200() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateOrgActionVariableOption {
            value: "updated-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .update_org_action_variable("testorg", "MY_VAR", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_org_action_variable_happy_204() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateOrgActionVariableOption {
            value: "updated-value".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .update_org_action_variable("testorg", "MY_VAR", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_update_org_action_variable_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/variables/MY%5FVAR"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateOrgActionVariableOption {
            value: "updated-value".to_string(),
            description: None,
        };
        let result = client
            .orgs()
            .update_org_action_variable("testorg", "MY_VAR", opt)
            .await;
        assert!(result.is_err());
    }

    // ── create_org_action_secret ─────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_action_secret_happy_201() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_secret("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_action_secret_happy_204() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let resp = client
            .orgs()
            .create_org_action_secret("testorg", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_create_org_action_secret_error_forbidden() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let result = client.orgs().create_org_action_secret("testorg", opt).await;
        assert!(result.is_err());
    }

    // ── list_org_blocks ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_blocks_happy() {
        let server = MockServer::start().await;
        let body = serde_json::json!([user_json(1, "baduser")]);
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (users, resp) = client
            .orgs()
            .list_org_blocks("testorg", Default::default())
            .await
            .unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(users[0].user_name, "baduser");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_blocks_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_org_blocks("testorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── check_org_block ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_check_org_block_is_blocked() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_blocked, resp) = client
            .orgs()
            .check_org_block("testorg", "baduser")
            .await
            .unwrap();
        assert!(is_blocked);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_check_org_block_not_blocked() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/blocks/gooduser"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (is_blocked, resp) = client
            .orgs()
            .check_org_block("testorg", "gooduser")
            .await
            .unwrap();
        assert!(!is_blocked);
        assert_eq!(resp.status, 404);
    }

    // ── block_org_user ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_block_org_user_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .block_org_user("testorg", "baduser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_block_org_user_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().block_org_user("testorg", "baduser").await;
        assert!(result.is_err());
    }

    // ── unblock_org_user ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_unblock_org_user_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .orgs()
            .unblock_org_user("testorg", "baduser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unblock_org_user_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.orgs().unblock_org_user("testorg", "baduser").await;
        assert!(result.is_err());
    }

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

    // ── list_team_repositories ───────────────────────────────────────────

    #[tokio::test]
    async fn test_list_team_repositories_happy() {
        let server = MockServer::start().await;
        let repo_json = make_minimal_repo_json();
        let body = serde_json::json!([repo_json]);
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/repos"))
            .and(query_param("page", "1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .orgs()
            .list_team_repositories(5, Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_team_repositories_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/teams/5/repos"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .orgs()
            .list_team_repositories(5, Default::default())
            .await;
        assert!(result.is_err());
    }
}
