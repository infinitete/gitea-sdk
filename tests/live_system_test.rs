// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use gitea_sdk_rs::options::miscellaneous::{MarkdownOption, MarkupOption};
use gitea_sdk_rs::options::user::{
    CreateGPGKeyOption, CreateKeyOption, ListEmailsOptions, ListPublicKeysOptions,
};

use live::{
    CleanupRegistry, create_repo_fixture, live_client, load_live_env,
    load_unused_or_generate_public_key, unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

struct GeneratedGpgKey {
    home: PathBuf,
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
        armored_public_key,
    }
}

#[tokio::test]
#[ignore = "requires the configured live Gitea instance"]
async fn live_misc_render_markdown() {
    let client = live_client();

    let opt = MarkdownOption {
        text: "# Hello".to_string(),
        mode: None,
        context: None,
        wiki: false,
    };
    let (rendered, response) = client
        .miscellaneous()
        .render_markdown(opt)
        .await
        .expect("render markdown");
    assert_success_status(response.status);
    assert!(rendered.contains("<h1"));

    let markup_opt = MarkupOption {
        text: "Hello *world*".to_string(),
        mode: None,
        context: None,
        file_path: None,
        wiki: false,
    };
    let (markup, markup_response) = client
        .miscellaneous()
        .render_markup(markup_opt)
        .await
        .expect("render markup");
    assert_success_status(markup_response.status);
    assert!(markup.contains("<em>world</em>"));

    let (raw_markup, raw_response) = client
        .miscellaneous()
        .render_markdown_raw("**raw markdown**")
        .await
        .expect("render raw markdown");
    assert_success_status(raw_response.status);
    assert!(raw_markup.contains("<strong>raw markdown</strong>"));
}

#[tokio::test]
#[ignore = "requires the configured live Gitea instance"]
async fn live_misc_gitignore_node_info() {
    let client = live_client();

    let (version, version_response) = client
        .miscellaneous()
        .get_version()
        .await
        .expect("get version");
    assert_success_status(version_response.status);
    assert!(!version.is_empty());

    match client.miscellaneous().get_node_info().await {
        Ok((node_info, node_response)) => {
            assert_success_status(node_response.status);
            assert!(!node_info.software.name.is_empty());
        }
        Err(err) => match err {
            gitea_sdk_rs::Error::UnknownApi { status: 404, .. } => return,
            other => panic!("get node info: {other}"),
        },
    }

    let (templates, template_response) = client
        .miscellaneous()
        .list_gitignore_templates()
        .await
        .expect("list gitignore templates");
    assert_success_status(template_response.status);
    assert!(
        templates
            .iter()
            .any(|name| name.to_lowercase().contains("rust"))
    );

    let gitignore_name = templates
        .iter()
        .find(|name| name.to_lowercase().contains("rust"))
        .expect("rust gitignore template should exist");
    let (gitignore, gitignore_response) = client
        .miscellaneous()
        .get_gitignore_template(gitignore_name)
        .await
        .expect("get gitignore template");
    assert_success_status(gitignore_response.status);
    assert!(
        !gitignore.source.is_empty(),
        "gitignore template content should not be empty"
    );

    let (label_templates, label_templates_response) = client
        .miscellaneous()
        .list_label_templates()
        .await
        .expect("list label templates");
    assert_success_status(label_templates_response.status);
    let label_template_name = label_templates
        .first()
        .expect("at least one label template should exist");
    let (labels, labels_response) = client
        .miscellaneous()
        .get_label_template(label_template_name)
        .await
        .expect("get label template");
    assert_success_status(labels_response.status);
    assert!(!labels.is_empty(), "label template should contain labels");

    let (license_templates, license_templates_response) = client
        .miscellaneous()
        .list_license_templates()
        .await
        .expect("list license templates");
    assert_success_status(license_templates_response.status);
    let license = license_templates
        .first()
        .expect("at least one license template should exist");
    let (license_template, license_response) = client
        .miscellaneous()
        .get_license_template(&license.key)
        .await
        .expect("get license template");
    assert_success_status(license_response.status);
    assert!(
        !license_template.body.is_empty(),
        "license template body should not be empty"
    );
}

#[tokio::test]
#[ignore = "requires the configured live Gitea instance"]
async fn live_misc_signing_keys() {
    let client = live_client();

    match client.miscellaneous().get_signing_key_gpg().await {
        Ok((key, response)) => {
            assert_success_status(response.status);
            assert!(
                !key.trim().is_empty(),
                "GPG signing key should not be empty"
            );
        }
        Err(gitea_sdk_rs::Error::Api {
            status, message, ..
        }) if status == 404 && message.contains("no signing key") => {
            println!("[signing key gpg] live instance returned 404 no signing key");
        }
        Err(err) => panic!("get signing key GPG: {err}"),
    }

    match client.miscellaneous().get_signing_key_ssh().await {
        Ok((key, response)) => {
            assert_success_status(response.status);
            assert!(
                key.trim().starts_with("ssh-"),
                "SSH signing key should start with ssh-"
            );
        }
        Err(gitea_sdk_rs::Error::Api {
            status, message, ..
        }) if status == 404 && message.contains("no signing key") => {
            println!("[signing key ssh] live instance returned 404 no signing key");
        }
        Err(err) => panic!("get signing key SSH: {err}"),
    }
}

#[tokio::test]
#[ignore = "requires the configured live Gitea instance"]
async fn live_activitypub_repository() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-repo-activitypub")
        .await
        .expect("create repo fixture");

    let env = load_live_env();
    match client
        .activitypub()
        .get_repository(&env.user_name, &fixture.repository.name)
        .await
    {
        Ok((repo_actor, response)) => {
            assert_success_status(response.status);
            assert!(repo_actor.get("type").and_then(|v| v.as_str()).is_some());
        }
        Err(err) => match err {
            gitea_sdk_rs::Error::UnknownApi { status: 404, .. } => {
                cleanup.run_all().await;
                return;
            }
            other => panic!("get activitypub repository actor: {other}"),
        },
    }
    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires the configured live Gitea instance"]
async fn live_global_settings_reads() {
    let client = live_client();

    let (ui_settings, ui_response) = client
        .settings()
        .get_ui_settings()
        .await
        .expect("get ui settings");
    assert_success_status(ui_response.status);
    assert!(!ui_settings.default_theme.is_empty());

    let (_repo_settings, repo_response) = client
        .settings()
        .get_repo_settings()
        .await
        .expect("get repo settings");
    assert_success_status(repo_response.status);

    let (_api_settings, api_response) = client
        .settings()
        .get_api_settings()
        .await
        .expect("get api settings");
    assert_success_status(api_response.status);

    let (_attachment_settings, attachment_response) = client
        .settings()
        .get_attachment_settings()
        .await
        .expect("get attachment settings");
    assert_success_status(attachment_response.status);
}

#[tokio::test]
#[ignore = "requires the configured live Gitea instance"]
async fn live_signing_key_reads() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();

    let (existing_keys, list_keys_response) = client
        .users()
        .list_my_public_keys(ListPublicKeysOptions::default())
        .await
        .expect("list public keys");
    assert_success_status(list_keys_response.status);
    let public_key = load_unused_or_generate_public_key(
        env,
        &existing_keys
            .iter()
            .map(|public_key| public_key.key.clone())
            .collect::<Vec<_>>(),
        "live-signing-key",
    )
    .expect("prepare ssh public key");
    let (created_public_key, create_public_key_response) = client
        .users()
        .create_public_key(CreateKeyOption {
            title: unique_name("live-signing-key"),
            key: public_key,
            read_only: false,
        })
        .await
        .expect("create public key");
    assert_success_status(create_public_key_response.status);
    let public_key_id = created_public_key.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .users()
            .delete_public_key(public_key_id)
            .await;
    });

    let (emails, email_response) = client
        .users()
        .list_emails(ListEmailsOptions::default())
        .await
        .expect("list emails for signing key gpg seed");
    assert_success_status(email_response.status);
    let primary_email = emails
        .iter()
        .find(|entry| entry.verified)
        .or_else(|| emails.first())
        .expect("at least one email for gpg")
        .email
        .clone();
    let generated_gpg = generate_gpg_key("live-signing-key-gpg", &primary_email);
    let (created_gpg_key, create_gpg_response) = client
        .users()
        .create_gpg_key(CreateGPGKeyOption {
            armored_key: generated_gpg.armored_public_key,
            signature: None,
        })
        .await
        .expect("create gpg key");
    assert_success_status(create_gpg_response.status);
    let gpg_key_id = created_gpg_key.id;
    let cleanup_client = client.clone();
    cleanup.register(async move {
        let _ = cleanup_client.users().delete_gpg_key(gpg_key_id).await;
    });
    let _ = fs::remove_dir_all(&generated_gpg.home);

    match client.miscellaneous().get_signing_key_gpg().await {
        Ok((gpg_key, gpg_response)) => {
            assert_success_status(gpg_response.status);
            assert!(
                !gpg_key.trim().is_empty(),
                "gpg signing key response should not be empty"
            );
        }
        Err(gitea_sdk_rs::Error::Api {
            status: 404,
            message,
            ..
        }) => {
            println!(
                "[misc capability] live signing-key.gpg endpoint unavailable with 404 ({message}) even after seeding a user GPG key; keeping get_signing_key_gpg blocked on this instance"
            );
        }
        Err(other) => panic!("get signing key gpg: {other}"),
    }

    match client.miscellaneous().get_signing_key_ssh().await {
        Ok((ssh_key, ssh_response)) => {
            assert_success_status(ssh_response.status);
            assert!(
                !ssh_key.trim().is_empty(),
                "ssh signing key response should not be empty"
            );
        }
        Err(gitea_sdk_rs::Error::Api {
            status: 404,
            message,
            ..
        }) => {
            println!(
                "[misc capability] live signing-key.pub endpoint unavailable with 404 ({message}) even after seeding a user SSH public key; keeping get_signing_key_ssh blocked on this instance"
            );
        }
        Err(other) => panic!("get signing key ssh: {other}"),
    }

    cleanup.run_all().await;
}
