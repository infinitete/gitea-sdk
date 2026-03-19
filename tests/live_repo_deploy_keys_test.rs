// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::options::repo::{CreateKeyOption, ListDeployKeysOptions};

use live::{
    CleanupRegistry, create_repo_fixture, generate_fresh_public_key, live_client, unique_name,
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
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-deploy-key")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (existing_keys, list_resp) = client
        .repos()
        .list_deploy_keys(&owner, &repo, ListDeployKeysOptions::default())
        .await
        .expect("list deploy keys before create");
    assert_success_status(list_resp.status);

    let _ = existing_keys;
    let key = generate_fresh_public_key("live-deploy-key")
        .unwrap_or_else(|err| panic!("prepare deploy key fixture failed: {err}"));
    let title = unique_name("live-deploy-key");

    let (created, create_resp) = client
        .repos()
        .create_deploy_key(
            &owner,
            &repo,
            CreateKeyOption {
                title: title.clone(),
                key,
                read_only: true,
            },
        )
        .await
        .expect("create deploy key");
    assert_success_status(create_resp.status);
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
        .expect("list deploy keys after create");
    assert_success_status(list_resp.status);
    assert!(keys.iter().any(|deploy_key| deploy_key.id == key_id));

    let (loaded, get_resp) = client
        .repos()
        .get_deploy_key(&owner, &repo, key_id)
        .await
        .expect("get deploy key");
    assert_success_status(get_resp.status);
    assert_eq!(loaded.id, key_id);
    assert_eq!(loaded.title, title);
    assert!(loaded.read_only);

    let delete_resp = client
        .repos()
        .delete_deploy_key(&owner, &repo, key_id)
        .await
        .expect("delete deploy key");
    assert_success_status(delete_resp.status);

    cleanup.run_all().await;
}
