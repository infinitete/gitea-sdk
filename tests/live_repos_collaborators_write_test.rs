// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::repo::{AddCollaboratorOption, ListCollaboratorsOptions};
use gitea_sdk::types::enums::AccessMode;

use live::{CleanupRegistry, create_repo_fixture, live_client, load_live_env};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_collaborator_write_flow() {
    let client = live_client();
    let env = load_live_env();
    let collaborator = env
        .next_user_name
        .as_ref()
        .expect("missing GITEA_NEXT_USER_NAME in .env")
        .clone();

    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-collaborator")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let add_resp = client
        .repos()
        .add_collaborator(
            &owner,
            &repo,
            &collaborator,
            AddCollaboratorOption {
                permission: Some(AccessMode::Write),
            },
        )
        .await
        .expect("add collaborator");
    assert_success_status(add_resp.status);

    let (collaborators, list_resp) = client
        .repos()
        .list_collaborators(&owner, &repo, ListCollaboratorsOptions::default())
        .await
        .expect("list collaborators");
    assert_success_status(list_resp.status);
    assert!(
        collaborators
            .iter()
            .any(|user| user.user_name == collaborator),
        "expected second user among collaborators"
    );

    let (is_collaborator, collab_resp) = client
        .repos()
        .is_collaborator(&owner, &repo, &collaborator)
        .await
        .expect("check collaborator");
    assert!(collab_resp.status >= 200);
    assert!(is_collaborator);

    let (permission, permission_resp) = client
        .repos()
        .get_collaborator_permission(&owner, &repo, &collaborator)
        .await
        .expect("get collaborator permission");
    assert_success_status(permission_resp.status);
    let permission = permission.expect("permission should exist");
    assert_eq!(permission.permission, AccessMode::Write);

    let delete_resp = client
        .repos()
        .delete_collaborator(&owner, &repo, &collaborator)
        .await
        .expect("delete collaborator");
    assert_success_status(delete_resp.status);

    let (is_collaborator_after, collab_after_resp) = client
        .repos()
        .is_collaborator(&owner, &repo, &collaborator)
        .await
        .expect("check collaborator after delete");
    assert!(collab_after_resp.status >= 200);
    assert!(!is_collaborator_after);

    cleanup.run_all().await;
}
