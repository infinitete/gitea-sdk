// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::collections::HashMap;

use crate::Client;
use crate::Response;
use crate::options::org::CreateSecretOption;
use crate::options::repo::*;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::repository::*;
use crate::types::{Label, Secret, Team, User};
use crate::version::{
    VERSION_1_12_0, VERSION_1_13_0, VERSION_1_14_0, VERSION_1_15_0, VERSION_1_16_0, VERSION_1_22_0,
    VERSION_1_23_0,
};

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

/// API methods for repository operations.
pub struct ReposApi<'a> {
    client: &'a Client,
}

impl<'a> ReposApi<'a> {
    /// Create a new `ReposApi` for the given client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get a reference to the underlying client.
    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    // ── repo.go (16 methods) ─────────────────────────────────────

    /// ListMyRepos list all repositories of the authenticated user
    pub async fn list_my_repos(
        &self,
        opt: ListReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let path = format!("/user/repos?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListUserRepos list repositories of a user
    pub async fn list_user_repos(
        &self,
        user: &str,
        opt: ListReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/repos?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListOrgRepos list repositories of an organization
    pub async fn list_org_repos(
        &self,
        org: &str,
        opt: ListOrgReposOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        let path = format!("/orgs/{}/repos?{}", escaped[0], opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// SearchRepos search for repositories
    pub async fn search_repos(
        &self,
        opt: SearchRepoOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let path = format!("/repos/search?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateRepo create a repository
    pub async fn create_repo(
        &self,
        opt: CreateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/user/repos",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// CreateOrgRepo create a repository in an organization
    pub async fn create_org_repo(
        &self,
        org: &str,
        opt: CreateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[org])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/orgs/{}/repos", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// GetRepo get a repository
    pub async fn get_repo(&self, owner: &str, repo: &str) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRepoByID get a repository by id
    pub async fn get_repo_by_id(&self, id: i64) -> crate::Result<(Repository, Response)> {
        let path = format!("/repositories/{id}");
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// EditRepo edit repository properties
    pub async fn edit_repo(
        &self,
        owner: &str,
        repo: &str,
        opt: EditRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteRepo delete a repository
    pub async fn delete_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// MirrorSync sync a mirror repository
    pub async fn mirror_sync(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/mirror-sync", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// GetRepoLanguages get languages of a repository
    pub async fn get_repo_languages(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(HashMap<String, i64>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/languages", escaped[0], escaped[1]);
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        let langs: HashMap<String, i64> = serde_json::from_slice(&data)?;
        Ok((langs, resp))
    }

    /// GetArchive get an archive of a repository
    pub async fn get_archive(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        archive: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/archive/{}.{}",
            escaped[0], escaped[1], ref_, archive
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetArchiveReader get an archive streaming reader of a repository
    pub async fn get_archive_reader(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
        archive: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/archive/{}.{}",
            escaped[0], escaped[1], ref_, archive
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// UpdateRepoAvatar update the avatar of a repository
    pub async fn update_repo_avatar(
        &self,
        owner: &str,
        repo: &str,
        file_content: &[u8],
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/avatar", escaped[0], escaped[1]);
        let form = reqwest::multipart::Form::new().part(
            "avatar",
            reqwest::multipart::Part::bytes(file_content.to_vec()).file_name("avatar".to_string()),
        );
        self.client()
            .get_parsed_response_multipart(reqwest::Method::PUT, &path, None, form)
            .await
    }

    /// DeleteRepoAvatar delete the avatar of a repository
    pub async fn delete_repo_avatar(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/avatar", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_branch.go (5 methods) ────────────────────────────────

    /// ListBranches list a repository's branches
    pub async fn list_branches(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoBranchesOptions,
    ) -> crate::Result<(Vec<Branch>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/branches?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetBranch get a single branch of a repository
    pub async fn get_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::Result<(Branch, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, branch])?;
        let path = format!(
            "/repos/{}/{}/branches/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeleteBranch delete a branch from a repository
    pub async fn delete_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, branch])?;
        let path = format!(
            "/repos/{}/{}/branches/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// UpdateBranch rename a branch in a repository
    pub async fn update_branch(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        opt: UpdateRepoBranchOption,
    ) -> crate::Result<(Branch, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_23_0)
            .await?;
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, branch])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/branches/{}",
            escaped[0], escaped[1], escaped[2]
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

    /// CreateBranch create a branch in a repository
    pub async fn create_branch(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateBranchOption,
    ) -> crate::Result<(Branch, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_13_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/branches", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── repo_tag.go (5 methods) ───────────────────────────────────

    /// ListTags list a repository's tags
    pub async fn list_tags(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoTagsOptions,
    ) -> crate::Result<(Vec<Tag>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/tags?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTag get a single tag of a repository
    pub async fn get_tag(
        &self,
        owner: &str,
        repo: &str,
        tag: &str,
    ) -> crate::Result<(Tag, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, tag])?;
        let path = format!("/repos/{}/{}/tags/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetAnnotatedTag get an annotated tag of a repository
    pub async fn get_annotated_tag(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::Result<(AnnotatedTag, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/tags/{}", escaped[0], escaped[1], sha);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateTag create a tag in a repository
    pub async fn create_tag(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateTagOption,
    ) -> crate::Result<(Tag, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tags", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteTag delete a tag from a repository
    pub async fn delete_tag(&self, owner: &str, repo: &str, tag: &str) -> crate::Result<Response> {
        self.client()
            .check_server_version_ge(&VERSION_1_14_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, tag])?;
        let path = format!("/repos/{}/{}/tags/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_file.go (7 methods) ──────────────────────────────────

    /// GetFile download a file from a repository
    pub async fn get_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/contents/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetFileReader get a streaming reader for a file from a repository
    pub async fn get_file_reader(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/raw/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetContents get the metadata and contents of a file in a repository
    pub async fn get_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(ContentsResponse, Response)> {
        let (data, resp) = self.get_contents_raw(owner, repo, filepath, ref_).await?;
        let cr: ContentsResponse = serde_json::from_slice(&data)?;
        Ok((cr, resp))
    }

    /// ListContents get a list of entries in a directory
    pub async fn list_contents(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<ContentsResponse>, Response)> {
        let (data, resp) = self.get_contents_raw(owner, repo, filepath, ref_).await?;
        let crl: Vec<ContentsResponse> = serde_json::from_slice(&data)?;
        Ok((crl, resp))
    }

    /// CreateFile create a file in a repository
    pub async fn create_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        opt: CreateFileOptions,
    ) -> crate::Result<(FileResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/contents/{}",
            escaped[0], escaped[1], escaped_path
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

    /// UpdateFile update a file in a repository
    pub async fn update_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        opt: UpdateFileOptions,
    ) -> crate::Result<(FileResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/contents/{}",
            escaped[0], escaped[1], escaped_path
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteFile delete a file from a repository
    pub async fn delete_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        opt: DeleteFileOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/contents/{}",
            escaped[0], escaped[1], escaped_path
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

    // ── repo_file_ext.go (4 methods) ──────────────────────────────

    /// GetContentsExt get extended contents of a repository
    pub async fn get_contents_ext(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
        opt: GetContentsExtOptions,
    ) -> crate::Result<(ContentsExtResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let mut qs = format!(
            "ref={}&includes={}",
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC),
            percent_encoding::utf8_percent_encode(
                &opt.includes,
                percent_encoding::NON_ALPHANUMERIC
            ),
        );
        if !opt.r#ref.is_empty() {
            qs = format!(
                "ref={}&includes={}",
                percent_encoding::utf8_percent_encode(
                    &opt.r#ref,
                    percent_encoding::NON_ALPHANUMERIC
                ),
                percent_encoding::utf8_percent_encode(
                    &opt.includes,
                    percent_encoding::NON_ALPHANUMERIC
                ),
            );
        }
        let path = format!(
            "/repos/{}/{}/contents/{}?{}",
            escaped[0], escaped[1], escaped_path, qs
        );
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        let ext: ContentsExtResponse = serde_json::from_slice(&data)?;
        Ok((ext, resp))
    }

    /// GetEditorConfig get the editorconfig of a repository
    pub async fn get_editor_config(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/editorconfig/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRawFileOrLFS get raw file from a repository, following LFS redirects
    pub async fn get_raw_file_or_lfs(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/raw/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRawFile get raw file from a repository
    pub async fn get_raw_file(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/raw/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_collaborator.go (7 methods) ──────────────────────────

    /// ListCollaborators list a repository's collaborators
    pub async fn list_collaborators(
        &self,
        owner: &str,
        repo: &str,
        opt: ListCollaboratorsOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/collaborators?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// IsCollaborator check if a user is a collaborator of a repository
    pub async fn is_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        Ok((status == 204, resp))
    }

    /// GetCollaboratorPermission get collaborator permission of a repository
    pub async fn get_collaborator_permission(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::Result<(Option<CollaboratorPermissionResult>, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}/permission",
            escaped[0], escaped[1], escaped[2]
        );
        match self
            .client()
            .get_parsed_response::<CollaboratorPermissionResult, _>(
                reqwest::Method::GET,
                &path,
                None,
                None::<&str>,
            )
            .await
        {
            Ok((result, resp)) => Ok((Some(result), resp)),
            Err(e) => {
                if let crate::Error::Api { status, .. } = &e
                    && *status == 404
                {
                    return Ok((
                        None,
                        Response {
                            status: *status,
                            headers: reqwest::header::HeaderMap::new(),
                            page_links: None,
                        },
                    ));
                }
                Err(e)
            }
        }
    }

    /// AddCollaborator add a user as a collaborator of a repository
    pub async fn add_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
        mut opt: AddCollaboratorOption,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteCollaborator remove a collaborator from a repository
    pub async fn delete_collaborator(
        &self,
        owner: &str,
        repo: &str,
        collaborator: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, collaborator])?;
        let path = format!(
            "/repos/{}/{}/collaborators/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// GetReviewers get all users that can be requested to review in this repo
    pub async fn get_reviewers(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<User>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/reviewers", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetAssignees get all users that have write access and can be assigned to issues
    pub async fn get_assignees(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<User>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/assignees", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_commit.go (4 methods) ────────────────────────────────

    /// GetSingleCommit get a single commit of a repository
    pub async fn get_single_commit(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Commit, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/commits/{}", escaped[0], escaped[1], ref_);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListCommits list commits of a repository
    pub async fn list_commits(
        &self,
        owner: &str,
        repo: &str,
        opt: ListCommitOptions,
    ) -> crate::Result<(Vec<Commit>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/commits?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetCommitDiff get the diff of a commit
    pub async fn get_commit_diff(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_16_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/git/commits/{}.diff",
            escaped[0], escaped[1], ref_
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetCommitPatch get the patch of a commit
    pub async fn get_commit_patch(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_16_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/git/commits/{}.patch",
            escaped[0], escaped[1], ref_
        );
        self.client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

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
        id: i64,
    ) -> crate::Result<(PushMirrorResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/push_mirrors/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeletePushMirror delete a push mirror of a repository
    pub async fn delete_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/push_mirrors/{id}", escaped[0], escaped[1]);
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

    // ── repo_watch.go (5 methods) ─────────────────────────────────

    /// GetWatchedRepos list all the watched repos of user
    pub async fn get_watched_repos(
        &self,
        user: &str,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/subscriptions", escaped[0]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetMyWatchedRepos list repositories watched by the authenticated user
    pub async fn get_my_watched_repos(&self) -> crate::Result<(Vec<Repository>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/user/subscriptions",
                None,
                None::<&str>,
            )
            .await
    }

    /// CheckRepoWatch check if the current user is watching a repo
    pub async fn check_repo_watch(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/subscription", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        match status {
            200 => Ok((true, resp)),
            404 => Ok((false, resp)),
            _ => Err(crate::Error::Api {
                status,
                message: format!("unexpected status: {status}"),
                body: vec![],
            }),
        }
    }

    /// WatchRepo start to watch a repository
    pub async fn watch_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/subscription", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::PUT, &path, None, None::<&str>)
            .await?;
        if status == 200 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status: {status}"),
                body: vec![],
            })
        }
    }

    /// UnWatchRepo stop to watch a repository
    pub async fn unwatch_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/subscription", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await?;
        if status == 204 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status: {status}"),
                body: vec![],
            })
        }
    }

    // ── repo_stars.go (6 methods) ─────────────────────────────────

    /// ListStargazers list a repository's stargazers
    pub async fn list_stargazers(
        &self,
        owner: &str,
        repo: &str,
        opt: ListStargazersOptions,
    ) -> crate::Result<(Vec<User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/stargazers?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetStarredRepos list repos starred by a given user
    pub async fn get_starred_repos(
        &self,
        user: &str,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[user])?;
        let path = format!("/users/{}/starred", escaped[0]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// GetMyStarredRepos list repos starred by the authenticated user
    pub async fn get_my_starred_repos(&self) -> crate::Result<(Vec<Repository>, Response)> {
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                "/user/starred",
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// IsRepoStarring check if the authenticated user has starred the repo
    pub async fn is_repo_starring(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(bool, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/user/starred/{}/{}", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        match status {
            204 => Ok((true, resp)),
            404 => Ok((false, resp)),
            _ => Err(crate::Error::Api {
                status,
                message: format!("unexpected status code '{status}'"),
                body: vec![],
            }),
        }
    }

    /// StarRepo star a repository as the authenticated user
    pub async fn star_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/user/starred/{}/{}", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status == 204 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status code '{status}'"),
                body: vec![],
            })
        }
    }

    /// UnstarRepo remove star from a repository as the authenticated user
    pub async fn unstar_repo(&self, owner: &str, repo: &str) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/user/starred/{}/{}", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status == 204 {
            Ok(resp)
        } else {
            Err(crate::Error::Api {
                status,
                message: format!("unexpected status code '{status}'"),
                body: vec![],
            })
        }
    }

    // ── repo_wiki.go (6 methods) ──────────────────────────────────

    /// CreateWikiPage create a wiki page
    pub async fn create_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateWikiPageOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/wiki/new", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// GetWikiPage get a wiki page
    pub async fn get_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
    ) -> crate::Result<(WikiPage, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let path = format!(
            "/repos/{}/{}/wiki/page/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// EditWikiPage edit a wiki page
    pub async fn edit_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
        opt: CreateWikiPageOptions,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/wiki/page/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteWikiPage delete a wiki page
    pub async fn delete_wiki_page(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let path = format!(
            "/repos/{}/{}/wiki/page/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// ListWikiPages list wiki pages
    pub async fn list_wiki_pages(
        &self,
        owner: &str,
        repo: &str,
        opt: ListWikiPagesOptions,
    ) -> crate::Result<(Vec<WikiPageMetaData>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/wiki/pages?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetWikiRevisions get wiki page revisions
    pub async fn get_wiki_revisions(
        &self,
        owner: &str,
        repo: &str,
        page: &str,
        opt: ListWikiPageRevisionsOptions,
    ) -> crate::Result<(WikiCommitList, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, page])?;
        let path = format!(
            "/repos/{}/{}/wiki/revisions/{}?page={}",
            escaped[0], escaped[1], escaped[2], opt.page
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_tree.go (1 method) ───────────────────────────────────

    /// GetTrees get a git tree of a repository
    pub async fn get_trees(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        opt: ListTreeOptions,
    ) -> crate::Result<(GitTreeResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/git/trees/{}?{}",
            escaped[0],
            escaped[1],
            sha,
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_label.go (5 methods) ─────────────────────────────────

    /// ListLabels list repository's labels
    pub async fn list_labels(
        &self,
        owner: &str,
        repo: &str,
        opt: ListLabelsOptions,
    ) -> crate::Result<(Vec<Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/labels?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetLabel get a single label
    pub async fn get_label(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/labels/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateLabel create a label
    pub async fn create_label(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/labels", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditLabel edit a label
    pub async fn edit_label(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditLabelOption,
    ) -> crate::Result<(Label, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/labels/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteLabel delete a label
    pub async fn delete_label(&self, owner: &str, repo: &str, id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/labels/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_migrate.go (1 method) ────────────────────────────────

    /// MigrateRepo migrate a repository from an external service
    pub async fn migrate_repo(
        &self,
        opt: MigrateRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        opt.validate()?;
        let body = json_body(&opt)?;
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                "/repos/migrate",
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── repo_branch_protection.go (5 methods) ─────────────────────

    /// ListBranchProtections list branch protections
    pub async fn list_branch_protections(
        &self,
        owner: &str,
        repo: &str,
        opt: ListBranchProtectionsOptions,
    ) -> crate::Result<(Vec<BranchProtection>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_12_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/branch_protections?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetBranchProtection get a branch protection
    pub async fn get_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<(BranchProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/branch_protections/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateBranchProtection create a branch protection
    pub async fn create_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateBranchProtectionOption,
    ) -> crate::Result<(BranchProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/branch_protections", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditBranchProtection edit a branch protection
    pub async fn edit_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        opt: EditBranchProtectionOption,
    ) -> crate::Result<(BranchProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/branch_protections/{}",
            escaped[0], escaped[1], escaped[2]
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

    /// DeleteBranchProtection delete a branch protection
    pub async fn delete_branch_protection(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/branch_protections/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_tag_protection.go (5 methods) ────────────────────────

    /// ListTagProtections list tag protections
    pub async fn list_tag_protections(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoTagProtectionsOptions,
    ) -> crate::Result<(Vec<TagProtection>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_23_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/tag_protections?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetTagProtection get a tag protection
    pub async fn get_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateTagProtection create a tag protection
    pub async fn create_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateTagProtectionOption,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tag_protections", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditTagProtection edit a tag protection
    pub async fn edit_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditTagProtectionOption,
    ) -> crate::Result<(TagProtection, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteTagProtection delete a tag protection
    pub async fn delete_tag_protection(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/tag_protections/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_transfer.go (3 methods) ──────────────────────────────

    /// TransferRepo transfer a repository to a new owner
    pub async fn transfer_repo(
        &self,
        owner: &str,
        repo: &str,
        opt: TransferRepoOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/transfer", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// AcceptRepoTransfer accept a repository transfer
    pub async fn accept_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/transfer/accept", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// RejectRepoTransfer reject a repository transfer
    pub async fn reject_repo_transfer(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/transfer/reject", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    // ── repo_team.go (4 methods) ──────────────────────────────────

    /// GetRepoTeams get teams from a repository
    pub async fn get_repo_teams(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<Team>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/teams", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddRepoTeam add a team to a repository
    pub async fn add_repo_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::Result<Response> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, team])?;
        let path = format!("/repos/{}/{}/teams/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::PUT, &path, None, None::<&str>)
            .await
    }

    /// RemoveRepoTeam remove a team from a repository
    pub async fn remove_repo_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::Result<Response> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, team])?;
        let path = format!("/repos/{}/{}/teams/{}", escaped[0], escaped[1], escaped[2]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// CheckRepoTeam check if a team is assigned to a repository
    pub async fn check_repo_team(
        &self,
        owner: &str,
        repo: &str,
        team: &str,
    ) -> crate::Result<(Option<Team>, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_15_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, team])?;
        let path = format!("/repos/{}/{}/teams/{}", escaped[0], escaped[1], escaped[2]);
        match self
            .client()
            .get_parsed_response::<Team, _>(reqwest::Method::GET, &path, None, None::<&str>)
            .await
        {
            Ok((t, resp)) => Ok((Some(t), resp)),
            Err(e) => {
                if let crate::Error::Api { status, .. } = &e
                    && *status == 404
                {
                    return Ok((
                        None,
                        Response {
                            status: *status,
                            headers: reqwest::header::HeaderMap::new(),
                            page_links: None,
                        },
                    ));
                }
                Err(e)
            }
        }
    }

    // ── repo_key.go (4 methods) ───────────────────────────────────

    /// ListDeployKeys list deploy keys
    pub async fn list_deploy_keys(
        &self,
        owner: &str,
        repo: &str,
        opt: ListDeployKeysOptions,
    ) -> crate::Result<(Vec<DeployKey>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/keys?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetDeployKey get a deploy key
    pub async fn get_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(DeployKey, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/keys/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateDeployKey create a deploy key
    pub async fn create_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateKeyOption,
    ) -> crate::Result<(DeployKey, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/keys", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteDeployKey delete a deploy key
    pub async fn delete_deploy_key(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/keys/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── fork.go (2 methods) ───────────────────────────────────────

    /// ListForks list repository's forks
    pub async fn list_forks(
        &self,
        owner: &str,
        repo: &str,
        opt: ListForksOptions,
    ) -> crate::Result<(Vec<Repository>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/forks?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateFork create a fork of a repository
    pub async fn create_fork(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateForkOption,
    ) -> crate::Result<(Repository, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/forks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── git_blob.go (1 method) ────────────────────────────────────

    /// GetBlob get a blob of a repository
    pub async fn get_blob(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
    ) -> crate::Result<(GitBlobResponse, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/blobs/{}", escaped[0], escaped[1], sha);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── git_hook.go (4 methods) ───────────────────────────────────

    /// ListGitHooks list git hooks
    pub async fn list_git_hooks(
        &self,
        owner: &str,
        repo: &str,
        opt: ListRepoGitHooksOptions,
    ) -> crate::Result<(Vec<GitHook>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/hooks/git?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetGitHook get a git hook
    pub async fn get_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::Result<(GitHook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/hooks/git/{}", escaped[0], escaped[1], id);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// EditGitHook edit a git hook
    pub async fn edit_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
        opt: EditGitHookOption,
    ) -> crate::Result<(GitHook, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/hooks/git/{}", escaped[0], escaped[1], id);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteGitHook delete a git hook
    pub async fn delete_git_hook(
        &self,
        owner: &str,
        repo: &str,
        id: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/hooks/git/{}", escaped[0], escaped[1], id);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── repo_refs.go (3 methods) ──────────────────────────────────

    /// GetRepoRef get one ref's information of one repository
    pub async fn get_repo_ref(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Reference, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let ref_trimmed = ref_.trim_start_matches("refs/");
        let ref_escaped = crate::internal::escape::path_escape_segments(ref_trimmed);
        let path = format!(
            "/repos/{}/{}/git/refs/{}",
            escaped[0], escaped[1], ref_escaped
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetRepoRefs get list of ref's information of one repository
    pub async fn get_repo_refs(
        &self,
        owner: &str,
        repo: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<Reference>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let ref_trimmed = ref_.trim_start_matches("refs/");
        let ref_escaped = crate::internal::escape::path_escape_segments(ref_trimmed);
        let path = format!(
            "/repos/{}/{}/git/refs/{}",
            escaped[0], escaped[1], ref_escaped
        );
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        // Try array first, then single object
        if let Ok(refs) = serde_json::from_slice::<Vec<Reference>>(&data) {
            Ok((refs, resp))
        } else {
            let single: Reference = serde_json::from_slice(&data)?;
            Ok((vec![single], resp))
        }
    }

    /// ListAllGitRefs get all refs from a repository
    pub async fn list_all_git_refs(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<Reference>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/git/refs", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_compare.go (1 method) ────────────────────────────────

    /// CompareCommits compare two commits
    pub async fn compare_commits(
        &self,
        owner: &str,
        repo: &str,
        before: &str,
        after: &str,
    ) -> crate::Result<(Compare, Response)> {
        self.client()
            .check_server_version_ge(&VERSION_1_22_0)
            .await?;
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/compare/{}...{}",
            escaped[0], escaped[1], before, after
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── repo_git_notes.go (1 method) ──────────────────────────────

    /// GetRepoNote get a note for a specific commit
    pub async fn get_repo_note(
        &self,
        owner: &str,
        repo: &str,
        sha: &str,
        opt: GetRepoNoteOptions,
    ) -> crate::Result<(Note, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, sha])?;
        let mut qs = String::new();
        if let Some(v) = &opt.verification {
            qs.push_str(&format!(
                "verification={}",
                percent_encoding::utf8_percent_encode(
                    if *v { "true" } else { "false" },
                    percent_encoding::NON_ALPHANUMERIC,
                )
            ));
        }
        if let Some(v) = &opt.files {
            if !qs.is_empty() {
                qs.push('&');
            }
            qs.push_str(&format!(
                "files={}",
                percent_encoding::utf8_percent_encode(
                    if *v { "true" } else { "false" },
                    percent_encoding::NON_ALPHANUMERIC,
                )
            ));
        }
        let mut path = format!(
            "/repos/{}/{}/git/notes/{}",
            escaped[0], escaped[1], escaped[2]
        );
        if !qs.is_empty() {
            path = format!("{}?{}", path, qs);
        }
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        let note: Note = serde_json::from_slice(&data)?;
        Ok((note, resp))
    }

    // ── repo_action.go (8 methods) ────────────────────────────────

    /// ListActionSecrets list a repository's secrets
    pub async fn list_action_secrets(
        &self,
        owner: &str,
        repo: &str,
        opt: ListOptions,
    ) -> crate::Result<(Vec<Secret>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/secrets?{}",
            escaped[0],
            escaped[1],
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

    /// ListActionVariables list a repository's action variables
    pub async fn list_action_variables(
        &self,
        owner: &str,
        repo: &str,
        opt: ListOptions,
    ) -> crate::Result<(Vec<RepoActionVariable>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/variables?{}",
            escaped[0],
            escaped[1],
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

    /// CreateActionSecret create a secret for a repository
    pub async fn create_action_secret(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateSecretOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/actions/secrets/{}",
            escaped[0], escaped[1], opt.name
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteActionSecret delete a secret from a repository
    pub async fn delete_action_secret(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/secrets/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// GetActionVariable get a repository action variable
    pub async fn get_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<(RepoActionVariable, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateActionVariable create a repository action variable
    pub async fn create_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        value: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = serde_json::to_string(&serde_json::json!({"value": value}))?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// UpdateActionVariable update a repository action variable
    pub async fn update_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        value: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = serde_json::to_string(&serde_json::json!({"name": name, "value": value}))?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteActionVariable delete a repository action variable
    pub async fn delete_action_variable(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/actions/variables/{}",
            escaped[0], escaped[1], name
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── Internal helpers ──────────────────────────────────────────

    /// Internal: get raw bytes for contents endpoint
    async fn get_contents_raw(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let escaped_path =
            crate::internal::escape::path_escape_segments(filepath.trim_start_matches('/'));
        let path = format!(
            "/repos/{}/{}/contents/{}?ref={}",
            escaped[0],
            escaped[1],
            escaped_path,
            percent_encoding::utf8_percent_encode(ref_, percent_encoding::NON_ALPHANUMERIC)
        );
        self.client()
            .get_response(
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

    fn minimal_repo_json(id: i64, name: &str, owner_name: &str) -> serde_json::Value {
        let ts = "2024-01-01T00:00:00Z";
        let owner = serde_json::json!({
            "id": 1, "login": owner_name, "full_name": owner_name,
            "email": "", "login_name": "", "source_id": 0,
            "avatar_url": "", "html_url": "", "language": "",
            "is_admin": false, "restricted": false, "active": false,
            "prohibit_login": false, "location": "", "website": "",
            "description": "", "visibility": "public",
            "followers_count": 0, "following_count": 0, "starred_repos_count": 0,
        });
        let base = serde_json::json!({
            "id": id,
            "owner": owner,
            "name": name,
            "full_name": format!("{}/{}", owner_name, name),
            "default_branch": "main",
            "archived": false,
            "archived_at": ts,
            "created_at": ts,
            "updated_at": ts,
            "has_issues": true,
            "has_code": true,
            "has_wiki": true,
            "has_pull_requests": true,
            "default_merge_style": "merge",
            "object_format_name": "sha1",
        });
        let mut map = base.as_object().unwrap().clone();
        let extra: Vec<(String, serde_json::Value)> = vec![
            ("description".into(), serde_json::json!("")),
            ("empty".into(), serde_json::json!(true)),
            ("private".into(), serde_json::json!(false)),
            ("fork".into(), serde_json::json!(false)),
            ("template".into(), serde_json::json!(false)),
            ("mirror".into(), serde_json::json!(false)),
            ("size".into(), serde_json::json!(0)),
            ("language".into(), serde_json::json!("")),
            ("languages_url".into(), serde_json::json!("")),
            ("html_url".into(), serde_json::json!("")),
            ("url".into(), serde_json::json!("")),
            ("link".into(), serde_json::json!("")),
            ("ssh_url".into(), serde_json::json!("")),
            ("clone_url".into(), serde_json::json!("")),
            ("original_url".into(), serde_json::json!("")),
            ("website".into(), serde_json::json!("")),
            ("stars_count".into(), serde_json::json!(0)),
            ("forks_count".into(), serde_json::json!(0)),
            ("watchers_count".into(), serde_json::json!(0)),
            ("open_issues_count".into(), serde_json::json!(0)),
            ("open_pr_counter".into(), serde_json::json!(0)),
            ("release_counter".into(), serde_json::json!(0)),
            (
                "ignore_whitespace_conflicts".into(),
                serde_json::json!(false),
            ),
            (
                "allow_fast_forward_only_merge".into(),
                serde_json::json!(false),
            ),
            ("allow_merge_commits".into(), serde_json::json!(true)),
            ("allow_rebase".into(), serde_json::json!(true)),
            ("allow_rebase_explicit".into(), serde_json::json!(true)),
            ("allow_rebase_update".into(), serde_json::json!(false)),
            ("allow_squash_merge".into(), serde_json::json!(true)),
            (
                "default_allow_maintainer_edit".into(),
                serde_json::json!(false),
            ),
            ("has_projects".into(), serde_json::json!(true)),
            ("avatar_url".into(), serde_json::json!("")),
            ("internal".into(), serde_json::json!(false)),
            ("mirror_interval".into(), serde_json::json!("")),
            (
                "default_delete_branch_after_merge".into(),
                serde_json::json!(false),
            ),
        ];
        for (key, val) in extra {
            map.insert(key, val);
        }
        serde_json::Value::Object(map)
    }

    #[tokio::test]
    async fn test_get_repo() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_repo_json(
                1,
                "testrepo",
                "testowner",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (repo, resp) = client
            .repos()
            .get_repo("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(repo.id, 1);
        assert_eq!(repo.name, "testrepo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_repo() {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/v1/user/repos"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(minimal_repo_json(2, "newrepo", "testuser")),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = CreateRepoOption {
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            issue_labels: String::new(),
            auto_init: false,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: crate::types::enums::TrustModel::Default,
            object_format_name: String::new(),
        };
        let (repo, resp) = client.repos().create_repo(opt).await.unwrap();
        assert_eq!(repo.id, 2);
        assert_eq!(repo.name, "newrepo");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_list_branches() {
        let server = MockServer::start().await;
        let body = serde_json::json!([
            {
                "name": "main",
                "protected": false,
                "required_approvals": 0,
                "enable_status_check": false,
                "status_check_contexts": [],
                "user_can_push": true,
                "user_can_merge": true,
                "effective_branch_protection_name": ""
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/branches"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (branches, resp) = client
            .repos()
            .list_branches("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_file() {
        use wiremock::matchers::query_param;

        let server = MockServer::start().await;
        let body = serde_json::json!({
            "name": "README.md",
            "path": "README.md",
            "sha": "abc123",
            "type": "file",
            "size": 10,
            "encoding": "base64",
            "content": "SGVsbG8=",
            "last_commit_sha": "def456"
        });

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/contents/README%2Emd",
            ))
            .and(query_param("ref", "main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_file("testowner", "testrepo", "README.md", "main")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_languages() {
        let server = MockServer::start().await;
        let body = serde_json::json!({"Go": 1000, "Rust": 500});

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/languages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (langs, resp) = client
            .repos()
            .get_repo_languages("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(langs.get("Go"), Some(&1000));
        assert_eq!(langs.get("Rust"), Some(&500));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_is_collaborator_true() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/collaborators/testuser",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (is_collab, resp) = client
            .repos()
            .is_collaborator("testowner", "testrepo", "testuser")
            .await
            .unwrap();
        assert!(is_collab);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_is_collaborator_false() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/testowner/testrepo/collaborators/nonuser",
            ))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (is_collab, resp) = client
            .repos()
            .is_collaborator("testowner", "testrepo", "nonuser")
            .await
            .unwrap();
        assert!(!is_collab);
        assert_eq!(resp.status, 404);
    }

    #[tokio::test]
    async fn test_star_repo() {
        let server = MockServer::start().await;

        Mock::given(method("PUT"))
            .and(path("/api/v1/user/starred/testowner/testrepo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let resp = client
            .repos()
            .star_repo("testowner", "testrepo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_error_case() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/nonrepo"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({"message": "Repository not found"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client.repos().get_repo("testowner", "nonrepo").await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Repository not found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }
}
