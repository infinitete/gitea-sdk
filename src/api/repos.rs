// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Repository API endpoints for managing Gitea repositories.

use bytes::Bytes;

use std::collections::HashMap;

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::org::CreateSecretOption;
use crate::options::repo::*;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::repository::*;
use crate::types::{Label, Secret, Team, User};
use crate::version::{
    VERSION_1_12_0, VERSION_1_13_0, VERSION_1_14_0, VERSION_1_15_0, VERSION_1_16_0, VERSION_1_22_0,
    VERSION_1_23_0,
};

/// API methods for repositories. Access via [`Client::repos()`](crate::Client::repos).
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
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;

        if let Ok(repos) = serde_json::from_slice::<Vec<Repository>>(&data) {
            return Ok((repos, resp));
        }

        #[derive(serde::Deserialize)]
        struct SearchReposEnvelope {
            #[serde(default)]
            data: Vec<Repository>,
        }

        let wrapped: SearchReposEnvelope = serde_json::from_slice(&data)?;
        Ok((wrapped.data, resp))
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
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;

        if data.is_empty() {
            let (updated, _) = self.get_branch(owner, repo, &opt.name).await?;
            return Ok((updated, resp));
        }

        let updated: Branch = serde_json::from_slice(&data)?;
        Ok((updated, resp))
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
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
        let mut ext: ContentsExtResponse = serde_json::from_slice(&data)?;
        if ext.file_contents.is_none() && ext.dir_contents.is_none() {
            if let Ok(file_contents) = serde_json::from_slice::<ContentsResponse>(&data) {
                ext.file_contents = Some(file_contents);
            } else if let Ok(dir_contents) = serde_json::from_slice::<Vec<ContentsResponse>>(&data)
            {
                ext.dir_contents = Some(dir_contents);
            }
        }
        Ok((ext, resp))
    }

    /// GetEditorConfig get the editorconfig of a repository
    pub async fn get_editor_config(
        &self,
        owner: &str,
        repo: &str,
        filepath: &str,
        ref_: &str,
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
    ) -> crate::Result<(Bytes, Response)> {
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
        remote_name: &str,
    ) -> crate::Result<(PushMirrorResponse, Response)> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, remote_name])?;
        let path = format!(
            "/repos/{}/{}/push_mirrors/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeletePushMirror delete a push mirror of a repository
    pub async fn delete_push_mirror(
        &self,
        owner: &str,
        repo: &str,
        remote_name: &str,
    ) -> crate::Result<Response> {
        let escaped =
            crate::internal::escape::validate_and_escape_segments(&[owner, repo, remote_name])?;
        let path = format!(
            "/repos/{}/{}/push_mirrors/{}",
            escaped[0], escaped[1], escaped[2]
        );
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
        let (data, resp) = self
            .client()
            .get_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await?;
        if let Ok(single) = serde_json::from_slice::<Reference>(&data) {
            Ok((single, resp))
        } else {
            let mut refs: Vec<Reference> = serde_json::from_slice(&data)?;
            refs.pop()
                .map(|reference| (reference, resp))
                .ok_or_else(|| {
                    crate::Error::Json(serde_json::Error::io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "empty ref array response",
                    )))
                })
        }
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
    ) -> crate::Result<(Bytes, Response)> {
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
    use serde_json::json;
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
    async fn test_create_branch_version_gated() {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "version": "1.12.0"
            })))
            .mount(&server)
            .await;

        let client = Client::builder(&server.uri())
            .token("test-token")
            .build()
            .unwrap();
        let result = client
            .repos()
            .create_branch(
                "testowner",
                "testrepo",
                CreateBranchOption {
                    branch_name: "feature".to_string(),
                    old_branch_name: "main".to_string(),
                },
            )
            .await;

        assert!(matches!(result, Err(crate::Error::Version { .. })));
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

    // ── list_my_repos ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/repos"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .list_my_repos(Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].name, "repo1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/repos"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().list_my_repos(Default::default()).await;
        assert!(result.is_err());
    }

    // ── list_user_repos ────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_user_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/someuser/repos"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "someuser"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .list_user_repos("someuser", Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_user_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/someuser/repos"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_user_repos("someuser", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_org_repos ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_org_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "myorg"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .list_org_repos("myorg", Default::default())
            .await
            .unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_org_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_org_repos("myorg", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── search_repos ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_search_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client
            .repos()
            .search_repos(Default::default())
            .await
            .unwrap();
        assert!(!repos.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_search_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/search"))
            .respond_with(
                ResponseTemplate::new(500)
                    .set_body_json(json!({"message": "Internal Server Error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().search_repos(Default::default()).await;
        assert!(result.is_err());
    }

    // ── create_repo ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/user/repos"))
            .respond_with(ResponseTemplate::new(422).set_body_json(json!({"message": "Invalid"})))
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
        let result = client.repos().create_repo(opt).await;
        assert!(result.is_err());
    }

    // ── create_org_repo ────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_org_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(minimal_repo_json(2, "newrepo", "myorg")),
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
        let (repo, resp) = client.repos().create_org_repo("myorg", opt).await.unwrap();
        assert_eq!(repo.id, 2);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_org_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/orgs/myorg/repos"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
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
        let result = client.repos().create_org_repo("myorg", opt).await;
        assert!(result.is_err());
    }

    // ── get_repo ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_repo_happy() {
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
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/nonrepo"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_repo("testowner", "nonrepo").await;
        assert!(result.is_err());
    }

    // ── get_repo_by_id ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_repo_by_id_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repositories/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_repo_json(
                42,
                "some-repo",
                "owner",
            )))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repo, resp) = client.repos().get_repo_by_id(42).await.unwrap();
        assert_eq!(repo.id, 42);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_by_id_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repositories/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_repo_by_id(999).await;
        assert!(result.is_err());
    }

    // ── edit_repo ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "owner")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditRepoOption {
            description: Some("new desc".to_string()),
            name: None,
            website: None,
            private: None,
            template: None,
            has_issues: None,
            internal_tracker: None,
            external_tracker: None,
            has_wiki: None,
            external_wiki: None,
            default_branch: None,
            has_pull_requests: None,
            has_projects: None,
            has_releases: None,
            has_packages: None,
            has_actions: None,
            ignore_whitespace_conflicts: None,
            allow_fast_forward_only_merge: None,
            allow_merge: None,
            allow_rebase: None,
            allow_rebase_merge: None,
            allow_squash: None,
            archived: None,
            mirror_interval: None,
            allow_manual_merge: None,
            autodetect_manual_merge: None,
            default_merge_style: None,
            projects_mode: None,
            default_delete_branch_after_merge: None,
        };
        let (repo, resp) = client
            .repos()
            .edit_repo("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(repo.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditRepoOption {
            description: Some("new desc".to_string()),
            name: None,
            website: None,
            private: None,
            template: None,
            has_issues: None,
            internal_tracker: None,
            external_tracker: None,
            has_wiki: None,
            external_wiki: None,
            default_branch: None,
            has_pull_requests: None,
            has_projects: None,
            has_releases: None,
            has_packages: None,
            has_actions: None,
            ignore_whitespace_conflicts: None,
            allow_fast_forward_only_merge: None,
            allow_merge: None,
            allow_rebase: None,
            allow_rebase_merge: None,
            allow_squash: None,
            archived: None,
            mirror_interval: None,
            allow_manual_merge: None,
            autodetect_manual_merge: None,
            default_merge_style: None,
            projects_mode: None,
            default_delete_branch_after_merge: None,
        };
        let result = client.repos().edit_repo("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    // ── delete_repo ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.repos().delete_repo("owner", "repo").await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── mirror_sync ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_mirror_sync_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/mirror-sync"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.repos().mirror_sync("owner", "repo").await.unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_mirror_sync_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/mirror-sync"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().mirror_sync("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── get_repo_languages ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_repo_languages_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/languages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"Go": 1000})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (langs, resp) = client
            .repos()
            .get_repo_languages("owner", "repo")
            .await
            .unwrap();
        assert_eq!(langs.get("Go"), Some(&1000));
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_languages_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/languages"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_repo_languages("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── get_archive ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_archive_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.tar.gz"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake-archive-data"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_archive("owner", "repo", "main", "tar.gz")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_archive_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.tar.gz"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_archive("owner", "repo", "main", "tar.gz")
            .await;
        assert!(result.is_err());
    }

    // ── get_archive_reader ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_archive_reader_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.zip"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"fake-zip-data"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_archive_reader("owner", "repo", "main", "zip")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_archive_reader_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/archive/main.zip"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_archive_reader("owner", "repo", "main", "zip")
            .await;
        assert!(result.is_err());
    }

    // ── update_repo_avatar (multipart — returns Repository via get_parsed_response_multipart)
    // Skipping: multipart mocking is complex and not reliably supported by wiremock.
    // The method is covered implicitly by compilation.

    // ── delete_repo_avatar ─────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_repo_avatar_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/avatar"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .repos()
            .delete_repo_avatar("owner", "repo")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_repo_avatar_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/avatar"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_repo_avatar("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── list_branches ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_branches_happy() {
        let server = MockServer::start().await;
        let body = json!([{
            "name": "main",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        }]);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branches, resp) = client
            .repos()
            .list_branches("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_branches_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_branches("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_branch ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_branch_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "develop",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches/develop"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branch, resp) = client
            .repos()
            .get_branch("owner", "repo", "develop")
            .await
            .unwrap();
        assert_eq!(branch.name, "develop");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_branch_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branches/nonexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_branch("owner", "repo", "nonexist").await;
        assert!(result.is_err());
    }

    // ── delete_branch ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_branch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/branches/old%2Dbranch"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .repos()
            .delete_branch("owner", "repo", "old-branch")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_branch_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/branches/old-branch"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_branch("owner", "repo", "old-branch")
            .await;
        assert!(result.is_err());
    }

    // ── update_branch (version-gated, PATCH) ───────────────────────

    #[tokio::test]
    async fn test_update_branch_happy() {
        let server = MockServer::start().await;
        // version check
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.23.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "name": "new-name",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        });
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branches/old%2Dname"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branch, resp) = client
            .repos()
            .update_branch(
                "owner",
                "repo",
                "old-name",
                UpdateRepoBranchOption {
                    name: "new-name".to_string(),
                },
            )
            .await
            .unwrap();
        assert_eq!(branch.name, "new-name");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_branch_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.23.0"})))
            .mount(&server)
            .await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branches/old-name"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .update_branch(
                "owner",
                "repo",
                "old-name",
                UpdateRepoBranchOption {
                    name: "new-name".to_string(),
                },
            )
            .await;
        assert!(result.is_err());
    }

    // ── create_branch (version-gated) ──────────────────────────────

    // create_branch is already covered by test_create_branch_version_gated
    // (error case). Adding happy path:

    #[tokio::test]
    async fn test_create_branch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "name": "feature",
            "protected": false,
            "required_approvals": 0,
            "enable_status_check": false,
            "status_check_contexts": [],
            "user_can_push": true,
            "user_can_merge": true,
            "effective_branch_protection_name": ""
        });
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/branches"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (branch, resp) = client
            .repos()
            .create_branch(
                "owner",
                "repo",
                CreateBranchOption {
                    branch_name: "feature".to_string(),
                    old_branch_name: "main".to_string(),
                },
            )
            .await
            .unwrap();
        assert_eq!(branch.name, "feature");
        assert_eq!(resp.status, 201);
    }

    // ── list_tags ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_tags_happy() {
        let server = MockServer::start().await;
        let body = json!([{
            "name": "v1.0",
            "message": "release 1.0",
            "id": "abc123",
            "commit": null,
            "zipball_url": "",
            "tarball_url": ""
        }]);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tags, resp) = client
            .repos()
            .list_tags("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].name, "v1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_tags_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_tags("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_tag ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_tag_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "v1.0",
            "message": "release",
            "id": "sha123",
            "commit": null,
            "zipball_url": "",
            "tarball_url": ""
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags/v1%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tag, resp) = client
            .repos()
            .get_tag("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(tag.name, "v1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tags/nonexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_tag("owner", "repo", "nonexist").await;
        assert!(result.is_err());
    }

    // ── get_annotated_tag (version-gated) ──────────────────────────

    #[tokio::test]
    async fn test_get_annotated_tag_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "tag": "v1.0",
            "sha": "abc123",
            "url": "https://example.com",
            "message": "annotated tag",
            "tagger": null,
            "object": null,
            "verification": null
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/tags/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tag, resp) = client
            .repos()
            .get_annotated_tag("owner", "repo", "abc123")
            .await
            .unwrap();
        assert_eq!(tag.tag, "v1.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_annotated_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/tags/badsha"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_annotated_tag("owner", "repo", "badsha")
            .await;
        assert!(result.is_err());
    }

    // ── create_tag (version-gated) ─────────────────────────────────

    #[tokio::test]
    async fn test_create_tag_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        let body = json!({
            "name": "v2.0",
            "message": "release 2",
            "id": "sha456",
            "commit": null,
            "zipball_url": "",
            "tarball_url": ""
        });
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tag, resp) = client
            .repos()
            .create_tag(
                "owner",
                "repo",
                CreateTagOption {
                    tag_name: "v2.0".to_string(),
                    message: "release 2".to_string(),
                    target: "sha456".to_string(),
                },
            )
            .await
            .unwrap();
        assert_eq!(tag.name, "v2.0");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tags"))
            .respond_with(ResponseTemplate::new(409).set_body_json(json!({"message": "Conflict"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .create_tag(
                "owner",
                "repo",
                CreateTagOption {
                    tag_name: "v2.0".to_string(),
                    message: "release 2".to_string(),
                    target: "sha456".to_string(),
                },
            )
            .await;
        assert!(result.is_err());
    }

    // ── delete_tag (version-gated) ─────────────────────────────────

    #[tokio::test]
    async fn test_delete_tag_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tags/v1%2E0"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .repos()
            .delete_tag("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_tag_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tags/v1.0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_tag("owner", "repo", "v1.0").await;
        assert!(result.is_err());
    }

    // ── get_file ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_file_happy() {
        let server = MockServer::start().await;
        let body = json!({
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
            .and(path("/api/v1/repos/owner/repo/contents/README%2Emd"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_file("owner", "repo", "README.md", "main")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_file("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    // ── get_file_reader ────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_file_reader_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/README%2Emd"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"Hello World"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_file_reader("owner", "repo", "README.md", "main")
            .await
            .unwrap();
        assert_eq!(&*data, b"Hello World");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_file_reader_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_file_reader("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    // ── get_contents ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_contents_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "README.md",
            "path": "README.md",
            "sha": "abc",
            "type": "file",
            "size": 5,
            "last_commit_sha": "def"
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/README%2Emd"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (content, resp) = client
            .repos()
            .get_contents("owner", "repo", "README.md", "main")
            .await
            .unwrap();
        assert_eq!(content.name, "README.md");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_contents_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_contents("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    // ── list_contents ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_contents_happy() {
        let server = MockServer::start().await;
        let body = json!([{
            "name": "src",
            "path": "src",
            "sha": "abc",
            "type": "dir",
            "size": 0,
            "last_commit_sha": ""
        }]);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/src"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (items, resp) = client
            .repos()
            .list_contents("owner", "repo", "src", "main")
            .await
            .unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "src");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_contents_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/nonexist"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_contents("owner", "repo", "nonexist", "main")
            .await;
        assert!(result.is_err());
    }

    // ── create_file ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_file_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "content": {
                "name": "newfile.txt",
                "path": "newfile.txt",
                "sha": "abc",
                "type": "file",
                "size": 5,
                "last_commit_sha": "def"
            },
            "commit": {
                "sha": "commit123",
                "url": "https://example.com",
                "html_url": "https://example.com/commit123",
                "created": "2024-01-01T00:00:00Z",
                "message": "create file",
                "parents": []
            }
        });
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/contents/newfile%2Etxt"))
            .respond_with(ResponseTemplate::new(201).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateFileOptions {
            file_options: FileOptions {
                message: "create file".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            content: "SGVsbG8=".to_string(),
        };
        let (fr, resp) = client
            .repos()
            .create_file("owner", "repo", "newfile.txt", opt)
            .await
            .unwrap();
        assert!(fr.content.is_some());
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/contents/newfile%2Etxt"))
            .respond_with(
                ResponseTemplate::new(422).set_body_json(json!({"message": "Validation error"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateFileOptions {
            file_options: FileOptions {
                message: "create file".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            content: "SGVsbG8=".to_string(),
        };
        let result = client
            .repos()
            .create_file("owner", "repo", "newfile.txt", opt)
            .await;
        assert!(result.is_err());
    }

    // ── update_file ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_update_file_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "content": {
                "name": "file.txt",
                "path": "file.txt",
                "sha": "newsha",
                "type": "file",
                "size": 10,
                "last_commit_sha": "commit2"
            },
            "commit": {
                "sha": "commit2",
                "url": "https://example.com",
                "html_url": "https://example.com/commit2",
                "created": "2024-01-01T00:00:00Z",
                "message": "update file",
                "parents": []
            }
        });
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateFileOptions {
            file_options: FileOptions {
                message: "update file".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "oldsha".to_string(),
            content: "bmV3IGNvbnRlbnQ=".to_string(),
            from_path: String::new(),
        };
        let (fr, resp) = client
            .repos()
            .update_file("owner", "repo", "file.txt", opt)
            .await
            .unwrap();
        assert!(fr.content.is_some());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_update_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(
                ResponseTemplate::new(422).set_body_json(json!({"message": "SHA mismatch"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = UpdateFileOptions {
            file_options: FileOptions {
                message: "update".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "oldsha".to_string(),
            content: "bmV3".to_string(),
            from_path: String::new(),
        };
        let result = client
            .repos()
            .update_file("owner", "repo", "file.txt", opt)
            .await;
        assert!(result.is_err());
    }

    // ── delete_file ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_file_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = DeleteFileOptions {
            file_options: FileOptions {
                message: "delete".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "abc123".to_string(),
        };
        let resp = client
            .repos()
            .delete_file("owner", "repo", "file.txt", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_delete_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = DeleteFileOptions {
            file_options: FileOptions {
                message: "delete".to_string(),
                branch_name: "main".to_string(),
                new_branch_name: String::new(),
                author: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                committer: Identity {
                    name: String::new(),
                    email: String::new(),
                },
                dates: CommitDateOptions {
                    author: time::OffsetDateTime::UNIX_EPOCH,
                    committer: time::OffsetDateTime::UNIX_EPOCH,
                },
                signoff: false,
            },
            sha: "abc123".to_string(),
        };
        let result = client
            .repos()
            .delete_file("owner", "repo", "file.txt", opt)
            .await;
        assert!(result.is_err());
    }

    // ── get_contents_ext ───────────────────────────────────────────

    #[tokio::test]
    async fn test_get_contents_ext_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "file_contents": {
                "name": "file.txt",
                "path": "file.txt",
                "sha": "abc",
                "type": "file",
                "size": 5,
                "last_commit_sha": ""
            }
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = GetContentsExtOptions::default();
        let (ext, resp) = client
            .repos()
            .get_contents_ext("owner", "repo", "file.txt", "main", opt)
            .await
            .unwrap();
        assert!(ext.file_contents.is_some());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_contents_ext_flat_file_payload() {
        let server = MockServer::start().await;
        let body = json!({
            "name": "file.txt",
            "path": "file.txt",
            "sha": "abc",
            "type": "file",
            "size": 5,
            "last_commit_sha": ""
        });
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = GetContentsExtOptions::default();
        let (ext, resp) = client
            .repos()
            .get_contents_ext("owner", "repo", "file.txt", "main", opt)
            .await
            .unwrap();
        assert_eq!(
            ext.file_contents
                .expect("flat file payload should map")
                .path,
            "file.txt"
        );
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_contents_ext_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/contents/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = GetContentsExtOptions::default();
        let result = client
            .repos()
            .get_contents_ext("owner", "repo", "missing.txt", "main", opt)
            .await;
        assert!(result.is_err());
    }

    // ── get_editor_config ──────────────────────────────────────────

    #[tokio::test]
    async fn test_get_editor_config_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/editorconfig/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"root = true\n"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_editor_config("owner", "repo", "file.txt", "main")
            .await
            .unwrap();
        assert!(!data.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_editor_config_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/editorconfig/file%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_editor_config("owner", "repo", "file.txt", "main")
            .await;
        assert!(result.is_err());
    }

    // ── get_raw_file_or_lfs ────────────────────────────────────────

    #[tokio::test]
    async fn test_get_raw_file_or_lfs_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"raw content"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_raw_file_or_lfs("owner", "repo", "file.txt", "main")
            .await
            .unwrap();
        assert_eq!(&*data, b"raw content");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_raw_file_or_lfs_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_raw_file_or_lfs("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    // ── get_raw_file ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_raw_file_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/file%2Etxt"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(b"raw data"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (data, resp) = client
            .repos()
            .get_raw_file("owner", "repo", "file.txt", "main")
            .await
            .unwrap();
        assert_eq!(&*data, b"raw data");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_raw_file_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/raw/missing%2Etxt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_raw_file("owner", "repo", "missing.txt", "main")
            .await;
        assert!(result.is_err());
    }

    // ── list_collaborators ─────────────────────────────────────────

    fn minimal_user_json(id: i64, login: &str) -> serde_json::Value {
        json!({
            "id": id,
            "login": login,
            "login_name": "",
            "source_id": 0,
            "full_name": login,
            "email": "",
            "avatar_url": "",
            "html_url": "",
            "language": "",
            "is_admin": false,
            "restricted": false,
            "active": false,
            "prohibit_login": false,
            "location": "",
            "website": "",
            "description": "",
            "visibility": "public",
            "followers_count": 0,
            "following_count": 0,
            "starred_repos_count": 0,
        })
    }

    #[tokio::test]
    async fn test_list_collaborators_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/collaborators"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_user_json(1, "alice"),])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (collabs, resp) = client
            .repos()
            .list_collaborators("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(collabs.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_collaborators_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/collaborators"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_collaborators("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── is_collaborator (already covered by existing tests) ─────────

    // ── get_collaborator_permission ────────────────────────────────

    #[tokio::test]
    async fn test_get_collaborator_permission_happy() {
        let server = MockServer::start().await;
        let body = json!({
            "permission": "write",
            "role_name": "Write",
            "user": minimal_user_json(1, "alice")
        });
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/owner/repo/collaborators/alice/permission",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (perm, resp) = client
            .repos()
            .get_collaborator_permission("owner", "repo", "alice")
            .await
            .unwrap();
        assert!(perm.is_some());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_collaborator_permission_not_found() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/api/v1/repos/owner/repo/collaborators/nobody/permission",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (perm, resp) = client
            .repos()
            .get_collaborator_permission("owner", "repo", "nobody")
            .await
            .unwrap();
        assert!(perm.is_none());
        assert_eq!(resp.status, 404);
    }

    // ── add_collaborator ───────────────────────────────────────────

    #[tokio::test]
    async fn test_add_collaborator_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/collaborators/alice"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let mut opt = AddCollaboratorOption {
            permission: Some(crate::types::enums::AccessMode::Write),
        };
        opt.validate().unwrap();
        let resp = client
            .repos()
            .add_collaborator("owner", "repo", "alice", opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_add_collaborator_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/collaborators/alice"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let mut opt = AddCollaboratorOption {
            permission: Some(crate::types::enums::AccessMode::Write),
        };
        opt.validate().unwrap();
        let result = client
            .repos()
            .add_collaborator("owner", "repo", "alice", opt)
            .await;
        assert!(result.is_err());
    }

    // ═══════════════════════════════════════════════════════════════
    // Batch B: Methods 42–82
    // ═══════════════════════════════════════════════════════════════

    // ── delete_collaborator ──────────────────────────────────────

    #[tokio::test]
    async fn test_delete_collaborator_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/collaborators/user1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_collaborator("owner", "repo", "user1")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_collaborator_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/collaborators/user1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_collaborator("owner", "repo", "user1")
            .await;
        assert!(result.is_err());
    }

    // ── get_reviewers ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_reviewers_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/reviewers"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_user_json(1, "reviewer1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reviewers, resp) = client.repos().get_reviewers("owner", "repo").await.unwrap();
        assert_eq!(reviewers.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_reviewers_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/reviewers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_reviewers("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── get_assignees ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_assignees_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/assignees"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_user_json(1, "assignee1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (assignees, resp) = client.repos().get_assignees("owner", "repo").await.unwrap();
        assert_eq!(assignees.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_assignees_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/assignees"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_assignees("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── get_single_commit ─────────────────────────────────────────

    fn minimal_commit_json(sha: &str) -> serde_json::Value {
        json!({
            "url": format!("https://gitea.example.com/api/v1/repos/owner/repo/git/commits/{sha}"),
            "sha": sha,
            "created": "2024-01-01T00:00:00Z",
            "html_url": "",
        })
    }

    #[tokio::test]
    async fn test_get_single_commit_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_commit_json("abc123")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (commit, resp) = client
            .repos()
            .get_single_commit("owner", "repo", "abc123")
            .await
            .unwrap();
        assert_eq!(commit.commit_meta.sha, "abc123");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_single_commit_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_single_commit("owner", "repo", "abc123")
            .await;
        assert!(result.is_err());
    }

    // ── list_commits ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_commits_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/commits"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_commit_json("sha1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (commits, resp) = client
            .repos()
            .list_commits("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(commits.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_commits_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/commits"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_commits("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_commit_diff ───────────────────────────────────────────

    #[tokio::test]
    async fn test_get_commit_diff_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.diff"))
            .respond_with(ResponseTemplate::new(200).set_body_string("diff --git a/file b/file"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (diff, resp) = client
            .repos()
            .get_commit_diff("owner", "repo", "abc123")
            .await
            .unwrap();
        assert!(!diff.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_commit_diff_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.diff"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_commit_diff("owner", "repo", "abc123")
            .await;
        assert!(result.is_err());
    }

    // ── get_commit_patch ──────────────────────────────────────────

    #[tokio::test]
    async fn test_get_commit_patch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.patch"))
            .respond_with(ResponseTemplate::new(200).set_body_string("patch content"))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (patch, resp) = client
            .repos()
            .get_commit_patch("owner", "repo", "abc123")
            .await
            .unwrap();
        assert!(!patch.is_empty());
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_commit_patch_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
            .mount(&server)
            .await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/commits/abc123.patch"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_commit_patch("owner", "repo", "abc123")
            .await;
        assert!(result.is_err());
    }

    // ── create_push_mirror ────────────────────────────────────────

    fn minimal_push_mirror_json() -> serde_json::Value {
        json!({
            "created": "2024-01-01T00:00:00Z",
            "interval": "8h",
            "last_error": "",
            "last_update": "2024-01-01T00:00:00Z",
            "remote_address": "https://example.com/repo.git",
            "remote_name": "origin",
            "repo_name": "repo",
            "sync_on_commit": false,
        })
    }

    #[tokio::test]
    async fn test_create_push_mirror_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_push_mirror_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreatePushMirrorOption {
            interval: "8h".to_string(),
            remote_address: "https://example.com/repo.git".to_string(),
            remote_password: String::new(),
            remote_username: String::new(),
            sync_on_commit: false,
        };
        let (mirror, resp) = client
            .repos()
            .create_push_mirror("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(mirror.remote_address, "https://example.com/repo.git");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_push_mirror_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreatePushMirrorOption {
            interval: "8h".to_string(),
            remote_address: "https://example.com/repo.git".to_string(),
            remote_password: String::new(),
            remote_username: String::new(),
            sync_on_commit: false,
        };
        let result = client
            .repos()
            .create_push_mirror("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    // ── list_push_mirrors ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_push_mirrors_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_push_mirror_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (mirrors, resp) = client
            .repos()
            .list_push_mirrors("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(mirrors.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_push_mirrors_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_push_mirrors("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_push_mirror ───────────────────────────────────────────

    #[tokio::test]
    async fn test_get_push_mirror_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_push_mirror_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (mirror, resp) = client
            .repos()
            .get_push_mirror("owner", "repo", "origin")
            .await
            .unwrap();
        assert_eq!(mirror.interval, "8h");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_push_mirror_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_push_mirror("owner", "repo", "origin")
            .await;
        assert!(result.is_err());
    }

    // ── delete_push_mirror ────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_push_mirror_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_push_mirror("owner", "repo", "origin")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_push_mirror_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/push_mirrors/origin"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_push_mirror("owner", "repo", "origin")
            .await;
        assert!(result.is_err());
    }

    // ── create_repo_from_template ─────────────────────────────────

    #[tokio::test]
    async fn test_create_repo_from_template_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/template/generate"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(minimal_repo_json(3, "newrepo", "newowner")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateRepoFromTemplateOption {
            owner: "newowner".to_string(),
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            git_content: true,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        let (repo, resp) = client
            .repos()
            .create_repo_from_template("owner", "template", opt)
            .await
            .unwrap();
        assert_eq!(repo.id, 3);
        assert_eq!(repo.name, "newrepo");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_repo_from_template_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/template/generate"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateRepoFromTemplateOption {
            owner: "newowner".to_string(),
            name: "newrepo".to_string(),
            description: String::new(),
            private: false,
            git_content: true,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        let result = client
            .repos()
            .create_repo_from_template("owner", "template", opt)
            .await;
        assert!(result.is_err());
    }

    // ── list_topics ───────────────────────────────────────────────

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

    // ── set_topics ────────────────────────────────────────────────

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

    // ── add_topic ─────────────────────────────────────────────────

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

    // ── delete_topic ──────────────────────────────────────────────

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

    // ── get_watched_repos ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_watched_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/subscriptions"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_watched_repos("testuser").await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_watched_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/subscriptions"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_watched_repos("testuser").await;
        assert!(result.is_err());
    }

    // ── get_my_watched_repos ──────────────────────────────────────

    #[tokio::test]
    async fn test_get_my_watched_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/subscriptions"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_my_watched_repos().await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_my_watched_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/subscriptions"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_my_watched_repos().await;
        assert!(result.is_err());
    }

    // ── check_repo_watch ──────────────────────────────────────────

    #[tokio::test]
    async fn test_check_repo_watch_true() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (watching, resp) = client
            .repos()
            .check_repo_watch("owner", "repo")
            .await
            .unwrap();
        assert!(watching);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_check_repo_watch_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (watching, resp) = client
            .repos()
            .check_repo_watch("owner", "repo")
            .await
            .unwrap();
        assert!(!watching);
        assert_eq!(resp.status, 404);
    }

    // ── watch_repo ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_watch_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().watch_repo("owner", "repo").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[tokio::test]
    async fn test_watch_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().watch_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── unwatch_repo ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_unwatch_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unwatch_repo("owner", "repo").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_unwatch_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/subscription"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unwatch_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── list_stargazers ───────────────────────────────────────────

    #[tokio::test]
    async fn test_list_stargazers_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/stargazers"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_user_json(1, "stargazer1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (stargazers, resp) = client
            .repos()
            .list_stargazers("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(stargazers.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_stargazers_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/stargazers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_stargazers("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_starred_repos ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_starred_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/starred"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_starred_repos("testuser").await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_starred_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/users/testuser/starred"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_starred_repos("testuser").await;
        assert!(result.is_err());
    }

    // ── get_my_starred_repos ──────────────────────────────────────

    #[tokio::test]
    async fn test_get_my_starred_repos_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(json!([minimal_repo_json(1, "repo1", "owner1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (repos, resp) = client.repos().get_my_starred_repos().await.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_my_starred_repos_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred"))
            .respond_with(
                ResponseTemplate::new(401).set_body_json(json!({"message": "Unauthorized"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_my_starred_repos().await;
        assert!(result.is_err());
    }

    // ── is_repo_starring ──────────────────────────────────────────

    #[tokio::test]
    async fn test_is_repo_starring_true() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (starring, resp) = client
            .repos()
            .is_repo_starring("owner", "repo")
            .await
            .unwrap();
        assert!(starring);
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_is_repo_starring_false() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (starring, resp) = client
            .repos()
            .is_repo_starring("owner", "repo")
            .await
            .unwrap();
        assert!(!starring);
        assert_eq!(resp.status, 404);
    }

    // ── star_repo error path (happy already exists) ───────────────

    #[tokio::test]
    async fn test_star_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().star_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── unstar_repo ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_unstar_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unstar_repo("owner", "repo").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_unstar_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/user/starred/owner/repo"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().unstar_repo("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── create_wiki_page ──────────────────────────────────────────

    #[tokio::test]
    async fn test_create_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/wiki/new"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "SGVsbG8gV29ybGQ=".to_string(),
            message: "create page".to_string(),
        };
        let result = client.repos().create_wiki_page("owner", "repo", opt).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_create_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/wiki/new"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "SGVsbG8gV29ybGQ=".to_string(),
            message: "create page".to_string(),
        };
        let result = client.repos().create_wiki_page("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    // ── get_wiki_page ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "title": "Home",
                "content_base64": "SGVsbG8gV29ybGQ=",
                "commit_count": 1,
                "sidebar": "",
                "footer": "",
                "html_url": "https://gitea.example.com/owner/repo/wiki/Home",
                "sub_url": "/owner/repo/wiki/Home"
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (page, resp) = client
            .repos()
            .get_wiki_page("owner", "repo", "Home")
            .await
            .unwrap();
        assert_eq!(page.title, "Home");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_wiki_page("owner", "repo", "Home").await;
        assert!(result.is_err());
    }

    // ── edit_wiki_page ────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "VXBkYXRlZCBjb250ZW50".to_string(),
            message: "update page".to_string(),
        };
        let result = client
            .repos()
            .edit_wiki_page("owner", "repo", "Home", opt)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[tokio::test]
    async fn test_edit_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateWikiPageOptions {
            title: "Home".to_string(),
            content_base64: "VXBkYXRlZCBjb250ZW50".to_string(),
            message: "update page".to_string(),
        };
        let result = client
            .repos()
            .edit_wiki_page("owner", "repo", "Home", opt)
            .await;
        assert!(result.is_err());
    }

    // ── delete_wiki_page ──────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_wiki_page_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_wiki_page("owner", "repo", "Home")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_wiki_page_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/wiki/page/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .delete_wiki_page("owner", "repo", "Home")
            .await;
        assert!(result.is_err());
    }

    // ── list_wiki_pages ───────────────────────────────────────────

    #[tokio::test]
    async fn test_list_wiki_pages_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/pages"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!([
                {
                    "title": "Home",
                    "html_url": "https://gitea.example.com/owner/repo/wiki/Home",
                    "sub_url": "/owner/repo/wiki/Home"
                }
            ])))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (pages, resp) = client
            .repos()
            .list_wiki_pages("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(pages.len(), 1);
        assert_eq!(pages[0].title, "Home");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_wiki_pages_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/pages"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_wiki_pages("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_wiki_revisions ────────────────────────────────────────

    #[tokio::test]
    async fn test_get_wiki_revisions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/revisions/Home"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "count": 1,
                "commits": [
                    { "sha": "abc123", "message": "initial page" }
                ]
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (revs, resp) = client
            .repos()
            .get_wiki_revisions("owner", "repo", "Home", Default::default())
            .await
            .unwrap();
        assert_eq!(revs.count, 1);
        assert_eq!(revs.commits.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_wiki_revisions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/wiki/revisions/Home"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_wiki_revisions("owner", "repo", "Home", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_trees ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_trees_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/trees/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "sha": "abc123",
                "url": "https://gitea.example.com/api/v1/repos/owner/repo/git/trees/abc123",
                "tree": [
                    {
                        "path": "README.md",
                        "mode": "100644",
                        "type": "blob",
                        "size": 10,
                        "sha": "def456",
                        "url": ""
                    }
                ],
                "truncated": false,
                "page": 1,
                "total_count": 1
            })))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (tree, resp) = client
            .repos()
            .get_trees("owner", "repo", "main", Default::default())
            .await
            .unwrap();
        assert_eq!(tree.sha, "abc123");
        assert_eq!(tree.tree.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_trees_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/trees/main"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .get_trees("owner", "repo", "main", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── list_labels ───────────────────────────────────────────────

    fn minimal_label_json(id: i64, name: &str) -> serde_json::Value {
        json!({
            "id": id,
            "name": name,
            "color": "ff0000",
            "description": "",
            "exclusive": false,
            "is_archived": false,
            "url": ""
        })
    }

    #[tokio::test]
    async fn test_list_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(json!([minimal_label_json(1, "bug")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (labels, resp) = client
            .repos()
            .list_labels("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .repos()
            .list_labels("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── get_label ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_label_json(1, "bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (label, resp) = client.repos().get_label("owner", "repo", 1).await.unwrap();
        assert_eq!(label.id, 1);
        assert_eq!(label.name, "bug");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/labels/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().get_label("owner", "repo", 999).await;
        assert!(result.is_err());
    }

    // ── create_label ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_create_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(minimal_label_json(2, "feature")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        let (label, resp) = client
            .repos()
            .create_label("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(label.id, 2);
        assert_eq!(label.name, "feature");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateLabelOption {
            name: "feature".to_string(),
            color: "00ff00".to_string(),
            description: String::new(),
            exclusive: false,
            is_archived: false,
        };
        let result = client.repos().create_label("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    // ── edit_label ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_edit_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_label_json(1, "bugfix")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditLabelOption {
            name: Some("bugfix".to_string()),
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
        };
        let (label, resp) = client
            .repos()
            .edit_label("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(label.name, "bugfix");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditLabelOption {
            name: Some("bugfix".to_string()),
            color: None,
            description: None,
            exclusive: None,
            is_archived: None,
        };
        let result = client.repos().edit_label("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    // ── delete_label ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_delete_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_label("owner", "repo", 1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/labels/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.repos().delete_label("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    // ── migrate_repo ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_migrate_repo_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/migrate"))
            .respond_with(
                ResponseTemplate::new(201)
                    .set_body_json(minimal_repo_json(10, "migrated", "owner")),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = MigrateRepoOption {
            repo_name: "migrated".to_string(),
            repo_owner: "owner".to_string(),
            uid: 0,
            clone_addr: "https://github.com/example/repo.git".to_string(),
            service: crate::types::enums::GitServiceType::Github,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: "ghp_test_token".to_string(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: true,
            milestones: true,
            labels: true,
            issues: true,
            pull_requests: true,
            releases: true,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        let (repo, resp) = client.repos().migrate_repo(opt).await.unwrap();
        assert_eq!(repo.id, 10);
        assert_eq!(repo.name, "migrated");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_migrate_repo_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/migrate"))
            .respond_with(
                ResponseTemplate::new(409)
                    .set_body_json(json!({"message": "Repository already exists"})),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = MigrateRepoOption {
            repo_name: "migrated".to_string(),
            repo_owner: "owner".to_string(),
            uid: 0,
            clone_addr: "https://github.com/example/repo.git".to_string(),
            service: crate::types::enums::GitServiceType::Github,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: "ghp_test_token".to_string(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: true,
            milestones: true,
            labels: true,
            issues: true,
            pull_requests: true,
            releases: true,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        let result = client.repos().migrate_repo(opt).await;
        assert!(result.is_err());
    }

    // ── Batch C: Branch/Tag Protection, Transfer, Team, Deploy Keys,
    //    Forks, Blob, Git Hooks, Refs, Compare, Notes, Action Secrets/Variables ──

    fn minimal_branch_protection_json() -> serde_json::Value {
        let ts = "2024-01-01T00:00:00Z";
        serde_json::json!({
            "branch_name": "main",
            "rule_name": "main",
            "enable_push": false,
            "enable_push_whitelist": false,
            "push_whitelist_usernames": [],
            "push_whitelist_teams": [],
            "push_whitelist_deploy_keys": false,
            "enable_merge_whitelist": false,
            "merge_whitelist_usernames": [],
            "merge_whitelist_teams": [],
            "enable_status_check": false,
            "status_check_contexts": [],
            "required_approvals": 0,
            "enable_approvals_whitelist": false,
            "approvals_whitelist_username": [],
            "approvals_whitelist_teams": [],
            "block_on_rejected_reviews": false,
            "block_on_official_review_requests": false,
            "block_on_outdated_branch": false,
            "dismiss_stale_approvals": false,
            "require_signed_commits": false,
            "protected_file_patterns": "",
            "unprotected_file_patterns": "",
            "created_at": ts,
            "updated_at": ts,
        })
    }

    fn minimal_tag_protection_json(id: i64) -> serde_json::Value {
        let ts = "2024-01-01T00:00:00Z";
        serde_json::json!({
            "id": id,
            "name_pattern": "v*",
            "whitelist_usernames": [],
            "whitelist_teams": [],
            "created_at": ts,
            "updated_at": ts,
        })
    }

    fn minimal_team_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": "developers",
            "description": "Dev team",
            "permission": "write",
            "can_create_org_repo": true,
            "includes_all_repositories": false,
            "units": [],
        })
    }

    fn minimal_deploy_key_json(id: i64) -> serde_json::Value {
        let ts = "2024-01-01T00:00:00Z";
        serde_json::json!({
            "id": id,
            "key_id": id,
            "key": "ssh-rsa AAAAB3...",
            "url": "",
            "title": "deploy-key",
            "fingerprint": "ab:cd:ef",
            "created_at": ts,
            "read_only": true,
        })
    }

    fn minimal_git_hook_json() -> serde_json::Value {
        serde_json::json!({
            "name": "pre-receive",
            "is_active": true,
            "content": "#!/bin/sh\necho hello",
        })
    }

    fn minimal_reference_json() -> serde_json::Value {
        serde_json::json!({
            "ref": "refs/heads/main",
            "url": "https://example.com/api/v1/repos/o/r/git/refs/heads/main",
            "object": {
                "type": "commit",
                "sha": "abc123",
                "url": "https://example.com/api/v1/repos/o/r/git/commits/abc123",
            },
        })
    }

    fn minimal_git_blob_json() -> serde_json::Value {
        serde_json::json!({
            "content": "SGVsbG8gV29ybGQ=",
            "encoding": "base64",
            "url": "https://example.com/api/v1/repos/o/r/git/blobs/abc123",
            "sha": "abc123",
            "size": 11,
        })
    }

    fn minimal_compare_json() -> serde_json::Value {
        serde_json::json!({
            "total_commits": 1,
            "commits": [],
        })
    }

    fn minimal_note_json() -> serde_json::Value {
        serde_json::json!({
            "message": "Test note",
        })
    }

    fn minimal_secret_json(name: &str) -> serde_json::Value {
        let ts = "2024-01-01T00:00:00Z";
        serde_json::json!({
            "name": name,
            "data": "",
            "description": "",
            "created": ts,
        })
    }

    fn minimal_repo_action_variable_json(name: &str, value: &str) -> serde_json::Value {
        serde_json::json!({
            "owner_id": 1,
            "repo_id": 1,
            "name": name,
            "data": value,
        })
    }

    // ── Branch Protection ───────────────────────────────────────

    #[tokio::test]
    async fn test_list_branch_protections_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_branch_protection_json()])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_branch_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (protections, resp) = result.unwrap();
        assert_eq!(protections.len(), 1);
        assert_eq!(protections[0].branch_name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_branch_protections_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_branch_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_branch_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_branch_protection_json()),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_branch_protection("owner", "repo", "main")
            .await;
        assert!(result.is_ok());
        let (bp, resp) = result.unwrap();
        assert_eq!(bp.branch_name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_branch_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_branch_protection("owner", "repo", "main")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_branch_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(
                ResponseTemplate::new(201).set_body_json(minimal_branch_protection_json()),
            )
            .mount(&server)
            .await;
        let opt = CreateBranchProtectionOption {
            branch_name: "main".to_string(),
            rule_name: "main".to_string(),
            enable_push: false,
            enable_push_whitelist: false,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: false,
            enable_merge_whitelist: false,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: false,
            status_check_contexts: vec![],
            required_approvals: 0,
            enable_approvals_whitelist: false,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: false,
            block_on_official_review_requests: false,
            block_on_outdated_branch: false,
            dismiss_stale_approvals: false,
            require_signed_commits: false,
            protected_file_patterns: String::new(),
            unprotected_file_patterns: String::new(),
        };
        let result = client
            .repos()
            .create_branch_protection("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        let (bp, resp) = result.unwrap();
        assert_eq!(bp.branch_name, "main");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_branch_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/branch_protections"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateBranchProtectionOption {
            branch_name: "main".to_string(),
            rule_name: "main".to_string(),
            enable_push: false,
            enable_push_whitelist: false,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: false,
            enable_merge_whitelist: false,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: false,
            status_check_contexts: vec![],
            required_approvals: 0,
            enable_approvals_whitelist: false,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: false,
            block_on_official_review_requests: false,
            block_on_outdated_branch: false,
            dismiss_stale_approvals: false,
            require_signed_commits: false,
            protected_file_patterns: String::new(),
            unprotected_file_patterns: String::new(),
        };
        let result = client
            .repos()
            .create_branch_protection("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_branch_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_branch_protection_json()),
            )
            .mount(&server)
            .await;
        let opt = EditBranchProtectionOption {
            enable_push: None,
            enable_push_whitelist: None,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: None,
            enable_merge_whitelist: None,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: None,
            status_check_contexts: vec![],
            required_approvals: None,
            enable_approvals_whitelist: None,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: None,
            block_on_official_review_requests: None,
            block_on_outdated_branch: None,
            dismiss_stale_approvals: None,
            require_signed_commits: None,
            protected_file_patterns: None,
            unprotected_file_patterns: None,
        };
        let result = client
            .repos()
            .edit_branch_protection("owner", "repo", "main", opt)
            .await;
        assert!(result.is_ok());
        let (bp, resp) = result.unwrap();
        assert_eq!(bp.branch_name, "main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_branch_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditBranchProtectionOption {
            enable_push: None,
            enable_push_whitelist: None,
            push_whitelist_usernames: vec![],
            push_whitelist_teams: vec![],
            push_whitelist_deploy_keys: None,
            enable_merge_whitelist: None,
            merge_whitelist_usernames: vec![],
            merge_whitelist_teams: vec![],
            enable_status_check: None,
            status_check_contexts: vec![],
            required_approvals: None,
            enable_approvals_whitelist: None,
            approvals_whitelist_usernames: vec![],
            approvals_whitelist_teams: vec![],
            block_on_rejected_reviews: None,
            block_on_official_review_requests: None,
            block_on_outdated_branch: None,
            dismiss_stale_approvals: None,
            require_signed_commits: None,
            protected_file_patterns: None,
            unprotected_file_patterns: None,
        };
        let result = client
            .repos()
            .edit_branch_protection("owner", "repo", "main", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_branch_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_branch_protection("owner", "repo", "main")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_branch_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/branch_protections/main"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_branch_protection("owner", "repo", "main")
            .await;
        assert!(result.is_err());
    }

    // ── Tag Protection ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_tag_protections_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_tag_protection_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_tag_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (protections, resp) = result.unwrap();
        assert_eq!(protections.len(), 1);
        assert_eq!(protections[0].name_pattern, "v*");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_tag_protections_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_tag_protections("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let result = client.repos().get_tag_protection("owner", "repo", 1).await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(tp.name_pattern, "v*");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_tag_protection("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let opt = CreateTagProtectionOption {
            name_pattern: "v*".to_string(),
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .create_tag_protection("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/tag_protections"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateTagProtectionOption {
            name_pattern: "v*".to_string(),
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .create_tag_protection("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_tag_protection_json(1)))
            .mount(&server)
            .await;
        let opt = EditTagProtectionOption {
            name_pattern: None,
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .edit_tag_protection("owner", "repo", 1, opt)
            .await;
        assert!(result.is_ok());
        let (tp, resp) = result.unwrap();
        assert_eq!(tp.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditTagProtectionOption {
            name_pattern: None,
            whitelist_usernames: vec![],
            whitelist_teams: vec![],
        };
        let result = client
            .repos()
            .edit_tag_protection("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_tag_protection_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_tag_protection("owner", "repo", 1)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_tag_protection_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/tag_protections/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_tag_protection("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    // ── Transfer ────────────────────────────────────────────────

    #[tokio::test]
    async fn test_transfer_repo_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "newowner")),
            )
            .mount(&server)
            .await;
        let opt = TransferRepoOption {
            new_owner: "newowner".to_string(),
            team_ids: None,
        };
        let result = client.repos().transfer_repo("owner", "repo", opt).await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_transfer_repo_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = TransferRepoOption {
            new_owner: "newowner".to_string(),
            team_ids: None,
        };
        let result = client.repos().transfer_repo("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_accept_repo_transfer_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/accept"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "owner")),
            )
            .mount(&server)
            .await;
        let result = client.repos().accept_repo_transfer("owner", "repo").await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_accept_repo_transfer_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/accept"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let result = client.repos().accept_repo_transfer("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reject_repo_transfer_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/reject"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(minimal_repo_json(1, "repo", "owner")),
            )
            .mount(&server)
            .await;
        let result = client.repos().reject_repo_transfer("owner", "repo").await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_reject_repo_transfer_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/transfer/reject"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let result = client.repos().reject_repo_transfer("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── Repo Teams ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_repo_teams_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([minimal_team_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client.repos().get_repo_teams("owner", "repo").await;
        assert!(result.is_ok());
        let (teams, resp) = result.unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].name, "developers");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_teams_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_repo_teams("owner", "repo").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_repo_team_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .add_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_add_repo_team_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .add_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_repo_team_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .remove_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_remove_repo_team_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .remove_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_repo_team_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_team_json(1)))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .check_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        let (team, resp) = result.unwrap();
        assert!(team.is_some());
        assert_eq!(team.unwrap().name, "developers");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_check_repo_team_not_found() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/teams/developers"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .check_repo_team("owner", "repo", "developers")
            .await;
        assert!(result.is_ok());
        let (team, resp) = result.unwrap();
        assert!(team.is_none());
        assert_eq!(resp.status, 404);
    }

    // ── Deploy Keys ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_deploy_keys_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_deploy_key_json(1)])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_deploy_keys("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (keys, resp) = result.unwrap();
        assert_eq!(keys.len(), 1);
        assert_eq!(keys[0].id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_deploy_keys_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_deploy_keys("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_deploy_key_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_deploy_key_json(1)))
            .mount(&server)
            .await;
        let result = client.repos().get_deploy_key("owner", "repo", 1).await;
        assert!(result.is_ok());
        let (key, resp) = result.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(key.title, "deploy-key");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_deploy_key_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_deploy_key("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_deploy_key_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(ResponseTemplate::new(201).set_body_json(minimal_deploy_key_json(1)))
            .mount(&server)
            .await;
        let opt = CreateKeyOption {
            title: "deploy-key".to_string(),
            key: "ssh-rsa AAAAB3...".to_string(),
            read_only: true,
        };
        let result = client.repos().create_deploy_key("owner", "repo", opt).await;
        assert!(result.is_ok());
        let (key, resp) = result.unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_deploy_key_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/keys"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateKeyOption {
            title: "deploy-key".to_string(),
            key: "ssh-rsa AAAAB3...".to_string(),
            read_only: true,
        };
        let result = client.repos().create_deploy_key("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_deploy_key_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client.repos().delete_deploy_key("owner", "repo", 1).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_deploy_key_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/keys/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().delete_deploy_key("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    // ── Forks ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_forks_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                minimal_repo_json(2, "fork-repo", "user")
            ])))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_forks("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (forks, resp) = result.unwrap();
        assert_eq!(forks.len(), 1);
        assert_eq!(forks[0].name, "fork-repo");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_forks_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_forks("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_fork_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(202).set_body_json(minimal_repo_json(
                2,
                "fork-repo",
                "user",
            )))
            .mount(&server)
            .await;
        let opt = CreateForkOption {
            organization: None,
            name: Some("fork-repo".to_string()),
        };
        let result = client.repos().create_fork("owner", "repo", opt).await;
        assert!(result.is_ok());
        let (repo, resp) = result.unwrap();
        assert_eq!(repo.name, "fork-repo");
        assert_eq!(resp.status, 202);
    }

    #[tokio::test]
    async fn test_create_fork_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/forks"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateForkOption {
            organization: None,
            name: Some("fork-repo".to_string()),
        };
        let result = client.repos().create_fork("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    // ── Blob ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_blob_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/blobs/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_git_blob_json()))
            .mount(&server)
            .await;
        let result = client.repos().get_blob("owner", "repo", "abc123").await;
        assert!(result.is_ok());
        let (blob, resp) = result.unwrap();
        assert_eq!(blob.sha, "abc123");
        assert_eq!(blob.encoding, "base64");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_blob_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/blobs/abc123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().get_blob("owner", "repo", "abc123").await;
        assert!(result.is_err());
    }

    // ── Git Hooks ───────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_git_hooks_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_git_hook_json()])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_git_hooks("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (hooks, resp) = result.unwrap();
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "pre-receive");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_git_hooks_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_git_hooks("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_git_hook_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_git_hook_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_ok());
        let (hook, resp) = result.unwrap();
        assert_eq!(hook.name, "pre-receive");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_git_hook_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_git_hook_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_git_hook_json()))
            .mount(&server)
            .await;
        let opt = EditGitHookOption {
            content: "#!/bin/sh\necho hello".to_string(),
        };
        let result = client
            .repos()
            .edit_git_hook("owner", "repo", "pre-receive", opt)
            .await;
        assert!(result.is_ok());
        let (hook, resp) = result.unwrap();
        assert_eq!(hook.name, "pre-receive");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_git_hook_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let opt = EditGitHookOption {
            content: "#!/bin/sh\necho hello".to_string(),
        };
        let result = client
            .repos()
            .edit_git_hook("owner", "repo", "pre-receive", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_git_hook_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_git_hook_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/hooks/git/pre-receive"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_git_hook("owner", "repo", "pre-receive")
            .await;
        assert!(result.is_err());
    }

    // ── Refs ────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_repo_ref_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads/main"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_reference_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_ref("owner", "repo", "refs/heads/main")
            .await;
        assert!(result.is_ok());
        let (ref_, resp) = result.unwrap();
        assert_eq!(ref_.ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_ref_happy_with_array_payload() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads/main"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_reference_json()])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_ref("owner", "repo", "refs/heads/main")
            .await;
        assert!(result.is_ok());
        let (ref_, resp) = result.unwrap();
        assert_eq!(ref_.ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_ref_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads/nonexistent"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_ref("owner", "repo", "refs/heads/nonexistent")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_repo_refs_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_reference_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_refs("owner", "repo", "refs/heads")
            .await;
        assert!(result.is_ok());
        let (refs, resp) = result.unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_refs_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs/heads"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_refs("owner", "repo", "refs/heads")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_all_git_refs_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_reference_json()])),
            )
            .mount(&server)
            .await;
        let result = client.repos().list_all_git_refs("owner", "repo").await;
        assert!(result.is_ok());
        let (refs, resp) = result.unwrap();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].ref_, "refs/heads/main");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_all_git_refs_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/refs"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client.repos().list_all_git_refs("owner", "repo").await;
        assert!(result.is_err());
    }

    // ── Compare ─────────────────────────────────────────────────

    #[tokio::test]
    async fn test_compare_commits_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/compare/abc123...def456"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_compare_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .compare_commits("owner", "repo", "abc123", "def456")
            .await;
        assert!(result.is_ok());
        let (compare, resp) = result.unwrap();
        assert_eq!(compare.total_commits, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_compare_commits_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/compare/abc123...def456"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .compare_commits("owner", "repo", "abc123", "def456")
            .await;
        assert!(result.is_err());
    }

    // ── Notes ───────────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_repo_note_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/notes/abc123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(minimal_note_json()))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_note(
                "owner",
                "repo",
                "abc123",
                GetRepoNoteOptions {
                    verification: None,
                    files: None,
                },
            )
            .await;
        assert!(result.is_ok());
        let (note, resp) = result.unwrap();
        assert_eq!(note.message, "Test note");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_repo_note_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/git/notes/abc123"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_repo_note(
                "owner",
                "repo",
                "abc123",
                GetRepoNoteOptions {
                    verification: None,
                    files: None,
                },
            )
            .await;
        assert!(result.is_err());
    }

    // ── Action Secrets ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_action_secrets_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([minimal_secret_json("MY_SECRET")])),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_secrets("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (secrets, resp) = result.unwrap();
        assert_eq!(secrets.len(), 1);
        assert_eq!(secrets[0].name, "MY_SECRET");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_action_secrets_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_secrets("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_action_variables_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
                minimal_repo_action_variable_json("VAR1", "val1")
            ])))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_variables("owner", "repo", Default::default())
            .await;
        assert!(result.is_ok());
        let (vars, resp) = result.unwrap();
        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name, "VAR1");
        assert_eq!(vars[0].data, "val1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_action_variables_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .list_action_variables("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_action_secret_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let result = client
            .repos()
            .create_action_secret("owner", "repo", opt)
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_create_action_secret_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let opt = CreateSecretOption {
            name: "MY_SECRET".to_string(),
            data: "secret-data".to_string(),
            description: None,
        };
        let result = client
            .repos()
            .create_action_secret("owner", "repo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_action_secret_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_secret("owner", "repo", "MY_SECRET")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_action_secret_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/secrets/MY_SECRET"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_secret("owner", "repo", "MY_SECRET")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(minimal_repo_action_variable_json("VAR1", "val1")),
            )
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_ok());
        let (var, resp) = result.unwrap();
        assert_eq!(var.name, "VAR1");
        assert_eq!(var.data, "val1");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .get_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .create_action_variable("owner", "repo", "VAR1", "val1")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 201);
    }

    #[tokio::test]
    async fn test_create_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(403).set_body_json(json!({"message": "Forbidden"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .create_action_variable("owner", "repo", "VAR1", "val1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .update_action_variable("owner", "repo", "VAR1", "newval")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 200);
    }

    #[tokio::test]
    async fn test_update_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("PUT"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .update_action_variable("owner", "repo", "VAR1", "newval")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_action_variable_happy() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().status, 204);
    }

    #[tokio::test]
    async fn test_delete_action_variable_error() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/actions/variables/VAR1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
            .mount(&server)
            .await;
        let result = client
            .repos()
            .delete_action_variable("owner", "repo", "VAR1")
            .await;
        assert!(result.is_err());
    }
}
