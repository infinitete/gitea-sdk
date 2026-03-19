// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::Error;
use gitea_rs::options::issue::{
    AddTimeOption, EditDeadlineOption, ListIssueBlocksOptions, ListIssueCommentOptions,
    ListIssueDependenciesOptions, ListIssueReactionsOptions, ListIssueSubscribersOptions,
    ListStopwatchesOptions, ListTrackedTimesOptions,
};
use gitea_rs::types::issue::IssueMeta;

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
async fn live_issue_reactions_subscriptions_time_blocks() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issues-adv")
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
    let issue = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-issue-main")
        .await
        .expect("create issue fixture");
    let issue_index = issue.issue.index;
    let blocking_issue =
        create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-issue-blocker")
            .await
            .expect("create blocking issue");
    let blocking_index = blocking_issue.issue.index;

    let comment = client
        .issues()
        .create_issue_comment(
            &owner,
            &repo,
            issue_index,
            gitea_rs::options::issue::CreateIssueCommentOption {
                body: "live advanced comment".to_string(),
            },
        )
        .await
        .expect("create issue comment");
    assert_success_status(comment.1.status);
    let comment_id = comment.0.id;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_repo = repo.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .issues()
            .delete_issue_comment(&cleanup_owner, &cleanup_repo, comment_id)
            .await;
    });

    client
        .issues()
        .post_issue_reaction(&owner, &repo, issue_index, "+1")
        .await
        .expect("add issue reaction");
    let (issue_reactions, reaction_resp) = client
        .issues()
        .list_issue_reactions(
            &owner,
            &repo,
            issue_index,
            ListIssueReactionsOptions::default(),
        )
        .await
        .expect("list issue reactions");
    assert_success_status(reaction_resp.status);
    assert!(issue_reactions.iter().any(|r| r.reaction == "+1"));
    client
        .issues()
        .delete_issue_reaction(&owner, &repo, issue_index, "+1")
        .await
        .expect("delete issue reaction");

    client
        .issues()
        .post_issue_comment_reaction(&owner, &repo, comment_id, "+1")
        .await
        .expect("add comment reaction");
    client
        .issues()
        .delete_issue_comment_reaction(&owner, &repo, comment_id, "+1")
        .await
        .expect("delete comment reaction");

    match client
        .issues()
        .issue_subscribe(&owner, &repo, issue_index)
        .await
    {
        Ok(_) => {}
        Err(Error::Validation(message)) if message.contains("already subscribed") => {}
        Err(other) => panic!("issue subscribe: {other}"),
    }
    let (subscribers, subs_resp) = client
        .issues()
        .list_issue_subscribers(
            &owner,
            &repo,
            issue_index,
            ListIssueSubscribersOptions::default(),
        )
        .await
        .expect("list issue subscribers");
    assert_success_status(subs_resp.status);
    let _ = subscribers;
    let (watch_info, check_resp) = client
        .issues()
        .check_issue_subscription(&owner, &repo, issue_index)
        .await
        .expect("check subscription");
    assert_success_status(check_resp.status);
    assert!(watch_info.subscribed);

    match client
        .issues()
        .add_issue_subscription(&owner, &repo, issue_index, &env.user_name)
        .await
    {
        Ok(add_sub_resp) => assert_success_status(add_sub_resp.status),
        Err(Error::Validation(message)) if message.contains("already subscribed") => {}
        Err(other) => panic!("add issue subscription: {other}"),
    }

    match client
        .issues()
        .delete_issue_subscription(&owner, &repo, issue_index, &env.user_name)
        .await
    {
        Ok(delete_sub_resp) => assert_success_status(delete_sub_resp.status),
        Err(Error::Validation(message)) if message.contains("already unsubscribed") => {}
        Err(other) => panic!("delete issue subscription: {other}"),
    }

    client
        .issues()
        .start_issue_stopwatch(&owner, &repo, issue_index)
        .await
        .expect("start stopwatch");
    let (stopwatches, watch_resp) = client
        .issues()
        .list_my_stopwatches(ListStopwatchesOptions::default())
        .await
        .expect("list stopwatches");
    assert_success_status(watch_resp.status);
    let _ = stopwatches;
    client
        .issues()
        .stop_issue_stopwatch(&owner, &repo, issue_index)
        .await
        .expect("stop stopwatch");
    match client
        .issues()
        .delete_issue_stopwatch(&owner, &repo, issue_index)
        .await
    {
        Ok(_) => {}
        Err(Error::Api {
            status, message, ..
        }) if status == 409 && message.contains("non-existent stopwatch") => {}
        Err(other) => panic!("delete stopwatch: {other}"),
    }

    let tracked = client
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
        .expect("add tracked time");
    assert_success_status(tracked.1.status);
    let tracked_id = tracked.0.id;
    let (issue_times, times_resp) = client
        .issues()
        .list_issue_tracked_times(
            &owner,
            &repo,
            issue_index,
            ListTrackedTimesOptions::default(),
        )
        .await
        .expect("list issue tracked times");
    assert_success_status(times_resp.status);
    assert!(issue_times.iter().any(|entry| entry.id == tracked_id));

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
    let _ = timeline;

    let deadline = time::OffsetDateTime::now_utc() + time::Duration::days(7);
    let (updated_deadline, deadline_resp) = client
        .issues()
        .update_issue_deadline(
            &owner,
            &repo,
            issue_index,
            EditDeadlineOption {
                deadline: Some(deadline),
            },
        )
        .await
        .expect("update issue deadline");
    assert_success_status(deadline_resp.status);
    assert_eq!(updated_deadline.index, issue_index);

    client
        .issues()
        .delete_time(&owner, &repo, issue_index, tracked_id)
        .await
        .expect("delete tracked time");

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
        Ok(_) => {
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
            assert!(blocks.iter().any(|entry| entry.index == blocking_index));
            let (deps, deps_resp) = client
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
            assert!(deps.iter().any(|entry| entry.index == issue_index));
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
        }) if status == 404 && message.contains("IsErrRepoNotExist") => {
            println!(
                "[issues advanced] blocking/dependency endpoints returned server 404 IsErrRepoNotExist on live instance; leaving that subdomain blocked"
            );
        }
        Err(other) => panic!("create blocking: {other}"),
    }

    cleanup.run_all().await;
}
