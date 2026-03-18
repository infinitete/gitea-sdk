// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::repo::{CreateTagOption, ListRepoTagsOptions};

use live::{CleanupRegistry, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_tag_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-repo-tag")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (repo_data, repo_resp) = client
        .repos()
        .get_repo(&owner, &repo)
        .await
        .expect("get repo");
    assert_success_status(repo_resp.status);

    let tag_name = unique_name("live-tag");
    let (created, create_resp) = client
        .repos()
        .create_tag(
            &owner,
            &repo,
            CreateTagOption {
                tag_name: tag_name.clone(),
                message: "live annotated tag".to_string(),
                target: repo_data.default_branch.clone(),
            },
        )
        .await
        .expect("create tag");
    assert_success_status(create_resp.status);
    assert_eq!(created.name, tag_name);

    let (tags, list_resp) = client
        .repos()
        .list_tags(&owner, &repo, ListRepoTagsOptions::default())
        .await
        .expect("list tags");
    assert_success_status(list_resp.status);
    assert!(tags.iter().any(|tag| tag.name == tag_name));

    let (loaded, get_resp) = client
        .repos()
        .get_tag(&owner, &repo, &tag_name)
        .await
        .expect("get tag");
    assert_success_status(get_resp.status);
    assert_eq!(loaded.name, tag_name);

    let (annotated, annotated_resp) = client
        .repos()
        .get_annotated_tag(&owner, &repo, &created.id)
        .await
        .expect("get annotated tag");
    assert_success_status(annotated_resp.status);
    assert_eq!(annotated.tag, tag_name);

    let delete_resp = client
        .repos()
        .delete_tag(&owner, &repo, &tag_name)
        .await
        .expect("delete tag");
    assert_success_status(delete_resp.status);

    cleanup.run_all().await;
}
