// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::options::repo::{CreateKeyOption, ListDeployKeysOptions};

use live::{
    cleanup::CleanupRegistry, create_repo_fixture, live_client, prepare_deploy_key_fixture,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_deploy_key_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();

    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-deploy-key-repo")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let deploy_key =
        prepare_deploy_key_fixture("live-deploy-key").expect("prepare deploy key fixture");

    let key_title = deploy_key.title.clone();
    let key_data = deploy_key.public_key.clone();

    let (created, create_resp) = client
        .repos()
        .create_deploy_key(
            &owner,
            &repo,
            CreateKeyOption {
                title: key_title.clone(),
                key: key_data.clone(),
                read_only: true,
            },
        )
        .await
        .expect("create deploy key");
    assert_eq!(create_resp.status, 201);

    let key_id = created.id;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_repo = repo.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_deploy_key(&cleanup_owner, &cleanup_repo, key_id)
            .await;
    });

    let (keys, list_resp) = client
        .repos()
        .list_deploy_keys(&owner, &repo, ListDeployKeysOptions::default())
        .await
        .expect("list deploy keys");
    assert_success_status(list_resp.status);
    assert!(keys.iter().any(|entry| entry.id == key_id));

    let (loaded, get_resp) = client
        .repos()
        .get_deploy_key(&owner, &repo, key_id)
        .await
        .expect("get deploy key");
    assert_success_status(get_resp.status);
    assert_eq!(loaded.id, key_id);
    assert_eq!(loaded.title, key_title);

    let delete_resp = client
        .repos()
        .delete_deploy_key(&owner, &repo, key_id)
        .await
        .expect("delete deploy key");
    assert_eq!(delete_resp.status, 204);

    let (after_delete, after_list_resp) = client
        .repos()
        .list_deploy_keys(&owner, &repo, ListDeployKeysOptions::default())
        .await
        .expect("list deploy keys after delete");
    assert_success_status(after_list_resp.status);
    assert!(after_delete.iter().all(|entry| entry.id != key_id));

    cleanup.run_all().await;
}
