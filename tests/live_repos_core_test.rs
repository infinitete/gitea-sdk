// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::repo::{CreateLabelOption, ListLabelsOptions, ListReposOptions};

use live::{CleanupRegistry, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_lifecycle_and_search() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-repo-core")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo_name = fixture.repository.name.clone();

    let (loaded, get_response) = client
        .repos()
        .get_repo(&owner, &repo_name)
        .await
        .expect("get repo");
    assert_success_status(get_response.status);
    assert_eq!(loaded.name, repo_name);

    let (user_repos, list_response) = client
        .repos()
        .list_user_repos(&owner, ListReposOptions::default())
        .await
        .expect("list user repos");
    assert_success_status(list_response.status);
    assert!(user_repos.iter().any(|repo| repo.name == repo_name));

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_label_cycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-label-repo")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let label_name = unique_name("live-label");
    let (label, create_response) = client
        .repos()
        .create_label(
            &owner,
            &repo,
            CreateLabelOption {
                name: label_name.clone(),
                color: "ff5500".into(),
                description: "live label".into(),
                exclusive: false,
                is_archived: false,
            },
        )
        .await
        .expect("create label");
    assert_success_status(create_response.status);

    let (loaded, get_response) = client
        .repos()
        .get_label(&owner, &repo, label.id)
        .await
        .expect("get label");
    assert_success_status(get_response.status);
    assert_eq!(loaded.name, label_name);

    let (labels, list_response) = client
        .repos()
        .list_labels(&owner, &repo, ListLabelsOptions::default())
        .await
        .expect("list labels");
    assert_success_status(list_response.status);
    assert!(labels.iter().any(|item| item.id == label.id));

    let delete_response = client
        .repos()
        .delete_label(&owner, &repo, label.id)
        .await
        .expect("delete label");
    assert_success_status(delete_response.status);

    cleanup.run_all().await;
}
