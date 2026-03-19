// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_rs::options::issue::CreateIssueOption;

use live::{CleanupRegistry, create_issue_fixture, create_repo_fixture, live_client, unique_name};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_workflow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-issue-repo")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let issue_fixture = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-issue")
        .await
        .expect("create issue fixture");

    let (loaded, get_response) = client
        .issues()
        .get_issue(&owner, &repo, issue_fixture.issue.index)
        .await
        .expect("get issue");
    assert_success_status(get_response.status);
    assert_eq!(loaded.index, issue_fixture.issue.index);

    let updated_title = unique_name("live-issue-updated");
    let (edited, edit_response) = client
        .issues()
        .edit_issue(
            &owner,
            &repo,
            issue_fixture.issue.index,
            gitea_rs::options::issue::EditIssueOption {
                title: Some(updated_title.clone()),
                body: None,
                r#ref: None,
                assignees: Vec::new(),
                milestone: None,
                state: None,
                deadline: None,
                remove_deadline: None,
            },
        )
        .await
        .expect("edit issue");
    assert_success_status(edit_response.status);
    assert_eq!(edited.title, updated_title);

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_release_workflow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let fixture = create_repo_fixture(&client, &mut cleanup, "live-release-repo")
        .await
        .expect("create repo fixture");
    let owner = fixture.owner.clone();
    let repo = fixture.repository.name.clone();

    let issue_opt = CreateIssueOption {
        title: unique_name("live-release-seed"),
        body: "seed issue for workflow repo".to_string(),
        r#ref: String::new(),
        assignees: Vec::new(),
        deadline: None,
        milestone: 0,
        labels: Vec::new(),
        closed: false,
    };
    let _ = client
        .issues()
        .create_issue(&owner, &repo, issue_opt)
        .await
        .expect("seed issue before release list");

    let (releases, list_response) = client
        .releases()
        .list(&owner, &repo, Default::default())
        .await
        .expect("list releases");
    assert_success_status(list_response.status);
    if let Some(release) = releases.first() {
        let (loaded, get_response) = client
            .releases()
            .get(&owner, &repo, release.id)
            .await
            .expect("get release by id");
        assert_success_status(get_response.status);
        assert_eq!(loaded.id, release.id);
        assert_eq!(loaded.tag_name, release.tag_name);
    }

    cleanup.run_all().await;
}
