// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine;
use gitea_sdk_rs::Error;
use gitea_sdk_rs::ListOptions;
use gitea_sdk_rs::options::action::{ListRepoActionJobsOptions, ListRepoActionRunsOptions};
use gitea_sdk_rs::options::repo::{CreateFileOptions, EditRepoOption, FileOptions};
use gitea_sdk_rs::types::repository::{CommitDateOptions, Identity};
use time::OffsetDateTime;
use tokio::time::{Duration, sleep};

use live::{CleanupRegistry, create_repo_fixture, live_client};

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

async fn wait_for_job_logs(
    client: &gitea_sdk_rs::Client,
    owner: &str,
    repo: &str,
    job_id: i64,
) -> Result<Option<Vec<u8>>, Error> {
    for _ in 0..30 {
        match client
            .actions()
            .get_repo_action_job_logs(owner, repo, job_id)
            .await
        {
            Ok((logs, _)) => return Ok(Some(logs.to_vec())),
            Err(Error::Api { status: 404, .. }) => {
                sleep(Duration::from_secs(2)).await;
            }
            Err(Error::UnknownApi { status: 404, .. }) => {
                sleep(Duration::from_secs(2)).await;
            }
            Err(err) => return Err(err),
        }
    }

    Ok(None)
}

async fn wait_for_deletable_run(
    client: &gitea_sdk_rs::Client,
    owner: &str,
    repo: &str,
    run_id: i64,
) -> Result<Option<gitea_sdk_rs::Response>, Error> {
    for _ in 0..30 {
        match client
            .actions()
            .delete_repo_action_run(owner, repo, run_id)
            .await
        {
            Ok(resp) => return Ok(Some(resp)),
            Err(Error::Api {
                status, message, ..
            }) if status == 400 && message.contains("not done") => {
                sleep(Duration::from_secs(2)).await;
            }
            Err(Error::UnknownApi { status, body })
                if status == 400 && body.contains("not done") =>
            {
                sleep(Duration::from_secs(2)).await;
            }
            Err(err) => return Err(err),
        }
    }

    Ok(None)
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_actions_probe_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-actions")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();
    let default_branch = fixture.repository.default_branch.clone();

    let (_, edit_repo_resp) = client
        .repos()
        .edit_repo(
            &owner,
            &repo,
            EditRepoOption {
                has_actions: Some(true),
                ..EditRepoOption {
                    name: None,
                    description: None,
                    website: None,
                    private: None,
                    template: None,
                    has_issues: None,
                    internal_tracker: None,
                    external_tracker: None,
                    has_wiki: None,
                    external_wiki: None,
                    default_branch: None,
                    has_pull_requests: None,
                    has_projects: None,
                    has_releases: None,
                    has_packages: None,
                    has_actions: None,
                    ignore_whitespace_conflicts: None,
                    allow_fast_forward_only_merge: None,
                    allow_merge: None,
                    allow_rebase: None,
                    allow_rebase_merge: None,
                    allow_squash: None,
                    default_delete_branch_after_merge: None,
                    default_merge_style: None,
                    archived: None,
                    mirror_interval: None,
                    allow_manual_merge: None,
                    autodetect_manual_merge: None,
                    projects_mode: None,
                }
            },
        )
        .await
        .expect("enable actions");
    assert_success_status(edit_repo_resp.status);

    let workflow_content = base64::engine::general_purpose::STANDARD.encode(
        "name: live-actions\non:\n  push:\n    branches:\n      - main\njobs:\n  hello:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo live-actions\n",
    );
    let (_, workflow_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            ".gitea/workflows/live-actions.yml",
            CreateFileOptions {
                file_options: file_options("create actions workflow", &default_branch),
                content: workflow_content,
            },
        )
        .await
        .expect("create workflow file");
    assert_success_status(workflow_resp.status);

    let trigger_content =
        base64::engine::general_purpose::STANDARD.encode("trigger actions workflow\n");
    let (_, trigger_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            "docs/actions-trigger.txt",
            CreateFileOptions {
                file_options: file_options("trigger actions workflow", &default_branch),
                content: trigger_content,
            },
        )
        .await
        .expect("create trigger file");
    assert_success_status(trigger_resp.status);

    let mut run_id = None;
    for _ in 0..20 {
        match client
            .actions()
            .list_repo_action_runs(&owner, &repo, ListRepoActionRunsOptions::default())
            .await
        {
            Ok((runs, list_runs_resp)) => {
                assert_success_status(list_runs_resp.status);
                if let Some(run) = runs.workflow_runs.first() {
                    run_id = Some(run.id);
                    break;
                }
            }
            Err(Error::UnknownApi { status, body }) => {
                println!(
                    "[actions capability] list_repo_action_runs returned {status}: {}",
                    body
                );
                cleanup.run_all().await;
                return;
            }
            Err(other) => panic!("list action runs: {other}"),
        }
        sleep(Duration::from_secs(1)).await;
    }

    match client
        .actions()
        .list_repo_action_tasks(&owner, &repo, ListOptions::default())
        .await
    {
        Ok((_tasks, list_tasks_resp)) => assert_success_status(list_tasks_resp.status),
        Err(Error::UnknownApi { status, body }) => {
            println!(
                "[actions capability] list_repo_action_tasks returned {status}: {}",
                body
            );
        }
        Err(other) => panic!("list action tasks: {other}"),
    }

    match client
        .actions()
        .list_repo_action_jobs(&owner, &repo, ListRepoActionJobsOptions::default())
        .await
    {
        Ok((_jobs, list_jobs_resp)) => assert_success_status(list_jobs_resp.status),
        Err(Error::UnknownApi { status, body }) => {
            println!(
                "[actions capability] list_repo_action_jobs returned {status}: {}",
                body
            );
        }
        Err(other) => panic!("list action jobs: {other}"),
    }

    if let Some(run_id) = run_id {
        let (run, get_run_resp) = client
            .actions()
            .get_repo_action_run(&owner, &repo, run_id)
            .await
            .expect("get action run");
        assert_success_status(get_run_resp.status);
        assert_eq!(run.id, run_id);

        let (run_jobs, list_run_jobs_resp) = client
            .actions()
            .list_repo_action_run_jobs(&owner, &repo, run_id, ListRepoActionJobsOptions::default())
            .await
            .expect("list run jobs");
        assert_success_status(list_run_jobs_resp.status);

        if let Some(job) = run_jobs.jobs.first() {
            let (loaded_job, get_job_resp) = client
                .actions()
                .get_repo_action_job(&owner, &repo, job.id)
                .await
                .expect("get action job");
            assert_success_status(get_job_resp.status);
            assert_eq!(loaded_job.id, job.id);

            match wait_for_job_logs(&client, &owner, &repo, job.id).await {
                Ok(Some(logs)) => {
                    assert!(!logs.is_empty(), "action job logs should not be empty");
                }
                Ok(None) => {
                    println!(
                        "[actions capability] get_repo_action_job_logs kept returning 404 after waiting for workflow completion on the configured instance"
                    );
                }
                Err(Error::Api {
                    status, message, ..
                }) => {
                    println!(
                        "[actions capability] get_repo_action_job_logs returned {status}: {message}"
                    );
                }
                Err(Error::UnknownApi { status, body }) => {
                    println!(
                        "[actions capability] get_repo_action_job_logs returned {status}: {}",
                        body
                    );
                }
                Err(other) => panic!("get action job logs: {other}"),
            }
        } else {
            println!(
                "[actions capability] workflow run {run_id} exists but returned no jobs on the configured instance"
            );
        }

        match wait_for_deletable_run(&client, &owner, &repo, run_id).await {
            Ok(Some(delete_resp)) => assert_success_status(delete_resp.status),
            Ok(None) => {
                println!(
                    "[actions capability] delete_repo_action_run kept returning 400 not done after waiting for workflow completion on the configured instance"
                );
            }
            Err(Error::Api {
                status, message, ..
            }) => {
                println!(
                    "[actions capability] delete_repo_action_run returned {status}: {message}"
                );
            }
            Err(Error::UnknownApi { status, body }) => {
                println!(
                    "[actions capability] delete_repo_action_run returned {status}: {}",
                    body
                );
            }
            Err(other) => panic!("delete action run: {other}"),
        }
    } else {
        println!(
            "[actions capability] seeded workflow file and trigger commit, but no action runs appeared on the configured instance"
        );
    }

    cleanup.run_all().await;
}
