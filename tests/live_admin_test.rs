// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use std::collections::HashMap;

use gitea_sdk::Error;
use gitea_sdk::options::admin::{
    AdminListOrgsOptions, AdminListUsersOptions, CreateHookOption, CreateUserOption,
    EditHookOption, EditUserOption, ListAdminEmailsOptions, ListAdminHooksOptions,
    ListCronTasksOptions, ListUnadoptedReposOptions, RenameUserOption, SearchAdminEmailsOptions,
    UserBadgeOption,
};
use gitea_sdk::options::org::CreateOrgOption;
use gitea_sdk::options::repo::CreateRepoOption;
use gitea_sdk::options::user::CreateKeyOption;
use gitea_sdk::types::enums::{HookType, TrustModel, VisibleType};

use live::{generate_fresh_public_key, live_client, load_live_env, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

fn live_hook_config(prefix: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert(
        "url".to_string(),
        format!("https://example.com/hooks/{}", unique_name(prefix)),
    );
    config.insert("content_type".to_string(), "json".to_string());
    config
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_admin_read_probe() {
    let client = live_client();

    let probes = vec![
        (
            "list_users",
            client
                .admin()
                .list_users(AdminListUsersOptions::default())
                .await
                .map(|(_, resp)| resp.status),
        ),
        (
            "list_orgs",
            client
                .admin()
                .list_orgs(AdminListOrgsOptions::default())
                .await
                .map(|(_, resp)| resp.status),
        ),
        (
            "list_cron_tasks",
            client
                .admin()
                .list_cron_tasks(ListCronTasksOptions::default())
                .await
                .map(|(_, resp)| resp.status),
        ),
        (
            "list_hooks",
            client
                .admin()
                .list_hooks(ListAdminHooksOptions::default())
                .await
                .map(|(_, resp)| resp.status),
        ),
        (
            "list_emails",
            client
                .admin()
                .list_emails(ListAdminEmailsOptions::default())
                .await
                .map(|(_, resp)| resp.status),
        ),
    ];

    for (name, result) in probes {
        match result {
            Ok(status) => {
                assert_success_status(status);
                println!("[admin capability] {name} succeeded with {status}");
            }
            Err(Error::Api {
                status, message, ..
            }) => {
                println!("[admin capability] {name} returned {status}: {message}");
            }
            Err(Error::UnknownApi { status, body }) => {
                println!("[admin capability] {name} returned {status}: {body}");
            }
            Err(other) => panic!("{name}: {other}"),
        }
    }
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_admin_user_management_flow() {
    let client = live_client();
    let initial_user = unique_name("live-admin-user");
    let renamed_user = unique_name("live-admin-renamed");
    let email = format!("{initial_user}@example.com");
    let edited_email = format!("{renamed_user}@example.com");

    let cleanup_client = client.clone();
    let cleanup_user = |user: String| async move {
        let _ = cleanup_client.admin().delete_user(&user).await;
    };

    let (created_user, create_resp) = client
        .admin()
        .create_user(CreateUserOption {
            username: initial_user.clone(),
            full_name: Some("Live Admin User".to_string()),
            email: email.clone(),
            password: "tester123456".to_string(),
            must_change_password: Some(false),
            send_notify: false,
            visibility: Some(VisibleType::Public),
            ..Default::default()
        })
        .await
        .expect("create admin user");
    assert_success_status(create_resp.status);
    assert_eq!(created_user.user_name, initial_user);

    let (users, list_users_resp) = client
        .admin()
        .list_users(AdminListUsersOptions::default())
        .await
        .expect("list users");
    assert_success_status(list_users_resp.status);
    assert!(users.iter().any(|user| user.user_name == initial_user));

    let search_resp = client
        .admin()
        .search_emails(SearchAdminEmailsOptions {
            query: email.clone(),
            ..Default::default()
        })
        .await;
    match search_resp {
        Ok((emails, resp)) => {
            assert_success_status(resp.status);
            assert!(emails.iter().any(|entry| entry.email == email));
        }
        Err(err) => {
            cleanup_user(initial_user.clone()).await;
            panic!("search emails: {err}");
        }
    }

    if let Err(err) = client
        .admin()
        .edit_user(
            &initial_user,
            EditUserOption {
                login_name: Some(initial_user.clone()),
                full_name: Some("Live Admin User Edited".to_string()),
                email: Some(edited_email.clone()),
                location: Some("Shanghai".to_string()),
                website: Some("https://example.com".to_string()),
                ..Default::default()
            },
        )
        .await
    {
        cleanup_user(initial_user.clone()).await;
        panic!("edit user: {err}");
    }

    let rename_resp = client
        .admin()
        .rename_user(
            &initial_user,
            RenameUserOption {
                new_username: renamed_user.clone(),
            },
        )
        .await;
    if let Err(err) = rename_resp {
        cleanup_user(initial_user.clone()).await;
        panic!("rename user: {err}");
    }

    let public_key = generate_fresh_public_key("live-admin-key").expect("generate public key");
    let (created_key, key_resp) = client
        .admin()
        .create_user_public_key(
            &renamed_user,
            CreateKeyOption {
                title: unique_name("live-admin-key"),
                key: public_key,
                read_only: false,
            },
        )
        .await
        .expect("create user public key");
    assert_success_status(key_resp.status);

    let delete_key_resp = client
        .admin()
        .delete_user_public_key(&renamed_user, created_key.id)
        .await
        .expect("delete user public key");
    assert_success_status(delete_key_resp.status);

    let (created_org, create_org_resp) = client
        .admin()
        .create_org_for_user(
            &renamed_user,
            CreateOrgOption {
                name: unique_name("live-admin-org"),
                full_name: Some("Live Admin Org".to_string()),
                email: Some(edited_email.clone()),
                description: Some("live admin org".to_string()),
                website: None,
                location: None,
                visibility: None,
                repo_admin_change_team_access: None,
            },
        )
        .await
        .expect("create org for user");
    assert_success_status(create_org_resp.status);

    let (created_repo, create_repo_resp) = client
        .admin()
        .create_repo_for_user(
            &renamed_user,
            CreateRepoOption {
                name: unique_name("live-admin-repo"),
                description: "live admin repo".to_string(),
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
            },
        )
        .await
        .expect("create repo for user");
    assert_success_status(create_repo_resp.status);
    assert_eq!(
        created_repo.owner.as_ref().expect("repo owner").user_name,
        renamed_user
    );

    let (orgs, list_orgs_resp) = client
        .admin()
        .list_orgs(AdminListOrgsOptions::default())
        .await
        .expect("list orgs");
    assert_success_status(list_orgs_resp.status);
    assert!(
        orgs.iter()
            .any(|org| org.user_name == created_org.user_name)
    );

    let delete_repo_resp = client
        .repos()
        .delete_repo(&renamed_user, &created_repo.name)
        .await
        .expect("delete created repo");
    assert_success_status(delete_repo_resp.status);

    let delete_org_resp = client
        .orgs()
        .delete_org(&created_org.user_name)
        .await
        .expect("delete created org");
    assert_success_status(delete_org_resp.status);

    let delete_resp = client
        .admin()
        .delete_user(&renamed_user)
        .await
        .expect("delete user");
    assert_success_status(delete_resp.status);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_admin_hook_lifecycle() {
    let client = live_client();

    let (hook, create_resp) = client
        .admin()
        .create_hook(CreateHookOption {
            hook_type: HookType::Gitea,
            config: live_hook_config("admin-hook"),
            events: vec!["push".to_string()],
            branch_filter: String::new(),
            active: true,
            authorization_header: String::new(),
        })
        .await
        .expect("create admin hook");
    assert_success_status(create_resp.status);

    let hook_id = hook.id;

    let (loaded, get_resp) = client
        .admin()
        .get_hook(hook_id)
        .await
        .expect("get admin hook");
    assert_success_status(get_resp.status);
    assert_eq!(loaded.id, hook_id);

    let (_hooks, list_resp) = client
        .admin()
        .list_hooks(ListAdminHooksOptions::default())
        .await
        .expect("list admin hooks");
    assert_success_status(list_resp.status);

    let (edited, edit_resp) = client
        .admin()
        .edit_hook(
            hook_id,
            EditHookOption {
                active: Some(false),
                ..Default::default()
            },
        )
        .await
        .expect("edit admin hook");
    assert_success_status(edit_resp.status);
    assert_eq!(edited.id, hook_id);
    assert!(!edited.active);

    let delete_resp = client
        .admin()
        .delete_hook(hook_id)
        .await
        .expect("delete admin hook");
    assert_eq!(delete_resp.status, 204);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_admin_badge_probe() {
    let client = live_client();
    let env = load_live_env();

    let (_badges, list_resp) = client
        .admin()
        .list_user_badges(&env.user_name)
        .await
        .expect("list user badges");
    assert_success_status(list_resp.status);
    let (main_badges, main_badges_resp) = client
        .admin()
        .list_user_badges(&env.user_name)
        .await
        .expect("list main user badges");
    assert_success_status(main_badges_resp.status);
    println!(
        "[admin capability] main user badges: {:?}",
        main_badges
            .iter()
            .map(|badge| badge.slug.clone())
            .collect::<Vec<_>>()
    );

    let badge_user = unique_name("live-badge-user");
    let badge_email = format!("{badge_user}@example.com");
    let (created_user, create_resp) = client
        .admin()
        .create_user(CreateUserOption {
            username: badge_user.clone(),
            full_name: Some("Live Badge User".to_string()),
            email: badge_email,
            password: "tester123456".to_string(),
            must_change_password: Some(false),
            send_notify: false,
            visibility: Some(VisibleType::Public),
            ..Default::default()
        })
        .await
        .expect("create badge probe user");
    assert_success_status(create_resp.status);

    for slug in ["contributor", "reviewer"] {
        match client
            .admin()
            .add_user_badges(
                &created_user.user_name,
                UserBadgeOption {
                    badge_slugs: vec![slug.to_string()],
                },
            )
            .await
        {
            Ok(add_resp) => {
                assert_success_status(add_resp.status);
                println!("[admin capability] add_user_badges accepted slug {slug}");
            }
            Err(Error::Api {
                status, message, ..
            }) => {
                println!("[admin capability] add_user_badges({slug}) returned {status}: {message}");
            }
            Err(Error::UnknownApi { status, body }) => {
                println!("[admin capability] add_user_badges({slug}) returned {status}: {body}");
            }
            Err(other) => panic!("add user badges with {slug}: {other}"),
        }

        match client
            .admin()
            .delete_user_badges(
                &created_user.user_name,
                UserBadgeOption {
                    badge_slugs: vec![slug.to_string()],
                },
            )
            .await
        {
            Ok(delete_resp) => {
                assert_eq!(delete_resp.status, 204);
                println!("[admin capability] delete_user_badges accepted slug {slug}");
            }
            Err(Error::Api {
                status, message, ..
            }) => {
                println!(
                    "[admin capability] delete_user_badges({slug}) returned {status}: {message}"
                );
            }
            Err(Error::UnknownApi { status, body }) => {
                println!("[admin capability] delete_user_badges({slug}) returned {status}: {body}");
            }
            Err(other) => panic!("delete user badges with {slug}: {other}"),
        }
    }

    let delete_user_resp = client
        .admin()
        .delete_user(&created_user.user_name)
        .await
        .expect("delete badge probe user");
    assert_success_status(delete_user_resp.status);
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_admin_cron_and_unadopted_probe() {
    let client = live_client();

    let (tasks, list_cron_resp) = client
        .admin()
        .list_cron_tasks(ListCronTasksOptions::default())
        .await
        .expect("list cron tasks");
    assert_success_status(list_cron_resp.status);
    assert!(
        !tasks.is_empty(),
        "configured instance should expose at least one cron task"
    );

    match client.admin().run_cron_task(&tasks[0].name).await {
        Ok(run_resp) => assert_eq!(run_resp.status, 204),
        Err(Error::Api {
            status, message, ..
        }) => {
            println!(
                "[admin capability] run_cron_task({}) returned {status}: {message}",
                tasks[0].name
            );
        }
        Err(Error::UnknownApi { status, body }) => {
            println!(
                "[admin capability] run_cron_task({}) returned {status}: {body}",
                tasks[0].name
            );
        }
        Err(other) => panic!("run cron task: {other}"),
    }

    let (unadopted, list_unadopted_resp) = client
        .admin()
        .list_unadopted_repos(ListUnadoptedReposOptions::default())
        .await
        .expect("list unadopted repos");
    assert_success_status(list_unadopted_resp.status);
    if unadopted.is_empty() {
        println!(
            "[admin capability] list_unadopted_repos returned an empty list; adopt/delete require host-level seed repos"
        );
    } else {
        println!(
            "[admin capability] list_unadopted_repos found {} host-level repos; adopt/delete left blocked to avoid mutating unknown server state",
            unadopted.len()
        );
    }
}
