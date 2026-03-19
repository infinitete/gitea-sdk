// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::Error;
use gitea_sdk::options::repo::{EditGitHookOption, ListRepoGitHooksOptions};

use live::{CleanupRegistry, create_repo_fixture, live_client};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_git_hooks_read_and_edit() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-git-hook")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (hooks, list_resp) = match client
        .repos()
        .list_git_hooks(&owner, &repo, ListRepoGitHooksOptions::default())
        .await
    {
        Ok(result) => result,
        Err(Error::Api {
            status, message, ..
        }) if status == 403 && message.contains("must be allowed to edit Git hooks") => {
            println!(
                "[repo git hooks] live instance denied access with HTTP 403 'must be allowed to edit Git hooks'; keeping git-hook methods blocked"
            );
            cleanup.run_all().await;
            return;
        }
        Err(other) => panic!("list git hooks: {other}"),
    };
    assert_success_status(list_resp.status);
    assert!(!hooks.is_empty());

    let hook_name = hooks[0].name.clone();
    let (loaded, get_resp) = client
        .repos()
        .get_git_hook(&owner, &repo, &hook_name)
        .await
        .expect("get git hook");
    assert_success_status(get_resp.status);
    assert_eq!(loaded.name, hook_name);
    let original_content = loaded.content.clone().unwrap_or_default();

    let new_content = if original_content.is_empty() {
        "#!/bin/sh\nexit 0\n".to_string()
    } else {
        original_content.clone()
    };
    let (edited, edit_resp) = client
        .repos()
        .edit_git_hook(
            &owner,
            &repo,
            &hook_name,
            EditGitHookOption {
                content: new_content,
            },
        )
        .await
        .expect("edit git hook");
    assert_success_status(edit_resp.status);
    assert_eq!(edited.name, hook_name);

    cleanup.run_all().await;
}
