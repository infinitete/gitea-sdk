// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::org::*;
use crate::pagination::QueryEncode;
use crate::types::{Activity, Label, OrgPermissions, Organization, Secret, Team, User};
use crate::{Deserialize, Serialize};

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

    #[tokio::test]
    async fn test_get_org() {
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
    async fn test_list_orgs() {
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
    async fn test_create_org() {
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
    async fn test_delete_org() {
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
    async fn test_list_org_teams() {
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
    async fn test_check_org_membership() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/exists"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/testorg/members/notexists"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);

        let (is_member, _) = client
            .orgs()
            .check_org_membership("testorg", "exists")
            .await
            .unwrap();
        assert!(is_member);

        let (is_member, _) = client
            .orgs()
            .check_org_membership("testorg", "notexists")
            .await
            .unwrap();
        assert!(!is_member);
    }

    #[tokio::test]
    async fn test_list_org_labels() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "id": 1,
                "name": "bug",
                "color": "ff0000",
                "description": "",
                "exclusive": false,
                "is_archived": false,
                "url": ""
            }
        ]);

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
    async fn test_block_and_unblock_user() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/orgs/testorg/blocks/baduser"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        Mock::given(method("DELETE"))
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

        let resp = client
            .orgs()
            .unblock_org_user("testorg", "baduser")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_empty_org_name_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let result = client.orgs().get_org("").await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("path segment [0] is empty"));
    }

    #[tokio::test]
    async fn test_search_org_teams() {
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
    async fn test_edit_org() {
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
    async fn test_delete_org_membership() {
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
}
