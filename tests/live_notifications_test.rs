// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::issue::CreateIssueOption;
use gitea_sdk::options::notification::{ListNotificationOptions, MarkNotificationOptions};
use gitea_sdk::options::repo::AddCollaboratorOption;
use gitea_sdk::types::enums::AccessMode;
use tokio::time::{Duration, sleep};

use live::{
    CleanupRegistry, create_repo_fixture, live_client, load_live_env, next_user_client, unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_notifications_issue_thread_flow() {
    let client = live_client();
    let env = load_live_env();
    let second_user = env
        .next_user_name
        .as_ref()
        .expect("missing GITEA_NEXT_USER_NAME in .env")
        .clone();
    let second_client = next_user_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-notify-repo")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let marker = unique_name("live-notify-marker");

    let add_resp = client
        .repos()
        .add_collaborator(
            &owner,
            &repo,
            &second_user,
            AddCollaboratorOption {
                permission: Some(AccessMode::Write),
            },
        )
        .await
        .expect("add second user as collaborator");
    assert_success_status(add_resp.status);

    let (_, check_response) = client
        .notifications()
        .check_notifications()
        .await
        .expect("check notifications");
    assert_success_status(check_response.status);

    let (all_threads, list_response) = client
        .notifications()
        .list_notifications(ListNotificationOptions::default())
        .await
        .expect("list notifications");
    assert_success_status(list_response.status);

    let (repo_threads, repo_list_response) = client
        .notifications()
        .list_repo_notifications(&owner, &repo, ListNotificationOptions::default())
        .await
        .expect("list repo notifications");
    assert_success_status(repo_list_response.status);

    let (marked_repo_threads, repo_mark_response) = client
        .notifications()
        .read_repo_notifications(&owner, &repo, MarkNotificationOptions::default())
        .await
        .expect("mark repo notifications as read");
    assert_success_status(repo_mark_response.status);
    assert_eq!(marked_repo_threads.len(), repo_threads.len());

    let (marked_threads, mark_response) = client
        .notifications()
        .read_notifications(MarkNotificationOptions::default())
        .await
        .expect("mark notifications as read");
    assert_success_status(mark_response.status);
    assert_eq!(marked_threads.len(), all_threads.len());

    let issue_title = format!("{marker} issue");
    let issue_body = format!("@{} seeded notification {}", env.user_name, marker);
    let (_created_issue, create_issue_resp) = second_client
        .issues()
        .create_issue(
            &owner,
            &repo,
            CreateIssueOption {
                title: issue_title.clone(),
                body: issue_body,
                r#ref: String::new(),
                assignees: Vec::new(),
                deadline: None,
                milestone: 0,
                labels: Vec::new(),
                closed: false,
            },
        )
        .await
        .expect("create notification-seeding issue");
    assert_success_status(create_issue_resp.status);

    let seeded_thread = {
        let mut found = None;
        for _ in 0..10 {
            let (threads, seeded_list_resp) = client
                .notifications()
                .list_notifications(ListNotificationOptions::default())
                .await
                .expect("list notifications after seeding");
            assert_success_status(seeded_list_resp.status);
            found = threads.into_iter().find(|thread| {
                thread
                    .subject
                    .as_ref()
                    .map(|subject| subject.title.contains(&marker))
                    .unwrap_or(false)
            });
            if found.is_some() {
                break;
            }
            sleep(Duration::from_millis(500)).await;
        }
        found.expect("seeded notification thread should appear")
    };

    let thread_id = seeded_thread.id;
    let (loaded, get_response) = client
        .notifications()
        .get_notification(thread_id)
        .await
        .expect("get notification");
    assert_success_status(get_response.status);
    assert_eq!(loaded.id, thread_id);
    assert!(
        loaded
            .subject
            .as_ref()
            .map(|subject| subject.title.contains(&marker))
            .unwrap_or(false),
        "loaded notification should match the seeded thread"
    );

    let (read_thread, read_response) = client
        .notifications()
        .read_notification(thread_id)
        .await
        .expect("read single notification");
    assert_success_status(read_response.status);
    assert_eq!(read_thread.id, thread_id);
    assert!(
        !read_thread.unread,
        "read notification should be marked read"
    );

    let (repo_threads_after, repo_list_after_response) = client
        .notifications()
        .list_repo_notifications(&owner, &repo, ListNotificationOptions::default())
        .await
        .expect("list repo notifications after seeding");
    assert_success_status(repo_list_after_response.status);
    let _ = repo_threads_after;

    let (marked_repo_threads_after, repo_mark_after_response) = client
        .notifications()
        .read_repo_notifications(&owner, &repo, MarkNotificationOptions::default())
        .await
        .expect("mark seeded repo notifications as read");
    assert_success_status(repo_mark_after_response.status);
    let _ = marked_repo_threads_after;

    let (marked_threads_after, mark_after_response) = client
        .notifications()
        .read_notifications(MarkNotificationOptions::default())
        .await
        .expect("mark seeded notifications as read");
    assert_success_status(mark_after_response.status);
    let _ = marked_threads_after;

    let (_, check_after_response) = client
        .notifications()
        .check_notifications()
        .await
        .expect("check notifications after seeding");
    assert_success_status(check_after_response.status);

    cleanup.run_all().await;
}
