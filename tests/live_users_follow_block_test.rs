// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Dual-user follow and block coverage exercising the live Gitea instance.

mod live;

use gitea_sdk_rs::Client;
use gitea_sdk_rs::options::admin::CreateUserOption;
use gitea_sdk_rs::options::user::{
    ListFollowersOptions, ListFollowingOptions, ListUserBlocksOptions,
};

use reqwest::Client as HttpClient;

use live::{CleanupRegistry, build_live_client, load_live_env, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires live Gitea instance configured in ../.env"]
async fn live_users_follow_block() {
    let env = load_live_env();
    let client = build_live_client(env);
    let base_url = client.base_url();

    let second_username = unique_name("live-follow-user");
    let second_password = unique_name("follow-pass");
    let email = format!("{second_username}@example.com");
    let create_opt = CreateUserOption {
        source_id: None,
        login_name: None,
        username: second_username.clone(),
        full_name: None,
        email,
        password: second_password.clone(),
        must_change_password: Some(false),
        send_notify: false,
        visibility: None,
    };

    let (second_user, create_resp) = client
        .admin()
        .create_user(create_opt)
        .await
        .expect("create helper user");
    assert_success_status(create_resp.status);

    let http = HttpClient::new();
    let second_client = Client::builder(&base_url)
        .basic_auth(&second_username, &second_password)
        .http_client(http)
        .build()
        .expect("build second user client");

    let mut cleanup = CleanupRegistry::new();
    let cleanup_second_client = second_client.clone();
    let cleanup_primary_user = env.user_name.clone();
    cleanup.register(async move {
        let _ = cleanup_second_client
            .users()
            .unfollow(&cleanup_primary_user)
            .await;
    });
    let cleanup_admin_client = client.clone();
    let cleanup_user = second_username.clone();
    cleanup.register(async move {
        let _ = cleanup_admin_client
            .admin()
            .delete_user(&cleanup_user)
            .await;
    });

    let follow_resp = second_client
        .users()
        .follow(&env.user_name)
        .await
        .expect("second user follow");
    assert_success_status(follow_resp.status);

    let (followers, follow_response) = client
        .users()
        .list_my_followers(ListFollowersOptions::default())
        .await
        .expect("list my followers");
    assert_success_status(follow_response.status);
    assert!(
        followers
            .iter()
            .any(|user| user.user_name == second_username)
    );

    let (following, following_response) = second_client
        .users()
        .list_my_following(ListFollowingOptions::default())
        .await
        .expect("list following from second user");
    assert_success_status(following_response.status);
    assert!(following.iter().any(|user| user.user_name == env.user_name));

    let (followers_of_primary, list_followers_resp) = client
        .users()
        .list_followers(&env.user_name, ListFollowersOptions::default())
        .await
        .expect("list followers for primary user");
    assert_success_status(list_followers_resp.status);
    assert!(
        followers_of_primary
            .iter()
            .any(|user| user.user_name == second_username)
    );

    let block_resp = client
        .users()
        .block_user(&second_username)
        .await
        .expect("block helper user");
    assert_success_status(block_resp.status);

    let (blocked, check_resp) = client
        .users()
        .check_user_block(&second_username)
        .await
        .expect("check block status");
    assert_success_status(check_resp.status);
    assert!(blocked);

    let (blocks, list_blocks_resp) = client
        .users()
        .list_my_blocks(ListUserBlocksOptions::default())
        .await
        .expect("list blocked users");
    assert_success_status(list_blocks_resp.status);
    assert!(blocks.iter().any(|user| user.user_name == second_username));

    let unblock_resp = client
        .users()
        .unblock_user(&second_username)
        .await
        .expect("unblock helper user");
    assert_success_status(unblock_resp.status);

    let (blocked_again, check_again_resp) = client
        .users()
        .check_user_block(&second_username)
        .await
        .expect("check block status again");
    assert_success_status(check_again_resp.status);
    assert!(!blocked_again);

    cleanup.run_all().await;
    assert_eq!(second_user.user_name, second_username);
}
