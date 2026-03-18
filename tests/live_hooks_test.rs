// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use std::collections::HashMap;

use gitea_sdk::options::hook::{CreateHookOption, EditHookOption, ListHooksOptions};
use gitea_sdk::types::enums::HookType;

use live::{CleanupRegistry, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

fn live_hook_config(prefix: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert(
        "url".to_string(),
        format!("https://example.com/hooks/{}", unique_name(prefix)),
    );
    config.insert("content_type".to_string(), "json".to_string());
    config
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_hook_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();

    let config = live_hook_config("user-hook");
    let (hook, create_response) = client
        .hooks()
        .create_my_hook(CreateHookOption {
            hook_type: HookType::Gitea,
            config,
            events: vec!["push".to_string()],
            branch_filter: None,
            active: true,
            authorization_header: None,
        })
        .await
        .expect("create user hook");
    assert_success_status(create_response.status);

    let cleanup_client = client.clone();
    let hook_id = hook.id;
    cleanup.register(async move {
        let _ = cleanup_client.hooks().delete_my_hook(hook_id).await;
    });

    let (loaded, get_response) = client
        .hooks()
        .get_my_hook(hook_id)
        .await
        .expect("get user hook");
    assert_success_status(get_response.status);
    assert_eq!(loaded.id, hook_id);

    let (hooks, list_response) = client
        .hooks()
        .list_my_hooks(ListHooksOptions::default())
        .await
        .expect("list user hooks");
    assert_success_status(list_response.status);
    assert!(hooks.iter().any(|entry| entry.id == hook_id));

    let edit_response = client
        .hooks()
        .edit_my_hook(
            hook_id,
            EditHookOption {
                active: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("edit user hook");
    assert_success_status(edit_response.status);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_hook_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-hook-repo")
        .await
        .expect("create hook repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let config = live_hook_config("repo-hook");
    let (hook, create_response) = client
        .hooks()
        .create_repo_hook(
            &owner,
            &repo,
            CreateHookOption {
                hook_type: HookType::Gitea,
                config,
                events: vec!["push".to_string()],
                branch_filter: None,
                active: true,
                authorization_header: None,
            },
        )
        .await
        .expect("create repo hook");
    assert_success_status(create_response.status);

    let cleanup_client = client.clone();
    let hook_id = hook.id;
    let owner_clone = owner.clone();
    let repo_clone = repo.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .hooks()
            .delete_repo_hook(&owner_clone, &repo_clone, hook_id)
            .await;
    });

    let (loaded, get_response) = client
        .hooks()
        .get_repo_hook(&owner, &repo, hook_id)
        .await
        .expect("get repo hook");
    assert_success_status(get_response.status);
    assert_eq!(loaded.id, hook_id);

    let (hooks, list_response) = client
        .hooks()
        .list_repo_hooks(&owner, &repo, ListHooksOptions::default())
        .await
        .expect("list repo hooks");
    assert_success_status(list_response.status);
    assert!(hooks.iter().any(|entry| entry.id == hook_id));

    let edit_response = client
        .hooks()
        .edit_repo_hook(
            &owner,
            &repo,
            hook_id,
            EditHookOption {
                active: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("edit repo hook");
    assert_success_status(edit_response.status);

    cleanup.run_all().await;
}
