// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#![allow(dead_code)]

use gitea_rs::Client;
use gitea_rs::Error;
use gitea_rs::options::issue::CreateIssueOption;
use gitea_rs::options::org::CreateOrgOption;
use gitea_rs::options::release::CreateReleaseOption;
use gitea_rs::options::repo::{CreateLabelOption, CreateRepoOption, EditRepoOption};
use gitea_rs::types::enums::TrustModel;
use gitea_rs::types::issue::Issue;
use gitea_rs::types::label::Label;
use gitea_rs::types::organization::Organization;
use gitea_rs::types::package::Package;
use gitea_rs::types::release::Release;
use gitea_rs::types::repository::{InternalTracker, Repository};

use super::cleanup::CleanupRegistry;
use super::env::load_live_env;
use super::keys::generate_fresh_public_key;
use super::naming::unique_name;

pub type LiveResult<T> = Result<T, Error>;

#[derive(Debug, Clone)]
pub struct RepoFixture {
    pub owner: String,
    pub repository: Repository,
}

#[derive(Debug, Clone)]
pub struct DeployKeyFixture {
    pub title: String,
    pub public_key: String,
}

#[derive(Debug, Clone)]
pub struct OrgFixture {
    pub organization: Organization,
}

#[derive(Debug, Clone)]
pub struct LabelFixture {
    pub owner: String,
    pub repo: String,
    pub label: Label,
}

#[derive(Debug, Clone)]
pub struct IssueFixture {
    pub owner: String,
    pub repo: String,
    pub issue: Issue,
}

#[derive(Debug, Clone)]
pub struct ReleaseFixture {
    pub owner: String,
    pub repo: String,
    pub release: Release,
}

#[derive(Debug, Clone)]
pub struct GenericPackageFixture {
    pub owner: String,
    pub package_type: String,
    pub name: String,
    pub version: String,
    pub file_name: String,
    pub package: Package,
}

pub async fn create_repo_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    prefix: &str,
) -> LiveResult<RepoFixture> {
    let owner = load_live_env().user_name.clone();
    let opt = CreateRepoOption {
        name: unique_name(prefix),
        description: "live fixture repository".to_string(),
        private: true,
        issue_labels: String::new(),
        auto_init: true,
        template: false,
        gitignores: String::new(),
        license: String::new(),
        readme: String::new(),
        default_branch: String::new(),
        trust_model: TrustModel::Default,
        object_format_name: String::new(),
    };

    let (repository, _) = client.repos().create_repo(opt).await?;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_repo = repository.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_repo)
            .await;
    });

    Ok(RepoFixture { owner, repository })
}

pub async fn create_org_repo_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    org: &str,
    prefix: &str,
) -> LiveResult<RepoFixture> {
    let opt = CreateRepoOption {
        name: unique_name(prefix),
        description: "live fixture repository".to_string(),
        private: true,
        issue_labels: String::new(),
        auto_init: true,
        template: false,
        gitignores: String::new(),
        license: String::new(),
        readme: String::new(),
        default_branch: String::new(),
        trust_model: TrustModel::Default,
        object_format_name: String::new(),
    };

    let (repository, _) = client.repos().create_org_repo(org, opt).await?;
    let cleanup_client = client.clone();
    let cleanup_org = org.to_string();
    let cleanup_repo = repository.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_org, &cleanup_repo)
            .await;
    });

    Ok(RepoFixture {
        owner: org.to_string(),
        repository,
    })
}

pub async fn create_org_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    prefix: &str,
) -> LiveResult<OrgFixture> {
    let opt = CreateOrgOption {
        name: unique_name(prefix),
        full_name: None,
        email: None,
        description: Some("live fixture organization".to_string()),
        website: None,
        location: None,
        visibility: None,
        repo_admin_change_team_access: None,
    };

    let (organization, _) = client.orgs().create_org(opt).await?;
    let cleanup_client = client.clone();
    let cleanup_org = organization.user_name.clone();
    cleanup.register(async move {
        let _ = cleanup_client.orgs().delete_org(&cleanup_org).await;
    });

    Ok(OrgFixture { organization })
}

pub async fn create_label_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    owner: &str,
    repo: &str,
    prefix: &str,
) -> LiveResult<LabelFixture> {
    let opt = CreateLabelOption {
        name: unique_name(prefix),
        color: "005cc5".to_string(),
        description: "live fixture label".to_string(),
        exclusive: false,
        is_archived: false,
    };

    let (label, _) = client.repos().create_label(owner, repo, opt).await?;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.to_string();
    let cleanup_repo = repo.to_string();
    let cleanup_id = label.id;
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_label(&cleanup_owner, &cleanup_repo, cleanup_id)
            .await;
    });

    Ok(LabelFixture {
        owner: owner.to_string(),
        repo: repo.to_string(),
        label,
    })
}

pub async fn create_issue_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    owner: &str,
    repo: &str,
    prefix: &str,
) -> LiveResult<IssueFixture> {
    let opt = CreateIssueOption {
        title: unique_name(prefix),
        body: "live fixture issue".to_string(),
        r#ref: String::new(),
        assignees: Vec::new(),
        deadline: None,
        milestone: 0,
        labels: Vec::new(),
        closed: false,
    };

    let (issue, _) = client.issues().create_issue(owner, repo, opt).await?;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.to_string();
    let cleanup_repo = repo.to_string();
    let cleanup_id = issue.id;
    cleanup.register(async move {
        let _ = cleanup_client
            .issues()
            .delete_issue(&cleanup_owner, &cleanup_repo, cleanup_id)
            .await;
    });

    Ok(IssueFixture {
        owner: owner.to_string(),
        repo: repo.to_string(),
        issue,
    })
}

pub async fn enable_issue_dependencies(
    client: &Client,
    owner: &str,
    repo: &str,
) -> LiveResult<Repository> {
    let (current, _) = client.repos().get_repo(owner, repo).await?;
    let mut tracker = current.internal_tracker.unwrap_or(InternalTracker {
        enable_time_tracker: false,
        allow_only_contributors_to_track_time: false,
        enable_issue_dependencies: false,
    });
    tracker.enable_issue_dependencies = true;

    let (updated, _) = client
        .repos()
        .edit_repo(
            owner,
            repo,
            EditRepoOption {
                has_issues: Some(true),
                internal_tracker: Some(tracker),
                name: None,
                description: None,
                website: None,
                private: None,
                template: None,
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
            },
        )
        .await?;

    Ok(updated)
}

pub async fn create_release_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    owner: &str,
    repo: &str,
    prefix: &str,
) -> LiveResult<ReleaseFixture> {
    let tag_name = unique_name(prefix);
    let opt = CreateReleaseOption {
        tag_name: tag_name.clone(),
        target: None,
        title: Some(tag_name),
        note: Some("live fixture release".to_string()),
        is_draft: false,
        is_prerelease: false,
    };

    let (release, _) = client.releases().create(owner, repo, opt).await?;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.to_string();
    let cleanup_repo = repo.to_string();
    let cleanup_id = release.id;
    cleanup.register(async move {
        let _ = cleanup_client
            .releases()
            .delete(&cleanup_owner, &cleanup_repo, cleanup_id)
            .await;
    });

    Ok(ReleaseFixture {
        owner: owner.to_string(),
        repo: repo.to_string(),
        release,
    })
}

pub async fn create_generic_package_fixture(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    prefix: &str,
) -> LiveResult<GenericPackageFixture> {
    let env = load_live_env();
    let owner = env.user_name.clone();
    let package_type = "generic".to_string();
    let name = unique_name(prefix);
    let version = "1.0.0".to_string();
    let file_name = "artifact.txt".to_string();
    let upload_url = format!(
        "{}/api/packages/{}/{}/{}/{}/{}",
        env.base_url(),
        owner,
        package_type,
        name,
        version,
        file_name
    );

    let form = reqwest::multipart::Form::new().part(
        "file",
        reqwest::multipart::Part::bytes(format!("fixture for {name}\n").into_bytes())
            .file_name(file_name.clone())
            .mime_str("text/plain")
            .map_err(Error::Request)?,
    );

    let response = reqwest::Client::new()
        .put(upload_url)
        .header(
            reqwest::header::AUTHORIZATION,
            format!("token {}", env.token_value),
        )
        .multipart(form)
        .send()
        .await?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(Error::UnknownApi {
            status: status.as_u16(),
            body,
        });
    }

    let mut loaded = None;
    for _ in 0..10 {
        if let Ok((package, _)) = client
            .packages()
            .get_package(&owner, &package_type, &name, &version)
            .await
        {
            loaded = Some(package);
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
    let package = loaded.expect("uploaded package should become readable");

    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_type = package_type.clone();
    let cleanup_name = name.clone();
    let cleanup_version = version.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .packages()
            .delete_package(
                &cleanup_owner,
                &cleanup_type,
                &cleanup_name,
                &cleanup_version,
            )
            .await;
    });

    Ok(GenericPackageFixture {
        owner,
        package_type,
        name,
        version,
        file_name,
        package,
    })
}

pub fn prepare_deploy_key_fixture(prefix: &str) -> LiveResult<DeployKeyFixture> {
    let title = unique_name(prefix);
    let public_key = generate_fresh_public_key(prefix)
        .map_err(|err| Error::Validation(format!("ssh key generation failed: {err}")))?;

    Ok(DeployKeyFixture { title, public_key })
}
