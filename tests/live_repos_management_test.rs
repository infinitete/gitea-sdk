// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

use gitea_sdk_rs::Error;
use gitea_sdk_rs::options::org::CreateSecretOption;
use gitea_sdk_rs::options::repo::{
    CreateBranchOption, CreateLabelOption, CreateRepoOption, EditLabelOption, EditRepoOption,
    ListOrgReposOptions, SearchRepoOptions, UpdateRepoBranchOption,
};
use gitea_sdk_rs::types::enums::TrustModel;
use tokio::time::sleep;

use live::{CleanupRegistry, create_org_fixture, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

const ONE_PIXEL_PNG: &[u8] = &[
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x04, 0x00, 0x00, 0x00, 0xB5, 0x1C, 0x0C,
    0x02, 0x00, 0x00, 0x00, 0x0B, 0x49, 0x44, 0x41, 0x54, 0x78, 0xDA, 0x63, 0xFC, 0xFF, 0x1F, 0x00,
    0x03, 0x03, 0x02, 0x00, 0xEF, 0xBF, 0x95, 0x51, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44,
    0xAE, 0x42, 0x60, 0x82,
];

fn short_live_name(prefix: &str) -> String {
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_millis()
        % 100_000;
    format!("{prefix}{millis}")
}

async fn search_repo_until_visible(
    client: &gitea_sdk_rs::Client,
    keyword: &str,
    expected_repo_name: &str,
) {
    for attempt in 0..10 {
        let (repos, resp) = client
            .repos()
            .search_repos(SearchRepoOptions {
                keyword: keyword.to_string(),
                ..SearchRepoOptions::default()
            })
            .await
            .expect("search repos");
        assert_success_status(resp.status);
        if repos.iter().any(|repo| repo.name == expected_repo_name) {
            return;
        }

        if attempt < 9 {
            sleep(Duration::from_millis(500)).await;
        }
    }

    panic!("search results never contained repo {expected_repo_name}");
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_org_creation_listing_and_search() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let org_fixture = create_org_fixture(&client, &mut cleanup, "lorg")
        .await
        .expect("create org fixture");
    let org_name = org_fixture.organization.user_name.clone();
    let repo_name = unique_name("live-org-owned-repo");

    let (created_repo, create_resp) = client
        .repos()
        .create_org_repo(
            &org_name,
            CreateRepoOption {
                name: repo_name.clone(),
                description: "live org repo fixture".to_string(),
                private: false,
                issue_labels: String::new(),
                auto_init: true,
                template: false,
                gitignores: String::new(),
                license: String::new(),
                readme: String::new(),
                default_branch: String::new(),
                trust_model: TrustModel::Default,
                object_format_name: String::new(),
            },
        )
        .await
        .expect("create org repo");
    assert_success_status(create_resp.status);
    assert_eq!(created_repo.name, repo_name);

    let (org_repos, list_org_repos_resp) = client
        .repos()
        .list_org_repos(&org_name, ListOrgReposOptions::default())
        .await
        .expect("list org repos");
    assert_success_status(list_org_repos_resp.status);
    assert!(org_repos.iter().any(|repo| repo.name == repo_name));

    search_repo_until_visible(&client, &repo_name, &repo_name).await;

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_label_branch_and_actions_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-repo-manage")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();
    let default_branch = fixture.repository.default_branch.clone();

    let (label, create_label_resp) = client
        .repos()
        .create_label(
            &owner,
            &repo,
            CreateLabelOption {
                name: unique_name("live-edit-label"),
                color: "005cc5".to_string(),
                description: "before edit".to_string(),
                exclusive: false,
                is_archived: false,
            },
        )
        .await
        .expect("create label");
    assert_success_status(create_label_resp.status);

    let edited_name = unique_name("live-edited-label");
    let (edited_label, edit_label_resp) = client
        .repos()
        .edit_label(
            &owner,
            &repo,
            label.id,
            EditLabelOption {
                name: Some(edited_name.clone()),
                color: Some("ffaa00".to_string()),
                description: Some("after edit".to_string()),
                exclusive: Some(false),
                is_archived: Some(false),
            },
        )
        .await
        .expect("edit label");
    assert_success_status(edit_label_resp.status);
    assert_eq!(edited_label.name, edited_name);

    let source_branch = unique_name("live-branch-src");
    let (_, create_branch_resp) = client
        .repos()
        .create_branch(
            &owner,
            &repo,
            CreateBranchOption {
                branch_name: source_branch.clone(),
                old_branch_name: default_branch.clone(),
            },
        )
        .await
        .expect("create branch");
    assert_success_status(create_branch_resp.status);

    let renamed_branch = unique_name("live-branch-renamed");
    let (updated_branch, update_branch_resp) = client
        .repos()
        .update_branch(
            &owner,
            &repo,
            &source_branch,
            UpdateRepoBranchOption {
                name: renamed_branch.clone(),
            },
        )
        .await
        .expect("update branch");
    assert_success_status(update_branch_resp.status);
    assert_eq!(updated_branch.name, renamed_branch);

    let (_, edit_repo_resp) = client
        .repos()
        .edit_repo(
            &owner,
            &repo,
            EditRepoOption {
                has_actions: Some(true),
                ..EditRepoOption {
                    name: None,
                    description: None,
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
                    default_delete_branch_after_merge: None,
                    default_merge_style: None,
                    archived: None,
                    mirror_interval: None,
                    allow_manual_merge: None,
                    autodetect_manual_merge: None,
                    projects_mode: None,
                }
            },
        )
        .await
        .expect("enable actions");
    assert_success_status(edit_repo_resp.status);

    let variable_name = short_live_name("VAR_").to_uppercase();
    let create_variable_resp = client
        .repos()
        .create_action_variable(&owner, &repo, &variable_name, "initial")
        .await
        .expect("create action variable");
    assert_success_status(create_variable_resp.status);

    let (variables, list_variables_resp) = client
        .repos()
        .list_action_variables(&owner, &repo, Default::default())
        .await
        .expect("list action variables");
    assert_success_status(list_variables_resp.status);
    assert!(variables.iter().any(|entry| entry.name == variable_name));

    let (variable, get_variable_resp) = client
        .repos()
        .get_action_variable(&owner, &repo, &variable_name)
        .await
        .expect("get action variable");
    assert_success_status(get_variable_resp.status);
    assert_eq!(variable.name, variable_name);

    let update_variable_resp = client
        .repos()
        .update_action_variable(&owner, &repo, &variable_name, "updated")
        .await
        .expect("update action variable");
    assert_success_status(update_variable_resp.status);

    let (updated_variable, get_updated_variable_resp) = client
        .repos()
        .get_action_variable(&owner, &repo, &variable_name)
        .await
        .expect("get updated action variable");
    assert_success_status(get_updated_variable_resp.status);
    assert_eq!(updated_variable.data, "updated");

    let delete_variable_resp = client
        .repos()
        .delete_action_variable(&owner, &repo, &variable_name)
        .await
        .expect("delete action variable");
    assert_success_status(delete_variable_resp.status);

    let secret_name = short_live_name("SEC_").to_uppercase();
    let create_secret_resp = client
        .repos()
        .create_action_secret(
            &owner,
            &repo,
            CreateSecretOption {
                name: secret_name.clone(),
                data: "super-secret-value".to_string(),
                description: Some("live repo secret".to_string()),
            },
        )
        .await
        .expect("create action secret");
    assert_success_status(create_secret_resp.status);

    let (secrets, list_secrets_resp) = client
        .repos()
        .list_action_secrets(&owner, &repo, Default::default())
        .await
        .expect("list action secrets");
    assert_success_status(list_secrets_resp.status);
    assert!(secrets.iter().any(|entry| entry.name == secret_name));

    let delete_secret_resp = client
        .repos()
        .delete_action_secret(&owner, &repo, &secret_name)
        .await
        .expect("delete action secret");
    assert_success_status(delete_secret_resp.status);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_avatar_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-repo-avatar")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    match client
        .repos()
        .update_repo_avatar(&owner, &repo, ONE_PIXEL_PNG)
        .await
    {
        Ok((updated_repo, update_avatar_resp)) => {
            assert_success_status(update_avatar_resp.status);
            assert_eq!(updated_repo.name, repo);

            let delete_avatar_resp = client
                .repos()
                .delete_repo_avatar(&owner, &repo)
                .await
                .expect("delete repo avatar");
            assert_success_status(delete_avatar_resp.status);
        }
        Err(Error::UnknownApi { status: 405, .. }) => {
            println!(
                "[repo avatar capability] live instance returned HTTP 405 for repo avatar endpoints; keeping avatar methods blocked on this instance"
            );
        }
        Err(other) => panic!("update repo avatar: {other}"),
    }

    cleanup.run_all().await;
}
