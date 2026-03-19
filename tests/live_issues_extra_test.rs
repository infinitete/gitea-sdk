// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk_rs::options::issue::{
    AddTimeOption, CreateIssueCommentOption, ListTrackedTimesOptions, LockIssueOption,
};

use live::{
    CleanupRegistry, create_issue_fixture, create_repo_fixture, live_client, load_live_env,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_extra_flows() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issues-extra")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let issue_a = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-extra-a")
        .await
        .expect("create issue a");
    let issue_b = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-extra-b")
        .await
        .expect("create issue b");
    let index_a = issue_a.issue.index;
    let index_b = issue_b.issue.index;

    let (comment, comment_resp) = client
        .issues()
        .create_issue_comment(
            &owner,
            &repo,
            index_a,
            CreateIssueCommentOption {
                body: "live extra comment".to_string(),
            },
        )
        .await
        .expect("create issue comment");
    assert_success_status(comment_resp.status);
    let comment_id = comment.id;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_repo = repo.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .issues()
            .delete_issue_comment(&cleanup_owner, &cleanup_repo, comment_id)
            .await;
    });

    let (_comment_reaction, reaction_resp) = client
        .issues()
        .post_issue_comment_reaction(&owner, &repo, comment_id, "+1")
        .await
        .expect("add comment reaction");
    assert_success_status(reaction_resp.status);
    let (comment_reactions, get_comment_reactions_resp) = client
        .issues()
        .get_issue_comment_reactions(&owner, &repo, comment_id)
        .await
        .expect("get issue comment reactions");
    assert_success_status(get_comment_reactions_resp.status);
    assert!(
        comment_reactions
            .iter()
            .any(|reaction| reaction.reaction == "+1")
    );
    let delete_comment_reaction_resp = client
        .issues()
        .delete_issue_comment_reaction(&owner, &repo, comment_id, "+1")
        .await
        .expect("delete comment reaction");
    assert_success_status(delete_comment_reaction_resp.status);

    let (tracked, tracked_resp) = client
        .issues()
        .add_time(
            &owner,
            &repo,
            index_a,
            AddTimeOption {
                time: 180,
                created: None,
                user: env.user_name.clone(),
            },
        )
        .await
        .expect("add tracked time");
    assert_success_status(tracked_resp.status);
    let tracked_id = tracked.id;

    let (repo_times, repo_times_resp) = client
        .issues()
        .list_repo_tracked_times(&owner, &repo, ListTrackedTimesOptions::default())
        .await
        .expect("list repo tracked times");
    assert_success_status(repo_times_resp.status);
    assert!(repo_times.iter().any(|entry| entry.id == tracked_id));

    let (my_times, my_times_resp) = client
        .issues()
        .list_my_tracked_times(ListTrackedTimesOptions::default())
        .await
        .expect("list my tracked times");
    assert_success_status(my_times_resp.status);
    assert!(my_times.iter().any(|entry| entry.id == tracked_id));

    let (issue_times, issue_times_resp) = client
        .issues()
        .list_issue_tracked_times(&owner, &repo, index_a, ListTrackedTimesOptions::default())
        .await
        .expect("list issue tracked times");
    assert_success_status(issue_times_resp.status);
    assert!(issue_times.iter().any(|entry| entry.id == tracked_id));

    let reset_resp = client
        .issues()
        .reset_issue_time(&owner, &repo, index_a)
        .await
        .expect("reset issue time");
    assert_success_status(reset_resp.status);

    let (issue_times_after_reset, issue_times_after_reset_resp) = client
        .issues()
        .list_issue_tracked_times(&owner, &repo, index_a, ListTrackedTimesOptions::default())
        .await
        .expect("list issue tracked times after reset");
    assert_success_status(issue_times_after_reset_resp.status);
    assert!(issue_times_after_reset.is_empty());

    let pin_a_resp = client
        .issues()
        .pin_issue(&owner, &repo, index_a)
        .await
        .expect("pin issue a");
    assert_success_status(pin_a_resp.status);
    let pin_b_resp = client
        .issues()
        .pin_issue(&owner, &repo, index_b)
        .await
        .expect("pin issue b");
    assert_success_status(pin_b_resp.status);

    let (pinned_before_move, pinned_before_move_resp) = client
        .issues()
        .list_repo_pinned_issues(&owner, &repo)
        .await
        .expect("list pinned issues before move");
    assert_success_status(pinned_before_move_resp.status);
    assert!(
        pinned_before_move
            .iter()
            .any(|issue| issue.index == index_a)
    );
    assert!(
        pinned_before_move
            .iter()
            .any(|issue| issue.index == index_b)
    );

    let move_pin_resp = client
        .issues()
        .move_issue_pin(&owner, &repo, index_b, 1)
        .await
        .expect("move issue pin");
    assert_success_status(move_pin_resp.status);

    let (pinned_after_move, pinned_after_move_resp) = client
        .issues()
        .list_repo_pinned_issues(&owner, &repo)
        .await
        .expect("list pinned issues after move");
    assert_success_status(pinned_after_move_resp.status);
    assert_eq!(
        pinned_after_move.first().map(|issue| issue.index),
        Some(index_b)
    );

    let unpin_b_resp = client
        .issues()
        .unpin_issue(&owner, &repo, index_b)
        .await
        .expect("unpin issue b");
    assert_success_status(unpin_b_resp.status);
    let unpin_a_resp = client
        .issues()
        .unpin_issue(&owner, &repo, index_a)
        .await
        .expect("unpin issue a");
    assert_success_status(unpin_a_resp.status);

    let lock_resp = client
        .issues()
        .lock_issue(
            &owner,
            &repo,
            index_a,
            LockIssueOption {
                lock_reason: "resolved".to_string(),
            },
        )
        .await
        .expect("lock issue");
    assert_success_status(lock_resp.status);
    let unlock_resp = client
        .issues()
        .unlock_issue(&owner, &repo, index_a)
        .await
        .expect("unlock issue");
    assert_success_status(unlock_resp.status);

    cleanup.run_all().await;
}
