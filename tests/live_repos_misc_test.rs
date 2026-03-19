// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::options::repo::{
    CreateForkOption, CreateRepoFromTemplateOption, EditRepoOption, ListForksOptions,
    ListReposOptions, MigrateRepoOption,
};
use gitea_rs::types::enums::GitServiceType;

use live::{CleanupRegistry, create_repo_fixture, live_client, load_live_env, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repos_misc_flow() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-repos-misc")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (_my_repos, my_repos_resp) = client
        .repos()
        .list_my_repos(ListReposOptions::default())
        .await
        .expect("list my repos");
    assert_success_status(my_repos_resp.status);

    let (loaded_by_id, by_id_resp) = client
        .repos()
        .get_repo_by_id(repo_fixture.repository.id)
        .await
        .expect("get repo by id");
    assert_success_status(by_id_resp.status);
    assert_eq!(loaded_by_id.name, repo);

    let edited_description = unique_name("live-repo-desc");
    let (edited, edit_resp) = client
        .repos()
        .edit_repo(
            &owner,
            &repo,
            EditRepoOption {
                description: Some(edited_description.clone()),
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
        .expect("edit repo");
    assert_success_status(edit_resp.status);
    assert_eq!(edited.description, edited_description);

    let template_name = unique_name("live-template-generated");
    let (templated_repo, template_resp) = client
        .repos()
        .edit_repo(
            &owner,
            &repo,
            EditRepoOption {
                template: Some(true),
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
        .expect("mark repo as template");
    assert_success_status(template_resp.status);
    assert!(templated_repo.template, "repo should be marked as template");

    let (generated_repo, generate_resp) = client
        .repos()
        .create_repo_from_template(
            &owner,
            &repo,
            CreateRepoFromTemplateOption {
                owner: owner.clone(),
                name: template_name.clone(),
                description: "generated from live template".to_string(),
                private: true,
                git_content: true,
                topics: false,
                git_hooks: false,
                webhooks: false,
                avatar: false,
                labels: false,
            },
        )
        .await
        .expect("create repo from template");
    assert_success_status(generate_resp.status);
    assert_eq!(generated_repo.name, template_name);
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_generated = generated_repo.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_generated)
            .await;
    });

    let fork_name = unique_name("live-forked-repo");
    let (forked_repo, fork_resp) = client
        .repos()
        .create_fork(
            &owner,
            &repo,
            CreateForkOption {
                organization: None,
                name: Some(fork_name.clone()),
            },
        )
        .await
        .expect("create fork");
    assert_success_status(fork_resp.status);
    assert_eq!(forked_repo.name, fork_name);
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_fork = forked_repo.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_fork)
            .await;
    });

    let (forks, forks_resp) = client
        .repos()
        .list_forks(&owner, &repo, ListForksOptions::default())
        .await
        .expect("list forks");
    assert_success_status(forks_resp.status);
    assert!(forks.iter().any(|entry| entry.name == fork_name));

    let (languages, languages_resp) = client
        .repos()
        .get_repo_languages(&owner, &repo)
        .await
        .expect("get repo languages");
    assert_success_status(languages_resp.status);
    let _ = languages;

    let default_branch = loaded_by_id.default_branch.clone();

    let (archive, archive_resp) = client
        .repos()
        .get_archive(&owner, &repo, &default_branch, "zip")
        .await
        .expect("get archive");
    assert_success_status(archive_resp.status);
    assert!(!archive.is_empty());

    let (archive_reader, archive_reader_resp) = client
        .repos()
        .get_archive_reader(&owner, &repo, &default_branch, "tar.gz")
        .await
        .expect("get archive reader");
    assert_success_status(archive_reader_resp.status);
    assert!(!archive_reader.is_empty());

    let migrate_source_name = unique_name("live-migrate-source");
    let (migrate_source, migrate_source_resp) = client
        .repos()
        .create_repo(gitea_rs::options::repo::CreateRepoOption {
            name: migrate_source_name.clone(),
            description: "live migrate source".to_string(),
            private: false,
            issue_labels: String::new(),
            auto_init: true,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: gitea_rs::types::enums::TrustModel::Default,
            object_format_name: String::new(),
        })
        .await
        .expect("create migrate source repo");
    assert_success_status(migrate_source_resp.status);
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_migrate_source = migrate_source.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_migrate_source)
            .await;
    });

    let migrated_repo_name = unique_name("live-migrated-repo");
    let (migrated_repo, migrate_resp) = client
        .repos()
        .migrate_repo(MigrateRepoOption {
            repo_name: migrated_repo_name.clone(),
            repo_owner: owner.clone(),
            uid: 0,
            clone_addr: migrate_source.clone_url.clone(),
            service: GitServiceType::Git,
            auth_username: env.user_name.clone(),
            auth_password: env.user_pass.clone(),
            auth_token: String::new(),
            mirror: false,
            private: true,
            description: "live migrated repo".to_string(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        })
        .await
        .expect("migrate repo from same instance");
    assert_success_status(migrate_resp.status);
    assert_eq!(migrated_repo.name, migrated_repo_name);
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_migrated_repo = migrated_repo.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_migrated_repo)
            .await;
    });

    let delete_fixture = create_repo_fixture(&client, &mut cleanup, "live-repos-delete")
        .await
        .expect("create explicit delete fixture");
    let delete_resp = client
        .repos()
        .delete_repo(&delete_fixture.owner, &delete_fixture.repository.name)
        .await
        .expect("delete repo");
    assert_success_status(delete_resp.status);

    cleanup.run_all().await;
}
