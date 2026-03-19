// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine;
use gitea_rs::Error;
use gitea_rs::options::pull::{
    CreatePullRequestOption, CreatePullReviewComment, CreatePullReviewOptions,
    DismissPullReviewOptions, ListPullReviewsOptions, SubmitPullReviewOptions,
};
use gitea_rs::options::repo::{
    AddCollaboratorOption, CreateBranchOption, CreateFileOptions, FileOptions,
};
use gitea_rs::types::enums::{AccessMode, ReviewStateType};
use gitea_rs::types::repository::{CommitDateOptions, Identity};
use live::{
    CleanupRegistry, create_repo_fixture, live_client, load_live_env, next_user_client, unique_name,
};
use time::OffsetDateTime;

fn file_options(message: &str, branch: &str) -> FileOptions {
    let now = OffsetDateTime::now_utc();
    FileOptions {
        message: message.to_string(),
        branch_name: branch.to_string(),
        new_branch_name: String::new(),
        author: Identity {
            name: "gitea-sdk live".to_string(),
            email: "gitea-sdk-live@example.com".to_string(),
        },
        committer: Identity {
            name: "gitea-sdk live".to_string(),
            email: "gitea-sdk-live@example.com".to_string(),
        },
        dates: CommitDateOptions {
            author: now,
            committer: now,
        },
        signoff: false,
    }
}

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_pull_review_dual_user_flow() {
    let client = live_client();
    let reviewer = next_user_client();
    let mut cleanup = CleanupRegistry::new();
    let env = load_live_env();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-pr-dual")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let next_user_name = env.next_user_credentials().0.to_string();
    let collab_opt = AddCollaboratorOption {
        permission: Some(AccessMode::Write),
    };
    let collab_resp = client
        .repos()
        .add_collaborator(&owner, &repo, &next_user_name, collab_opt)
        .await
        .expect("add next user collaborator");
    assert_success_status(collab_resp.status);
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_repo = repo.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .repos()
            .delete_collaborator(&cleanup_owner, &cleanup_repo, &next_user_name)
            .await;
    });

    let head_branch = unique_name("live-pr-dual-branch");
    let (_, create_branch_resp) = client
        .repos()
        .create_branch(
            &owner,
            &repo,
            CreateBranchOption {
                branch_name: head_branch.clone(),
                old_branch_name: repo_fixture.repository.default_branch.clone(),
            },
        )
        .await
        .expect("create PR branch");
    assert_success_status(create_branch_resp.status);

    let path = "docs/pr-review-dual.txt";
    let content = base64::engine::general_purpose::STANDARD.encode("dual user review change\n");
    let (_, create_file_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            path,
            CreateFileOptions {
                file_options: file_options("dual review change", &head_branch),
                content,
            },
        )
        .await
        .expect("create file for review");
    assert_success_status(create_file_resp.status);

    let pr_title = unique_name("live-pr-review-dual");
    let (created_pr, create_pr_resp) = client
        .pulls()
        .create(
            &owner,
            &repo,
            CreatePullRequestOption {
                head: head_branch.clone(),
                base: repo_fixture.repository.default_branch.clone(),
                title: pr_title.clone(),
                body: Some("dual review body".to_string()),
                assignee: None,
                assignees: Vec::new(),
                reviewers: Vec::new(),
                team_reviewers: Vec::new(),
                milestone: 0,
                labels: Vec::new(),
                deadline: None,
            },
        )
        .await
        .expect("create pull request");
    assert_success_status(create_pr_resp.status);
    let pr_index = created_pr.index;
    let head_sha = created_pr
        .head
        .as_ref()
        .map(|head| head.sha.clone())
        .unwrap_or_default();

    let comment = CreatePullReviewComment {
        path: path.to_string(),
        body: "Dual user review comment".to_string(),
        old_line_num: 0,
        new_line_num: 1,
    };
    let review_opt = CreatePullReviewOptions {
        body: Some("Dual review".to_string()),
        commit_id: Some(head_sha.clone()),
        comments: vec![comment],
        state: None,
    };

    let (created_review, review_resp) = reviewer
        .pulls()
        .create_review(&owner, &repo, pr_index, review_opt)
        .await
        .expect("create review as reviewer");
    assert_success_status(review_resp.status);

    let (reviews, list_resp) = client
        .pulls()
        .list_reviews(&owner, &repo, pr_index, ListPullReviewsOptions::default())
        .await
        .expect("list reviews");
    assert_success_status(list_resp.status);
    assert!(reviews.iter().any(|review| review.id == created_review.id));

    let (fetched_review, get_resp) = client
        .pulls()
        .get_review(&owner, &repo, pr_index, created_review.id)
        .await
        .expect("get review");
    assert_success_status(get_resp.status);
    assert_eq!(fetched_review.body, created_review.body);

    let (comments, comments_resp) = client
        .pulls()
        .list_review_comments(&owner, &repo, pr_index, created_review.id)
        .await
        .expect("list review comments");
    assert_success_status(comments_resp.status);
    assert!(
        comments
            .iter()
            .any(|c| c.body == "Dual user review comment")
    );
    let submit_opt = SubmitPullReviewOptions {
        state: Some(ReviewStateType::Approved),
        body: Some("dual user approval".to_string()),
    };
    match reviewer
        .pulls()
        .submit_review(&owner, &repo, pr_index, created_review.id, submit_opt)
        .await
    {
        Ok((submitted, submit_resp)) => {
            assert_success_status(submit_resp.status);
            assert_eq!(submitted.state, ReviewStateType::Approved);

            let dismiss_opt = DismissPullReviewOptions {
                message: Some("live dismiss".to_string()),
            };
            match client
                .pulls()
                .dismiss_review(&owner, &repo, pr_index, created_review.id, dismiss_opt)
                .await
            {
                Ok(dismiss_resp) => assert_success_status(dismiss_resp.status),
                Err(Error::Api { status: 403, .. }) => {
                    println!("[pulls capability] dismiss blocked with 403, skipping");
                }
                Err(other) => panic!("dismiss review: {other}"),
            }

            match client
                .pulls()
                .undismiss_review(&owner, &repo, pr_index, created_review.id)
                .await
            {
                Ok(undismiss_resp) => assert_success_status(undismiss_resp.status),
                Err(Error::Api { status: 403, .. }) => {
                    println!("[pulls capability] undismiss blocked with 403, skipping");
                }
                Err(other) => panic!("undismiss review: {other}"),
            }

            match client
                .pulls()
                .delete_review(&owner, &repo, pr_index, created_review.id)
                .await
            {
                Ok(delete_resp) => assert_success_status(delete_resp.status),
                Err(Error::Api { status: 403, .. }) => {
                    println!("[pulls capability] delete review blocked with 403, skipping");
                }
                Err(other) => panic!("delete review: {other}"),
            }
        }
        Err(Error::UnknownApi { status: 422, .. }) => {
            println!(
                "[pulls capability] approved dual-user review returned 422 (policy); keeping submit path blocked"
            );
        }
        Err(other) => panic!("submit review (dual user): {other}"),
    }

    cleanup.run_all().await;
}
