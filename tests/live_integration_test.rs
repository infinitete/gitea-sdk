// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Live integration tests against a real Gitea instance.
//!
//! These tests are ignored by default and expect the repository root `.env`
//! to define at least:
//! - `GITEA_HOST`
//! - `GITEA_HTTP_PORT`
//! - `GITEA_USER_NAME`
//! - `GITEA_TOKEN_VALUE`
//!
//! Run them explicitly with:
//! `cargo test --test live_integration_test -- --ignored --nocapture`

mod live;

use gitea_sdk_rs::options::user::CreateKeyOption;

use live::{
    CleanupRegistry, build_live_client, create_org_fixture, create_repo_fixture, live_client,
    load_live_env, load_public_key_from_env, load_public_key_from_repo,
    load_unused_or_generate_public_key, unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

fn unique_key_title() -> String {
    unique_name("live-test-key")
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_server_version() {
    let env = load_live_env();
    let client = build_live_client(env);
    let mut cleanup = CleanupRegistry::new();
    cleanup.register(async {});
    let version = client.server_version().await.expect("live version");
    cleanup.run_all().await;
    assert!(!version.is_empty());
}

#[test]
fn live_load_public_key_fixture() {
    let env = load_live_env();
    let public_key = load_public_key_from_env(env).expect("load public key from repo path");
    assert!(public_key.starts_with("ssh-"));

    if let Some(path) = env.ed25519_public_key.as_deref() {
        let loaded = load_public_key_from_repo(path).expect("load configured ed25519 public key");
        assert_eq!(loaded, public_key);
    }
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_fixture_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-fixture-repo")
        .await
        .expect("create repo fixture");
    assert_eq!(fixture.owner, load_live_env().user_name);
    assert!(!fixture.repository.name.is_empty());
    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_org_fixture_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    match create_org_fixture(&client, &mut cleanup, "live-fixture-org").await {
        Ok(fixture) => {
            assert!(!fixture.organization.user_name.is_empty());
            cleanup.run_all().await;
        }
        Err(gitea_sdk_rs::Error::Api {
            status: 401 | 403 | 404,
            ..
        }) => {
            cleanup.run_all().await;
        }
        Err(err) => {
            cleanup.run_all().await;
            panic!("create org fixture failed unexpectedly: {err}");
        }
    }
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_get_my_info() {
    let env = load_live_env();
    let client = live_client();
    let (user, response) = client.users().get_my_info().await.expect("get my info");
    assert_success_status(response.status);
    assert_eq!(user.user_name, env.user_name);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_list_my_public_keys() {
    let client = live_client();
    let (_keys, response) = client
        .users()
        .list_my_public_keys(Default::default())
        .await
        .expect("list my public keys");
    assert_success_status(response.status);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_public_key_lifecycle() {
    let client = live_client();
    let title = unique_key_title();
    let (existing_keys, list_response) = client
        .users()
        .list_my_public_keys(Default::default())
        .await
        .expect("list existing public keys before create");
    assert_success_status(list_response.status);
    let key = load_unused_or_generate_public_key(
        load_live_env(),
        &existing_keys
            .iter()
            .map(|public_key| public_key.key.clone())
            .collect::<Vec<_>>(),
        "live-test-key",
    )
    .unwrap_or_else(|err| panic!("prepare public key fixture failed: {err}"));
    let create_opt = CreateKeyOption {
        title: title.clone(),
        key,
        read_only: false,
    };

    let (created, create_response) = client
        .users()
        .create_public_key(create_opt)
        .await
        .expect("create public key");
    assert_success_status(create_response.status);

    let key_id = created.id;

    let fetched_result = client.users().get_public_key(key_id).await;
    let (fetched, fetch_response) = match fetched_result {
        Ok(ok) => ok,
        Err(err) => {
            let _ = client.users().delete_public_key(key_id).await;
            panic!("fetch public key failed after create: {err}");
        }
    };
    assert_success_status(fetch_response.status);
    assert_eq!(fetched.id, key_id);
    assert_eq!(fetched.title.as_deref(), Some(title.as_str()));

    let delete_result = client.users().delete_public_key(key_id).await;
    let delete_response = delete_result.unwrap_or_else(|err| {
        panic!("delete public key failed for key_id={key_id}: {err}");
    });
    assert_success_status(delete_response.status);
}
