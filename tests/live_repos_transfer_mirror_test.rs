// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::Client;
use gitea_rs::Error;
use gitea_rs::options::repo::{
    CreatePushMirrorOption, CreateRepoOption, ListPushMirrorOptions, MigrateRepoOption,
    TransferRepoOption,
};
use gitea_rs::types::enums::{GitServiceType, TrustModel};

use live::{
    CleanupRegistry, create_repo_fixture, live_client, load_live_env, next_user_client, unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

async fn create_repo_for_client(
    client: &Client,
    cleanup: &mut CleanupRegistry,
    owner: &str,
    prefix: &str,
) -> gitea_rs::Result<String> {
    let (repo, _) = client
        .repos()
        .create_repo(CreateRepoOption {
            name: unique_name(prefix),
            description: "live seeded repository".to_string(),
            private: true,
            issue_labels: String::new(),
            auto_init: true,
            template: false,
            gitignores: String::new(),
            license: String::new(),
            readme: String::new(),
            default_branch: String::new(),
            trust_model: TrustModel::Default,
            object_format_name: String::new(),
        })
        .await?;

    let cleanup_client = client.clone();
    let cleanup_owner = owner.to_string();
    let cleanup_repo = repo.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_repo)
            .await;
    });

    Ok(repo.name)
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_transfer_flow() {
    let client = live_client();
    let next_client = next_user_client();
    let env = load_live_env();

    let mut cleanup = CleanupRegistry::new();
    let accept_fixture = create_repo_fixture(&client, &mut cleanup, "live-transfer-accept")
        .await
        .expect("create accept repo fixture");
    let accept_owner = accept_fixture.owner.clone();
    let accept_repo = accept_fixture.repository.name.clone();
    let next_owner = env
        .next_user_name
        .clone()
        .expect("missing GITEA_NEXT_USER_NAME");

    let (pending_accept_repo, accept_transfer_resp) = client
        .repos()
        .transfer_repo(
            &accept_owner,
            &accept_repo,
            TransferRepoOption {
                new_owner: env
                    .next_user_name
                    .clone()
                    .expect("missing GITEA_NEXT_USER_NAME"),
                team_ids: None,
            },
        )
        .await
        .expect("transfer repo for accept");
    assert_success_status(accept_transfer_resp.status);
    assert_eq!(pending_accept_repo.name, accept_repo);

    let (accepted_repo, accept_resp) = next_client
        .repos()
        .get_repo(&next_owner, &accept_repo)
        .await
        .expect("get transferred repo from new owner");
    assert_success_status(accept_resp.status);
    assert_eq!(
        accepted_repo.owner.expect("repo owner").user_name,
        next_owner
    );

    let cleanup_next_client = next_client.clone();
    let cleanup_next_owner = next_owner.clone();
    let cleanup_accept_repo = accept_repo.clone();
    cleanup.register(async move {
        let _ = cleanup_next_client
            .repos()
            .delete_repo(&cleanup_next_owner, &cleanup_accept_repo)
            .await;
    });
    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_push_mirror_flow() {
    let client = live_client();
    let next_client = next_user_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();

    let source_fixture = create_repo_fixture(&client, &mut cleanup, "live-push-mirror-src")
        .await
        .expect("create source repo");
    let source_owner = source_fixture.owner.clone();
    let source_repo = source_fixture.repository.name.clone();

    let next_owner = env
        .next_user_name
        .clone()
        .expect("missing GITEA_NEXT_USER_NAME");
    let target_repo = create_repo_for_client(
        &next_client,
        &mut cleanup,
        &next_owner,
        "live-push-mirror-dst",
    )
    .await
    .expect("create target repo");

    let remote_address = format!("{}/{}/{}.git", env.base_url(), next_owner, target_repo);

    let (created_mirror, create_resp) = client
        .repos()
        .create_push_mirror(
            &source_owner,
            &source_repo,
            CreatePushMirrorOption {
                interval: "8h".to_string(),
                remote_address: remote_address.clone(),
                remote_password: env
                    .next_user_pass
                    .clone()
                    .expect("missing GITEA_NEXT_USER_PASS"),
                remote_username: next_owner.clone(),
                sync_on_commit: false,
            },
        )
        .await
        .expect("create push mirror");
    assert_success_status(create_resp.status);
    assert_eq!(created_mirror.remote_address, remote_address);

    let (mirrors, list_resp) = client
        .repos()
        .list_push_mirrors(
            &source_owner,
            &source_repo,
            ListPushMirrorOptions::default(),
        )
        .await
        .expect("list push mirrors");
    assert_success_status(list_resp.status);
    assert!(
        mirrors
            .iter()
            .any(|entry| entry.remote_address == remote_address)
    );

    let remote_name = created_mirror.remote_name.clone();

    match client
        .repos()
        .get_push_mirror(&source_owner, &source_repo, &remote_name)
        .await
    {
        Ok((mirror, get_resp)) => {
            assert_success_status(get_resp.status);
            assert_eq!(mirror.remote_address, remote_address);
        }
        Err(Error::Api {
            status, message, ..
        }) => {
            panic!("get push mirror failed with HTTP {status}: {message}");
        }
        Err(Error::UnknownApi { status, body }) => {
            panic!("get push mirror failed with HTTP {status}: {body}");
        }
        Err(other) => panic!("get push mirror: {other}"),
    }

    match client
        .repos()
        .mirror_sync(&source_owner, &source_repo)
        .await
    {
        Ok(mirror_sync_resp) => assert_success_status(mirror_sync_resp.status),
        Err(Error::Api {
            status, message, ..
        }) if status == 400 && message.contains("not a mirror") => {
            println!(
                "[mirror sync capability] live instance returned HTTP 400 Repository is not a mirror for push-mirror-backed repos"
            );
        }
        Err(other) => panic!("mirror sync: {other}"),
    }

    let delete_resp = client
        .repos()
        .delete_push_mirror(&source_owner, &source_repo, &remote_name)
        .await
        .expect("delete push mirror");
    assert_success_status(delete_resp.status);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_true_mirror_sync_flow() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();

    let (source_repo, source_resp) = client
        .repos()
        .create_repo(CreateRepoOption {
            name: unique_name("live-true-mirror-src"),
            description: "live true mirror source".to_string(),
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
        })
        .await
        .expect("create true mirror source");
    assert_success_status(source_resp.status);
    let source_owner = env.user_name.clone();
    let source_repo_name = source_repo.name.clone();
    let cleanup_client = client.clone();
    let cleanup_owner = source_owner.clone();
    let cleanup_source_repo = source_repo_name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_source_repo)
            .await;
    });

    let mirror_repo_name = unique_name("live-true-mirror");
    let (mirror_repo, migrate_resp) = client
        .repos()
        .migrate_repo(MigrateRepoOption {
            repo_name: mirror_repo_name.clone(),
            repo_owner: source_owner.clone(),
            uid: 0,
            clone_addr: source_repo.clone_url.clone(),
            service: GitServiceType::Git,
            auth_username: env.user_name.clone(),
            auth_password: env.user_pass.clone(),
            auth_token: String::new(),
            mirror: true,
            private: true,
            description: "live true mirror repo".to_string(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: "8h".to_string(),
            lfs: false,
            lfs_endpoint: String::new(),
        })
        .await
        .expect("migrate true mirror repo");
    assert_success_status(migrate_resp.status);
    assert_eq!(mirror_repo.name, mirror_repo_name);
    let cleanup_client = client.clone();
    let cleanup_owner = source_owner.clone();
    let cleanup_mirror_repo = mirror_repo.name.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_repo(&cleanup_owner, &cleanup_mirror_repo)
            .await;
    });

    let mirror_sync_resp = client
        .repos()
        .mirror_sync(&source_owner, &mirror_repo_name)
        .await
        .expect("mirror sync true mirror repo");
    assert_success_status(mirror_sync_resp.status);

    cleanup.run_all().await;
}
