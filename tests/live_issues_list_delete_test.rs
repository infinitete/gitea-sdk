// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::options::issue::ListIssueOption;
use gitea_sdk::types::enums::StateType;

use live::{CleanupRegistry, create_issue_fixture, create_repo_fixture, live_client};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_list_and_delete_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issues-list-delete")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();

    let issue_fixture = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "list-issue")
        .await
        .expect("create issue fixture");
    let issue_index = issue_fixture.issue.index;

    let list_opt = ListIssueOption {
        state: Some(StateType::All),
        ..Default::default()
    };
    let (issues, list_resp) = client
        .issues()
        .list_issues(list_opt.clone())
        .await
        .expect("list issues");
    assert_success_status(list_resp.status);
    assert!(issues.iter().any(|entry| entry.index == issue_index));

    let repo_list_opt = ListIssueOption {
        state: Some(StateType::All),
        ..Default::default()
    };
    let (repo_issues, repo_list_resp) = client
        .issues()
        .list_repo_issues(&owner, &repo, repo_list_opt.clone())
        .await
        .expect("list repo issues");
    assert_success_status(repo_list_resp.status);
    assert!(repo_issues.iter().any(|entry| entry.index == issue_index));

    let issue_to_delete =
        create_issue_fixture(&client, &mut cleanup, &owner, &repo, "delete-issue")
            .await
            .expect("create delete fixture");
    let delete_index = issue_to_delete.issue.index;

    let delete_resp = client
        .issues()
        .delete_issue(&owner, &repo, delete_index)
        .await
        .expect("delete issue");
    assert_success_status(delete_resp.status);

    let repo_list_after_opt = ListIssueOption {
        state: Some(StateType::All),
        ..Default::default()
    };
    let (repo_issues_after, after_resp) = client
        .issues()
        .list_repo_issues(&owner, &repo, repo_list_after_opt)
        .await
        .expect("list repo issues after delete");
    assert_success_status(after_resp.status);
    assert!(
        !repo_issues_after
            .iter()
            .any(|entry| entry.index == delete_index)
    );

    cleanup.run_all().await;
}
