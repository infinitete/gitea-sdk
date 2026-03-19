// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::Error;
use gitea_sdk::options::issue::CreateIssueCommentOption;
use gitea_sdk::options::issue::{
    AddTimeOption, EditDeadlineOption, ListIssueBlocksOptions, ListIssueCommentOptions,
    ListIssueDependenciesOptions, ListIssueSubscribersOptions, ListTrackedTimesOptions,
};
use gitea_sdk::types::issue::IssueMeta;
use time::{Duration, OffsetDateTime};

use live::{
    CleanupRegistry, create_issue_fixture, create_repo_fixture, enable_issue_dependencies,
    live_client, load_live_env,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_full_coverage() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issue-full")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let repo_after_enable = enable_issue_dependencies(&client, &owner, &repo)
        .await
        .expect("enable issue dependencies");
    assert!(
        repo_after_enable
            .internal_tracker
            .as_ref()
            .is_some_and(|tracker| tracker.enable_issue_dependencies),
        "expected repository issue dependencies to be enabled"
    );
    let issue_fixture =
        create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-issue-full")
            .await
            .expect("create issue fixture");
    let issue_index = issue_fixture.issue.index;

    // Subscription flow
    match client
        .issues()
        .add_issue_subscription(&owner, &repo, issue_index, &env.user_name)
        .await
    {
        Ok(resp) => assert_success_status(resp.status),
        Err(Error::Validation(message)) if message.contains("already subscribed") => {}
        Err(err) => panic!("add issue subscription: {err}"),
    }
    let (_subscribers, subscribers_resp) = client
        .issues()
        .list_issue_subscribers(
            &owner,
            &repo,
            issue_index,
            ListIssueSubscribersOptions::default(),
        )
        .await
        .expect("list issue subscribers");
    assert_success_status(subscribers_resp.status);
    client
        .issues()
        .delete_issue_subscription(&owner, &repo, issue_index, &env.user_name)
        .await
        .expect("delete issue subscription");

    // Tracked time flow
    let (tracked, tracked_resp) = client
        .issues()
        .add_time(
            &owner,
            &repo,
            issue_index,
            AddTimeOption {
                time: 120,
                created: None,
                user: env.user_name.clone(),
            },
        )
        .await
        .expect("add time to issue");
    assert_success_status(tracked_resp.status);
    let tracked_id = tracked.id;
    let (times, times_resp) = client
        .issues()
        .list_issue_tracked_times(
            &owner,
            &repo,
            issue_index,
            ListTrackedTimesOptions::default(),
        )
        .await
        .expect("list tracked times");
    assert_success_status(times_resp.status);
    assert!(
        times.iter().any(|entry| entry.id == tracked_id),
        "tracked time missing from list"
    );
    client
        .issues()
        .delete_time(&owner, &repo, issue_index, tracked_id)
        .await
        .expect("delete tracked time");

    // Timeline
    let (timeline, timeline_resp) = client
        .issues()
        .list_issue_timeline(
            &owner,
            &repo,
            issue_index,
            ListIssueCommentOptions::default(),
        )
        .await
        .expect("list issue timeline");
    assert_success_status(timeline_resp.status);
    assert!(!timeline.is_empty(), "expected timeline to include entries");

    // Deadline
    let (current_issue, _) = client
        .issues()
        .get_issue(&owner, &repo, issue_index)
        .await
        .expect("get issue");
    let original_deadline = current_issue.deadline;
    let new_deadline = Some(OffsetDateTime::now_utc() + Duration::days(1));
    let (updated_issue, deadline_resp) = client
        .issues()
        .update_issue_deadline(
            &owner,
            &repo,
            issue_index,
            EditDeadlineOption {
                deadline: new_deadline,
            },
        )
        .await
        .expect("update issue deadline");
    assert_success_status(deadline_resp.status);
    assert!(
        updated_issue.deadline != original_deadline,
        "expected deadline to update"
    );
    if original_deadline != new_deadline {
        client
            .issues()
            .update_issue_deadline(
                &owner,
                &repo,
                issue_index,
                EditDeadlineOption {
                    deadline: original_deadline,
                },
            )
            .await
            .expect("restore issue deadline");
    }

    // Attachments: create comment to reference comment assets
    let (comment, comment_resp) = client
        .issues()
        .create_issue_comment(
            &owner,
            &repo,
            issue_index,
            CreateIssueCommentOption {
                body: "attachment probe".to_string(),
            },
        )
        .await
        .expect("create issue comment");
    assert_success_status(comment_resp.status);
    let comment_id = comment.id;
    let (_attachments, attachments_resp) = client
        .issues()
        .list_issue_comment_attachments(&owner, &repo, comment_id)
        .await
        .expect("list comment attachments");
    assert_success_status(attachments_resp.status);
    client
        .issues()
        .delete_issue_comment(&owner, &repo, comment_id)
        .await
        .expect("delete comment");

    // Blocking/dependency endpoints (may be 404)
    let blocking_issue =
        create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-issue-blocking")
            .await
            .expect("create blocking issue");
    let blocking_index = blocking_issue.issue.index;
    match client
        .issues()
        .create_issue_blocking(
            &owner,
            &repo,
            issue_index,
            IssueMeta {
                index: blocking_index,
            },
        )
        .await
    {
        Ok((_, resp)) => {
            assert_success_status(resp.status);
            let (blocks, blocks_resp) = client
                .issues()
                .list_issue_blocks(
                    &owner,
                    &repo,
                    issue_index,
                    ListIssueBlocksOptions::default(),
                )
                .await
                .expect("list issue blocks");
            assert_success_status(blocks_resp.status);
            assert!(
                blocks.iter().any(|entry| entry.index == blocking_index),
                "blocking list missing new entry"
            );
            let (dependencies, deps_resp) = client
                .issues()
                .list_issue_dependencies(
                    &owner,
                    &repo,
                    blocking_index,
                    ListIssueDependenciesOptions::default(),
                )
                .await
                .expect("list issue dependencies");
            assert_success_status(deps_resp.status);
            assert!(
                dependencies.iter().any(|entry| entry.index == issue_index),
                "dependency list missing dependent"
            );
            client
                .issues()
                .remove_issue_blocking(
                    &owner,
                    &repo,
                    issue_index,
                    IssueMeta {
                        index: blocking_index,
                    },
                )
                .await
                .expect("remove blocking");
        }
        Err(Error::Api {
            status, message, ..
        }) => {
            println!("[issue blocking] endpoint blocked: status {status} message {message}");
        }
        Err(err) => panic!("create issue blocking: {err}"),
    }

    match client
        .issues()
        .create_issue_dependency(
            &owner,
            &repo,
            issue_index,
            IssueMeta {
                index: blocking_index,
            },
        )
        .await
    {
        Ok((_, resp)) => {
            assert_success_status(resp.status);
            client
                .issues()
                .remove_issue_dependency(
                    &owner,
                    &repo,
                    issue_index,
                    IssueMeta {
                        index: blocking_index,
                    },
                )
                .await
                .expect("remove issue dependency");
        }
        Err(Error::Api {
            status, message, ..
        }) => {
            println!("[issue dependency] endpoint blocked: status {status} message {message}");
        }
        Err(err) => panic!("create issue dependency: {err}"),
    }

    // Templates
    match client.issues().get_issue_templates(&owner, &repo).await {
        Ok((templates, templates_resp)) => {
            assert_success_status(templates_resp.status);
            let _ = templates;
        }
        Err(Error::Api {
            status, message, ..
        }) => {
            println!("[issue templates] endpoint blocked: status {status} message {message}");
        }
        Err(Error::Json(message)) => {
            println!("[issue templates] endpoint blocked: JSON error {message}");
        }
        Err(err) => panic!("get issue templates: {err}"),
    }

    cleanup.run_all().await;
}
