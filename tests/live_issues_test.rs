// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk_rs::options::issue::{
    CreateIssueCommentOption, CreateMilestoneOption, EditIssueCommentOption, EditMilestoneOption,
    IssueLabelsOption, ListIssueCommentOptions, ListMilestoneOption,
};
use gitea_sdk_rs::types::enums::StateType;

use live::{
    CleanupRegistry, create_issue_fixture, create_label_fixture, create_repo_fixture, live_client,
    unique_name,
};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_comments_labels_and_milestones() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issue-domain")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let issue_fixture = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-issue")
        .await
        .expect("create issue fixture");
    let issue_index = issue_fixture.issue.index;

    let (comment, create_comment_response) = client
        .issues()
        .create_issue_comment(
            &owner,
            &repo,
            issue_index,
            CreateIssueCommentOption {
                body: "live comment".to_string(),
            },
        )
        .await
        .expect("create issue comment");
    assert_success_status(create_comment_response.status);

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

    let (issue_comments, list_issue_comments_response) = client
        .issues()
        .list_issue_comments(
            &owner,
            &repo,
            issue_index,
            ListIssueCommentOptions::default(),
        )
        .await
        .expect("list issue comments");
    assert_success_status(list_issue_comments_response.status);
    assert!(issue_comments.iter().any(|entry| entry.id == comment_id));

    let (repo_comments, list_repo_comments_response) = client
        .issues()
        .list_repo_issue_comments(&owner, &repo, ListIssueCommentOptions::default())
        .await
        .expect("list repo issue comments");
    assert_success_status(list_repo_comments_response.status);
    assert!(repo_comments.iter().any(|entry| entry.id == comment_id));

    let (loaded_comment, get_comment_response) = client
        .issues()
        .get_issue_comment(&owner, &repo, comment_id)
        .await
        .expect("get issue comment");
    assert_success_status(get_comment_response.status);
    assert_eq!(loaded_comment.id, comment_id);

    let edited_body = "live comment edited".to_string();
    let (edited_comment, edit_comment_response) = client
        .issues()
        .edit_issue_comment(
            &owner,
            &repo,
            comment_id,
            EditIssueCommentOption {
                body: edited_body.clone(),
            },
        )
        .await
        .expect("edit issue comment");
    assert_success_status(edit_comment_response.status);
    assert_eq!(edited_comment.body, edited_body);

    let label_fixture = create_label_fixture(&client, &mut cleanup, &owner, &repo, "issue-label")
        .await
        .expect("create label fixture");
    let label_id = label_fixture.label.id;

    let (added_labels, add_labels_response) = client
        .issues()
        .add_issue_labels(
            &owner,
            &repo,
            issue_index,
            IssueLabelsOption {
                labels: vec![label_id],
            },
        )
        .await
        .expect("add issue labels");
    assert_success_status(add_labels_response.status);
    assert!(added_labels.iter().any(|label| label.id == label_id));

    let (issue_labels, get_labels_response) = client
        .issues()
        .get_issue_labels(&owner, &repo, issue_index, Default::default())
        .await
        .expect("get issue labels");
    assert_success_status(get_labels_response.status);
    assert!(issue_labels.iter().any(|label| label.id == label_id));

    let replacement_label =
        create_label_fixture(&client, &mut cleanup, &owner, &repo, "issue-label-replace")
            .await
            .expect("create replacement label");
    let replacement_id = replacement_label.label.id;

    let (replaced_labels, replace_labels_response) = client
        .issues()
        .replace_issue_labels(
            &owner,
            &repo,
            issue_index,
            IssueLabelsOption {
                labels: vec![replacement_id],
            },
        )
        .await
        .expect("replace issue labels");
    assert_success_status(replace_labels_response.status);
    assert!(
        replaced_labels
            .iter()
            .any(|label| label.id == replacement_id)
    );
    assert!(!replaced_labels.iter().any(|label| label.id == label_id));

    let delete_label_response = client
        .issues()
        .delete_issue_label(&owner, &repo, issue_index, replacement_id)
        .await
        .expect("delete issue label");
    assert_success_status(delete_label_response.status);

    let clear_labels_response = client
        .issues()
        .clear_issue_labels(&owner, &repo, issue_index)
        .await
        .expect("clear issue labels");
    assert_success_status(clear_labels_response.status);

    let milestone_title = unique_name("live-milestone");
    let (milestone, create_milestone_response) = client
        .issues()
        .create_milestone(
            &owner,
            &repo,
            CreateMilestoneOption {
                title: milestone_title.clone(),
                description: "live milestone".to_string(),
                state: StateType::Open,
                deadline: None,
            },
        )
        .await
        .expect("create milestone");
    assert_success_status(create_milestone_response.status);

    let milestone_id = milestone.id;
    let cleanup_client = client.clone();
    let cleanup_owner = owner.clone();
    let cleanup_repo = repo.clone();
    let cleanup_name = milestone_title.clone();
    cleanup.register(async move {
        let _ = cleanup_client
            .issues()
            .delete_milestone_by_name(&cleanup_owner, &cleanup_repo, &cleanup_name)
            .await;
    });

    let (milestones, list_milestones_response) = client
        .issues()
        .list_repo_milestones(&owner, &repo, ListMilestoneOption::default())
        .await
        .expect("list milestones");
    assert_success_status(list_milestones_response.status);
    assert!(milestones.iter().any(|entry| entry.id == milestone_id));

    let (loaded_milestone, get_milestone_response) = client
        .issues()
        .get_milestone(&owner, &repo, milestone_id)
        .await
        .expect("get milestone");
    assert_success_status(get_milestone_response.status);
    assert_eq!(loaded_milestone.id, milestone_id);

    let (named_milestone, get_milestone_by_name_response) = client
        .issues()
        .get_milestone_by_name(&owner, &repo, &milestone_title)
        .await
        .expect("get milestone by name");
    assert_success_status(get_milestone_by_name_response.status);
    assert_eq!(named_milestone.id, milestone_id);

    let edited_title = format!("{milestone_title}-edited");
    let (edited_milestone, edit_milestone_response) = client
        .issues()
        .edit_milestone(
            &owner,
            &repo,
            milestone_id,
            EditMilestoneOption {
                title: Some(edited_title.clone()),
                description: Some("edited milestone".to_string()),
                state: None,
                deadline: None,
            },
        )
        .await
        .expect("edit milestone");
    assert_success_status(edit_milestone_response.status);
    assert_eq!(edited_milestone.title, edited_title);

    let renamed_title = format!("{edited_title}-by-name");
    let (edited_by_name, edit_by_name_response) = client
        .issues()
        .edit_milestone_by_name(
            &owner,
            &repo,
            &edited_title,
            EditMilestoneOption {
                title: Some(renamed_title.clone()),
                description: Some("edited milestone by name".to_string()),
                state: None,
                deadline: None,
            },
        )
        .await
        .expect("edit milestone by name");
    assert_success_status(edit_by_name_response.status);
    assert_eq!(edited_by_name.title, renamed_title);

    let delete_milestone_response = client
        .issues()
        .delete_milestone(&owner, &repo, milestone_id)
        .await
        .expect("delete milestone");
    assert_success_status(delete_milestone_response.status);

    cleanup.run_all().await;
}
