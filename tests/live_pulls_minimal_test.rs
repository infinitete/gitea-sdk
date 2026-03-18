// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine;
use gitea_sdk::options::pull::{
    CreatePullRequestOption, EditPullRequestOption, ListPullRequestCommitsOptions,
    ListPullRequestFilesOptions, ListPullRequestsOptions, MergePullRequestOption,
    PullRequestDiffOptions,
};
use gitea_sdk::options::repo::{CreateBranchOption, CreateFileOptions, FileOptions};
use gitea_sdk::types::enums::{MergeStyle, StateType};
use gitea_sdk::types::repository::{CommitDateOptions, Identity};
use time::OffsetDateTime;
use tokio::time::{Duration, sleep};

use live::{CleanupRegistry, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

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

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_pull_minimal_happy_path() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-pr")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let (repo_data, repo_resp) = client
        .repos()
        .get_repo(&owner, &repo)
        .await
        .expect("get repo");
    assert_success_status(repo_resp.status);
    let default_branch = repo_data.default_branch;

    let head_branch = unique_name("live-pr-branch");
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
        .expect("create branch");
    assert_success_status(create_branch_resp.status);

    let path = "docs/pr-change.txt";
    let content = base64::engine::general_purpose::STANDARD.encode("pull request change\n");
    let (_, create_file_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            path,
            CreateFileOptions {
                file_options: file_options("create pr change", &head_branch),
                content,
            },
        )
        .await
        .expect("create file on head branch");
    assert_success_status(create_file_resp.status);

    let pr_title = unique_name("live-pr-title");
    let (created, create_resp) = client
        .pulls()
        .create(
            &owner,
            &repo,
            CreatePullRequestOption {
                head: head_branch.clone(),
                base: default_branch.clone(),
                title: pr_title.clone(),
                body: Some("live pull body".to_string()),
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
    assert_success_status(create_resp.status);
    let index = created.index;

    let (pulls, list_resp) = client
        .pulls()
        .list(
            &owner,
            &repo,
            ListPullRequestsOptions {
                state: StateType::Open,
                ..Default::default()
            },
        )
        .await
        .expect("list pull requests");
    assert_success_status(list_resp.status);
    assert!(pulls.iter().any(|pull| pull.index == index));

    let (loaded, get_resp) = client
        .pulls()
        .get(&owner, &repo, index)
        .await
        .expect("get pull");
    assert_success_status(get_resp.status);
    assert_eq!(loaded.title, pr_title);

    let edited_title = unique_name("live-pr-edited");
    let (edited, edit_resp) = client
        .pulls()
        .edit(
            &owner,
            &repo,
            index,
            EditPullRequestOption {
                title: Some(edited_title.clone()),
                body: Some("updated live pull body".to_string()),
                ..Default::default()
            },
        )
        .await
        .expect("edit pull");
    assert_success_status(edit_resp.status);
    assert_eq!(edited.title, edited_title);

    let (patch, patch_resp) = client
        .pulls()
        .patch(&owner, &repo, index)
        .await
        .expect("get pull patch");
    assert_success_status(patch_resp.status);
    assert!(!patch.is_empty());

    let (diff, diff_resp) = client
        .pulls()
        .diff(&owner, &repo, index, PullRequestDiffOptions::default())
        .await
        .expect("get pull diff");
    assert_success_status(diff_resp.status);
    assert!(!diff.is_empty());

    let (commits, commits_resp) = client
        .pulls()
        .list_commits(
            &owner,
            &repo,
            index,
            ListPullRequestCommitsOptions::default(),
        )
        .await
        .expect("list pull commits");
    assert_success_status(commits_resp.status);
    assert!(!commits.is_empty());

    let (files, files_resp) = client
        .pulls()
        .list_files(&owner, &repo, index, ListPullRequestFilesOptions::default())
        .await
        .expect("list pull files");
    assert_success_status(files_resp.status);
    assert!(files.iter().any(|file| file.filename == path));

    let (merged_before, merged_before_resp) = client
        .pulls()
        .is_merged(&owner, &repo, index)
        .await
        .expect("is merged before");
    assert!(merged_before_resp.status == 204 || merged_before_resp.status == 404);
    assert!(!merged_before);

    let mut loaded = loaded;
    for _ in 0..10 {
        if loaded.mergeable {
            break;
        }
        sleep(Duration::from_millis(200)).await;
        let (refreshed, refreshed_resp) = client
            .pulls()
            .get(&owner, &repo, index)
            .await
            .expect("refresh pull before merge");
        assert_success_status(refreshed_resp.status);
        loaded = refreshed;
    }

    match client
        .pulls()
        .merge(
            &owner,
            &repo,
            index,
            MergePullRequestOption {
                style: Some(MergeStyle::Squash),
                title: Some(edited_title.clone()),
                message: Some(format!("squash: {edited_title}")),
                head_commit_id: loaded.head.as_ref().map(|head| head.sha.clone()),
                delete_branch_after_merge: false,
                ..Default::default()
            },
        )
        .await
    {
        Ok((merged_now, merge_resp)) => {
            if merge_resp.status == 405 {
                println!(
                    "[pulls capability] live merge endpoint returned HTTP 405 after waiting for mergeability on a disposable same-repo PR; keeping merge-specific coverage blocked on this instance"
                );
                cleanup.run_all().await;
                return;
            }
            assert_success_status(merge_resp.status);
            assert!(merged_now);

            let (merged_after, merged_after_resp) = client
                .pulls()
                .is_merged(&owner, &repo, index)
                .await
                .expect("is merged after");
            assert!(merged_after_resp.status == 204 || merged_after_resp.status == 404);
            assert!(merged_after);
        }
        Err(gitea_sdk::Error::UnknownApi { status: 422, .. }) => {
            println!(
                "[pulls capability] live merge endpoint returned HTTP 422 for a disposable same-repo PR; keeping merge-specific coverage blocked on this instance"
            );
        }
        Err(other) => panic!("merge pull: {other}"),
    }

    cleanup.run_all().await;
}
