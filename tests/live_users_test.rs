// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use gitea_rs::options::user::{
    CreateAccessTokenOption, CreateEmailOption, CreateGPGKeyOption, DeleteEmailOption,
    ListAccessTokensOptions, ListEmailsOptions, ListGPGKeysOptions, ListPublicKeysOptions,
    ListUserActivityFeedsOptions, SearchUsersOption, UpdateUserAvatarOption, UserSettingsOptions,
    VerifyGPGKeyOption,
};
use gitea_rs::types::enums::AccessTokenScope;

use live::{
    CleanupRegistry, build_basic_auth_client, create_repo_fixture, live_client, load_live_env,
    unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

const AVATAR_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAIAAACQd1PeAAAAC0lEQVR42mNgYAAAAAMAASsJTYQAAAAASUVORK5CYII=";

struct GeneratedGpgKey {
    home: PathBuf,
    user_id: String,
    armored_public_key: String,
}

fn run_gpg(home: &PathBuf, args: &[&str]) -> Vec<u8> {
    let output = Command::new("gpg")
        .arg("--homedir")
        .arg(home)
        .args(args)
        .output()
        .expect("run gpg");
    assert!(
        output.status.success(),
        "gpg {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

fn generate_gpg_key(prefix: &str, email: &str) -> GeneratedGpgKey {
    let home = std::env::temp_dir().join(unique_name(prefix));
    fs::create_dir_all(&home).expect("create GNUPGHOME");
    let user_id = format!("{} <{}>", unique_name(prefix), email);

    run_gpg(
        &home,
        &[
            "--batch",
            "--pinentry-mode",
            "loopback",
            "--passphrase",
            "",
            "--quick-generate-key",
            &user_id,
            "rsa2048",
            "sign",
            "0",
        ],
    );

    let armored_public_key = String::from_utf8(run_gpg(&home, &["--armor", "--export", &user_id]))
        .expect("utf8 armored key");

    GeneratedGpgKey {
        home,
        user_id,
        armored_public_key,
    }
}

fn sign_gpg_token(home: &PathBuf, user_id: &str, token: &str) -> String {
    let token_file = home.join("verification-token.txt");
    fs::write(&token_file, token).expect("write verification token");

    run_gpg(
        home,
        &[
            "--batch",
            "--yes",
            "--pinentry-mode",
            "loopback",
            "--passphrase",
            "",
            "--armor",
            "--local-user",
            user_id,
            "--output",
            home.join("verification-token.asc")
                .to_str()
                .expect("ascii signature path"),
            "--detach-sign",
            token_file.to_str().expect("ascii token path"),
        ],
    );

    fs::read_to_string(home.join("verification-token.asc")).expect("read signature")
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_read_flows() {
    let client = live_client();
    let env = load_live_env();

    let (me, me_response) = client.users().get_my_info().await.expect("get my info");
    assert_success_status(me_response.status);
    assert_eq!(me.user_name, env.user_name);

    let (by_name, by_name_response) = client.users().get(&env.user_name).await.expect("get user");
    assert_success_status(by_name_response.status);
    assert_eq!(by_name.user_name, env.user_name);

    let (by_id, by_id_response) = client
        .users()
        .get_by_id(me.id)
        .await
        .expect("get user by id");
    assert_success_status(by_id_response.status);
    assert_eq!(by_id.id, me.id);

    let (search_results, search_response) = client
        .users()
        .search(SearchUsersOption {
            key_word: env.user_name.clone(),
            ..Default::default()
        })
        .await
        .expect("search users");
    assert_success_status(search_response.status);
    assert!(
        search_results
            .iter()
            .any(|user| user.user_name == env.user_name)
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_key_and_email_reads() {
    let client = live_client();
    let env = load_live_env();

    let (_emails, email_response) = client
        .users()
        .list_emails(ListEmailsOptions::default())
        .await
        .expect("list emails");
    assert_success_status(email_response.status);

    let (my_keys, my_keys_response) = client
        .users()
        .list_my_public_keys(ListPublicKeysOptions::default())
        .await
        .expect("list my public keys");
    assert_success_status(my_keys_response.status);
    assert!(!my_keys.is_empty());

    let (user_keys, user_keys_response) = client
        .users()
        .list_public_keys(&env.user_name, ListPublicKeysOptions::default())
        .await
        .expect("list public keys by username");
    assert_success_status(user_keys_response.status);
    assert!(user_keys.iter().any(|key| key.key == my_keys[0].key));
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_email_lifecycle() {
    let client = live_client();
    let email_to_manage = format!("{}@example.com", unique_name("live-email"));

    let (added, add_response) = client
        .users()
        .add_email(CreateEmailOption {
            emails: vec![email_to_manage.clone()],
        })
        .await
        .expect("add email");
    assert_success_status(add_response.status);
    assert!(
        added.iter().any(|entry| entry.email == email_to_manage),
        "API should return the newly added email"
    );

    let (after_add, list_response) = client
        .users()
        .list_emails(ListEmailsOptions::default())
        .await
        .expect("list emails after add");
    assert_success_status(list_response.status);
    assert!(
        after_add.iter().any(|entry| entry.email == email_to_manage),
        "list should contain the new email"
    );

    let delete_response = client
        .users()
        .delete_email(DeleteEmailOption {
            emails: vec![email_to_manage.clone()],
        })
        .await
        .expect("delete email");
    assert_eq!(delete_response.status, 204);

    let (after_delete, delete_list_response) = client
        .users()
        .list_emails(ListEmailsOptions::default())
        .await
        .expect("list emails after delete");
    assert_success_status(delete_list_response.status);
    assert!(
        after_delete
            .iter()
            .all(|entry| entry.email != email_to_manage),
        "email should no longer appear after deletion"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_avatar_lifecycle() {
    let client = live_client();

    let update_response = client
        .users()
        .update_avatar(UpdateUserAvatarOption {
            image: AVATAR_BASE64.to_string(),
        })
        .await
        .expect("update avatar");
    assert_eq!(update_response.status, 204);

    let delete_response = client.users().delete_avatar().await.expect("delete avatar");
    assert_eq!(delete_response.status, 204);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_gpg_key_lifecycle() {
    let client = live_client();
    let env = load_live_env();
    let (emails, email_response) = client
        .users()
        .list_emails(ListEmailsOptions::default())
        .await
        .expect("list emails for gpg");
    assert_success_status(email_response.status);
    let primary_email = emails
        .iter()
        .find(|entry| entry.verified)
        .or_else(|| emails.first())
        .expect("at least one email for gpg")
        .email
        .clone();

    let generated = generate_gpg_key("live-gpg", &primary_email);

    let (created_key, create_response) = client
        .users()
        .create_gpg_key(CreateGPGKeyOption {
            armored_key: generated.armored_public_key.clone(),
            signature: None,
        })
        .await
        .expect("create gpg key");
    assert_success_status(create_response.status);

    let key_id = created_key.id;

    let (my_keys, my_keys_response) = client
        .users()
        .list_my_gpg_keys(ListGPGKeysOptions::default())
        .await
        .expect("list my gpg keys");
    assert_success_status(my_keys_response.status);
    assert!(my_keys.iter().any(|key| key.id == key_id));

    let (user_keys, user_keys_response) = client
        .users()
        .list_gpg_keys(&env.user_name, ListGPGKeysOptions::default())
        .await
        .expect("list user gpg keys");
    assert_success_status(user_keys_response.status);
    assert!(user_keys.iter().any(|key| key.id == key_id));

    let (loaded_key, get_response) = client
        .users()
        .get_gpg_key(key_id)
        .await
        .expect("get gpg key");
    assert_success_status(get_response.status);
    assert_eq!(loaded_key.id, key_id);

    let (verification_token, token_response) = client
        .users()
        .get_gpg_key_verification_token()
        .await
        .expect("get gpg key verification token");
    assert_success_status(token_response.status);
    assert!(!verification_token.trim().is_empty());

    let armored_signature = sign_gpg_token(
        &generated.home,
        &generated.user_id,
        verification_token.trim(),
    );
    let (verified_key, verify_response) = client
        .users()
        .verify_gpg_key(VerifyGPGKeyOption {
            key_id: loaded_key.key_id.clone(),
            signature: armored_signature,
        })
        .await
        .expect("verify gpg key");
    assert_success_status(verify_response.status);
    assert_eq!(verified_key.id, key_id);

    let delete_response = client
        .users()
        .delete_gpg_key(key_id)
        .await
        .expect("delete gpg key");
    assert_eq!(delete_response.status, 204);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_access_token_lifecycle() {
    let env = load_live_env();
    let client = build_basic_auth_client(env);
    let token_name = unique_name("live-access-token");

    let (created, create_response) = client
        .users()
        .create_access_token(
            &env.user_name,
            CreateAccessTokenOption {
                name: token_name.clone(),
                scopes: vec![AccessTokenScope::All],
            },
        )
        .await
        .expect("create access token");
    assert_success_status(create_response.status);
    assert_eq!(created.name, token_name);

    let (tokens, list_response) = client
        .users()
        .list_access_tokens(&env.user_name, ListAccessTokensOptions::default())
        .await
        .expect("list access tokens");
    assert_success_status(list_response.status);
    assert!(tokens.iter().any(|token| token.name == token_name));

    let delete_response = client
        .users()
        .delete_access_token(&env.user_name, &token_name)
        .await
        .expect("delete access token");
    assert_success_status(delete_response.status);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_settings_read() {
    let client = live_client();
    let (_settings, response) = client.users().get_settings().await.expect("get settings");
    assert_success_status(response.status);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_user_activity_and_settings_flow() {
    let env = load_live_env();
    let token_client = live_client();
    let basic_client = build_basic_auth_client(env);

    let mut cleanup = CleanupRegistry::new();
    let _repo_fixture = create_repo_fixture(&token_client, &mut cleanup, "live-user-activity")
        .await
        .expect("create repo fixture for activity");

    let (heatmap, heatmap_response) = token_client
        .users()
        .get_user_heatmap(&load_live_env().user_name)
        .await
        .expect("get user heatmap");
    assert_success_status(heatmap_response.status);
    let _ = heatmap;

    let (activity_feeds, activity_response) = token_client
        .users()
        .list_user_activity_feeds(
            &load_live_env().user_name,
            ListUserActivityFeedsOptions::default(),
        )
        .await
        .expect("list user activity feeds");
    assert_success_status(activity_response.status);
    let _ = activity_feeds;

    let (original_settings, get_response) = basic_client
        .users()
        .get_settings()
        .await
        .expect("get settings before update");
    assert_success_status(get_response.status);

    let toggled_hide_activity = !original_settings.hide_activity;
    let (updated_settings, update_response) = basic_client
        .users()
        .update_settings(UserSettingsOptions {
            hide_activity: Some(toggled_hide_activity),
            ..Default::default()
        })
        .await
        .expect("update user settings");
    assert_success_status(update_response.status);
    assert_eq!(updated_settings.hide_activity, toggled_hide_activity);

    let (restored_settings, restore_response) = basic_client
        .users()
        .update_settings(UserSettingsOptions {
            hide_activity: Some(original_settings.hide_activity),
            ..Default::default()
        })
        .await
        .expect("restore user settings");
    assert_success_status(restore_response.status);
    assert_eq!(
        restored_settings.hide_activity,
        original_settings.hide_activity
    );

    cleanup.run_all().await;
}
