// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Live coverage for repo fork and template APIs.

mod live;

use gitea_sdk_rs::options::repo::{
    CreateForkOption, CreateRepoFromTemplateOption, CreateRepoOption, ListForksOptions,
};
use gitea_sdk_rs::types::enums::TrustModel;

use live::{CleanupRegistry, build_live_client, create_repo_fixture, load_live_env, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {}",
        status
    );
}

#[tokio::test]
#[ignore = "requires live Gitea instance configured in ../.env"]
async fn live_repos_fork_template() {
    let env = load_live_env();
    let client = build_live_client(env);
    let mut cleanup = CleanupRegistry::new();

    let base = create_repo_fixture(&client, &mut cleanup, "live-fork-base")
        .await
        .expect("create base repo");
    let owner = base.owner.clone();
    let repo_name = base.repository.name.clone();

    let fork_name = unique_name("fork");
    let (fork_repo, fork_resp) = client
        .repos()
        .create_fork(
            &owner,
            &repo_name,
            CreateForkOption {
                organization: None,
                name: Some(fork_name.clone()),
            },
        )
        .await
        .expect("create fork");
    assert_success_status(fork_resp.status);

    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_name = fork_repo.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_name)
            .await;
    });

    let (forks, list_resp) = client
        .repos()
        .list_forks(&owner, &repo_name, ListForksOptions::default())
        .await
        .expect("list forks");
    assert_success_status(list_resp.status);
    assert!(forks.iter().any(|repo| repo.name == fork_repo.name));

    let template_name = unique_name("template-base");
    let template_opt = CreateRepoOption {
        name: template_name.clone(),
        description: "live template repo".into(),
        private: true,
        issue_labels: String::new(),
        auto_init: true,
        template: true,
        gitignores: String::new(),
        license: String::new(),
        readme: String::new(),
        default_branch: String::new(),
        trust_model: TrustModel::Default,
        object_format_name: String::new(),
    };

    let (template_repo, template_resp) = client
        .repos()
        .create_repo(template_opt)
        .await
        .expect("create template repo");
    assert_success_status(template_resp.status);

    let cleanup_template_client = client.clone();
    let cleanup_template_name = template_repo.name.clone();
    let cleanup_template_owner = owner.clone();
    cleanup.register(async move {
        let _ = cleanup_template_client
            .repos()
            .delete_repo(&cleanup_template_owner, &cleanup_template_name)
            .await;
    });

    let generated_name = unique_name("template-child");
    let template_from_opt = CreateRepoFromTemplateOption {
        owner: owner.clone(),
        name: generated_name.clone(),
        description: "live child from template".into(),
        private: true,
        git_content: true,
        topics: false,
        git_hooks: false,
        webhooks: false,
        avatar: false,
        labels: false,
    };

    let (generated, generated_resp) = client
        .repos()
        .create_repo_from_template(&owner, &template_repo.name, template_from_opt)
        .await
        .expect("create from template");
    assert_success_status(generated_resp.status);
    assert_eq!(generated.name, generated_name);

    let cleanup_generated_client = client.clone();
    let cleanup_generated_name = generated.name.clone();
    let cleanup_generated_owner = owner.clone();
    cleanup.register(async move {
        let _ = cleanup_generated_client
            .repos()
            .delete_repo(&cleanup_generated_owner, &cleanup_generated_name)
            .await;
    });

    cleanup.run_all().await;
}
