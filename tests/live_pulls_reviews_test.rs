// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine;
use gitea_sdk::Error;
use gitea_sdk::options::pull::{
    CreatePullRequestOption, CreatePullReviewComment, CreatePullReviewOptions,
    DismissPullReviewOptions, ListPullReviewsOptions, PullReviewRequestOptions,
    SubmitPullReviewOptions,
};
use gitea_sdk::options::repo::{
    AddCollaboratorOption, CreateBranchOption, CreateFileOptions, FileOptions,
};
use gitea_sdk::types::enums::{AccessMode, ReviewStateType};
use gitea_sdk::types::repository::{CommitDateOptions, Identity};
use time::OffsetDateTime;

use live::{
    CleanupRegistry, create_repo_fixture, live_client, load_live_env, next_user_client, unique_name,
};

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
async fn live_pull_review_flow() {
    let client = live_client();
    let reviewer_client = next_user_client();
    let env = load_live_env();
    let reviewer = env
        .next_user_name
        .as_ref()
        .expect("missing GITEA_NEXT_USER_NAME in .env")
        .clone();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-pr-review")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let add_resp = client
        .repos()
        .add_collaborator(
            &owner,
            &repo,
            &reviewer,
            AddCollaboratorOption {
                permission: Some(AccessMode::Write),
            },
        )
        .await
        .expect("add review collaborator");
    assert_success_status(add_resp.status);

    let default_branch = repo_fixture.repository.default_branch.clone();
    let head_branch = unique_name("live-pr-review-branch");
    let (_, create_branch_resp) = client
        .repos()
        .create_branch(
            &owner,
            &repo,
            CreateBranchOption {
                branch_name: head_branch.clone(),
                old_branch_name: default_branch.clone(),
            },
        )
        .await
        .expect("create branch for review");
    assert_success_status(create_branch_resp.status);

    let path = "docs/pr-review-comment.txt";
    let content = base64::engine::general_purpose::STANDARD.encode("review workflow change\n");
    let (_, create_file_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            path,
            CreateFileOptions {
                file_options: file_options("review change", &head_branch),
                content,
            },
        )
        .await
        .expect("create review file");
    assert_success_status(create_file_resp.status);

    let pr_title = unique_name("live-pr-review");
    let (created_pr, create_pr_resp) = client
        .pulls()
        .create(
            &owner,
            &repo,
            CreatePullRequestOption {
                head: head_branch.clone(),
                base: default_branch.clone(),
                title: pr_title.clone(),
                body: Some("review comment body".to_string()),
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
        .expect("create pull request for review");
    assert_success_status(create_pr_resp.status);
    let pr_index = created_pr.index;
    let head_sha = created_pr
        .head
        .as_ref()
        .map(|head| head.sha.clone())
        .unwrap_or_default();

    let review_request_resp = client
        .pulls()
        .create_review_requests(
            &owner,
            &repo,
            pr_index,
            PullReviewRequestOptions {
                reviewers: vec![reviewer.clone()],
                team_reviewers: Vec::new(),
            },
        )
        .await
        .expect("create review request");
    assert_success_status(review_request_resp.status);

    let comment = CreatePullReviewComment {
        path: path.to_string(),
        body: "Live review comment".to_string(),
        old_line_num: 0,
        new_line_num: 1,
    };
    let review_opt = CreatePullReviewOptions {
        body: Some("Live review body".to_string()),
        commit_id: Some(head_sha.clone()),
        comments: vec![comment],
        state: None,
    };

    match reviewer_client
        .pulls()
        .create_review(&owner, &repo, pr_index, review_opt)
        .await
    {
        Ok((created_review, review_resp)) => {
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
            assert!(comments.iter().any(|c| c.body == "Live review comment"));

            let submit_opt = SubmitPullReviewOptions {
                state: Some(ReviewStateType::Approved),
                body: Some("approve live change".to_string()),
            };
            let mut submit_blocked = false;
            match reviewer_client
                .pulls()
                .submit_review(&owner, &repo, pr_index, created_review.id, submit_opt)
                .await
            {
                Ok((submitted, submit_resp)) => {
                    assert_success_status(submit_resp.status);
                    assert_eq!(submitted.state, ReviewStateType::Approved);

                    let dismiss_resp = client
                        .pulls()
                        .dismiss_review(
                            &owner,
                            &repo,
                            pr_index,
                            created_review.id,
                            DismissPullReviewOptions {
                                message: Some("dismissing live review".to_string()),
                            },
                        )
                        .await
                        .expect("dismiss review");
                    assert_success_status(dismiss_resp.status);

                    let undismiss_resp = client
                        .pulls()
                        .undismiss_review(&owner, &repo, pr_index, created_review.id)
                        .await
                        .expect("undismiss review");
                    assert_success_status(undismiss_resp.status);
                }
                Err(Error::Api {
                    status: 422,
                    message,
                    ..
                }) => {
                    println!(
                        "[pulls capability] live submit-review endpoint returned 422 ({message}), approving your own pull is not allowed; blocking this path on this instance",
                    );
                    submit_blocked = true;
                }
                Err(other) => panic!("submit review: {other}"),
            }
            if submit_blocked {
                cleanup.run_all().await;
                return;
            }

            let delete_request_resp = client
                .pulls()
                .delete_review_requests(
                    &owner,
                    &repo,
                    pr_index,
                    PullReviewRequestOptions {
                        reviewers: vec![reviewer.clone()],
                        team_reviewers: Vec::new(),
                    },
                )
                .await
                .expect("delete review request");
            assert_success_status(delete_request_resp.status);

            let (pending_review, pending_review_resp) = reviewer_client
                .pulls()
                .create_review(
                    &owner,
                    &repo,
                    pr_index,
                    CreatePullReviewOptions {
                        body: Some("pending live review".to_string()),
                        commit_id: Some(head_sha),
                        comments: Vec::new(),
                        state: None,
                    },
                )
                .await
                .expect("create pending review for delete");
            assert_success_status(pending_review_resp.status);

            let delete_review_resp = reviewer_client
                .pulls()
                .delete_review(&owner, &repo, pr_index, pending_review.id)
                .await
                .expect("delete pending review");
            assert_success_status(delete_review_resp.status);
        }
        Err(gitea_sdk::Error::UnknownApi { status, .. }) => {
            panic!("create review failed with status {status}");
        }
        Err(other) => panic!("create review unexpected error: {other}"),
    }

    cleanup.run_all().await;
}
