// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::status::CreateStatusOption;
use gitea_sdk::types::enums::StatusState;

use live::{CleanupRegistry, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_commit_status_lifecycle() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-status-repo")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let (repo_data, repo_response) = client
        .repos()
        .get_repo(&owner, &repo)
        .await
        .expect("get repo");
    assert_success_status(repo_response.status);
    let default_branch = repo_data.default_branch.clone();

    let (branch_ref, ref_response) = client
        .repos()
        .get_repo_ref(&owner, &repo, &format!("refs/heads/{default_branch}"))
        .await
        .expect("get default branch ref");
    assert_success_status(ref_response.status);
    let sha = branch_ref.object.as_ref().expect("ref object").sha.clone();

    let context = unique_name("live-status");
    let (created, create_response) = client
        .status()
        .create_status(
            &owner,
            &repo,
            &sha,
            CreateStatusOption {
                state: StatusState::Success,
                target_url: Some("https://example.com/ci/live-status".to_string()),
                description: Some("live status check".to_string()),
                context: Some(context.clone()),
            },
        )
        .await
        .expect("create status");
    assert_success_status(create_response.status);
    assert_eq!(created.context, context);
    assert_eq!(created.state, StatusState::Success);

    let (statuses, list_response) = client
        .status()
        .list_statuses(&owner, &repo, &sha, Default::default())
        .await
        .expect("list statuses");
    assert_success_status(list_response.status);
    assert!(statuses.iter().any(|status| status.context == context));

    let (combined, combined_response) = client
        .status()
        .get_combined_status(&owner, &repo, &sha)
        .await
        .expect("get combined status");
    assert_success_status(combined_response.status);
    assert_eq!(combined.sha, sha);
    assert!(
        combined
            .statuses
            .iter()
            .any(|status| status.context == context)
    );

    cleanup.run_all().await;
}
