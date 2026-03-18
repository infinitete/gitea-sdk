// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine;
use gitea_sdk::options::issue::CreateIssueCommentOption;
use gitea_sdk::options::release::EditAttachmentOption;
use gitea_sdk::options::repo::{CreateFileOptions, FileOptions};
use gitea_sdk::types::release::Attachment;
use gitea_sdk::types::repository::{CommitDateOptions, Identity};
use reqwest::multipart::{Form, Part};
use time::OffsetDateTime;

use live::{
    CleanupRegistry, create_issue_fixture, create_repo_fixture, live_client, load_live_env,
};

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

async fn upload_issue_comment_attachment(owner: &str, repo: &str, comment_id: i64) -> Attachment {
    let env = load_live_env();
    let url = format!(
        "{}/api/v1/repos/{}/{}/issues/comments/{}/assets",
        env.base_url(),
        owner,
        repo,
        comment_id
    );
    let form = Form::new().part(
        "attachment",
        Part::bytes(b"issue attachment fixture\n".to_vec()).file_name("live-issue-asset.txt"),
    );
    let response = reqwest::Client::new()
        .post(url)
        .header("Authorization", format!("token {}", env.token_value))
        .multipart(form)
        .send()
        .await
        .expect("upload comment attachment");
    let status = response.status();
    assert!(
        status.is_success(),
        "attachment upload failed with HTTP {status}"
    );
    response
        .json::<Attachment>()
        .await
        .expect("parse attachment payload")
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_templates_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issue-template")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let default_branch = repo_fixture.repository.default_branch.clone();

    let content = r#"---
name: Bug Report
about: Report a live issue
title: "[Bug]: "
labels: ["bug"]
---
Describe the bug.
"#;
    let encoded = base64::engine::general_purpose::STANDARD.encode(content);
    let (_, create_template_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            ".gitea/ISSUE_TEMPLATE/bug-report.md",
            CreateFileOptions {
                file_options: file_options("create issue template", &default_branch),
                content: encoded,
            },
        )
        .await
        .expect("create issue template file");
    assert_success_status(create_template_resp.status);

    let (templates, templates_resp) = client
        .issues()
        .get_issue_templates(&owner, &repo)
        .await
        .expect("get issue templates");
    assert_success_status(templates_resp.status);
    assert!(
        !templates.is_empty(),
        "expected at least one issue template"
    );
    assert!(
        templates.iter().any(|entry| {
            entry.filename.contains("bug-report")
                || entry.name.contains("Bug Report")
                || entry.title.contains("[Bug]")
        }),
        "seeded issue template missing from response: {templates:#?}"
    );

    cleanup.run_all().await;
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_issue_comment_attachments_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-issue-attach")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let issue_fixture = create_issue_fixture(&client, &mut cleanup, &owner, &repo, "live-attach")
        .await
        .expect("create issue fixture");
    let issue_index = issue_fixture.issue.index;

    let (comment, comment_resp) = client
        .issues()
        .create_issue_comment(
            &owner,
            &repo,
            issue_index,
            CreateIssueCommentOption {
                body: "attachment host comment".to_string(),
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

    let attachment = upload_issue_comment_attachment(&owner, &repo, comment_id).await;
    let attachment_id = attachment.id;

    let (attachments, list_resp) = client
        .issues()
        .list_issue_comment_attachments(&owner, &repo, comment_id)
        .await
        .expect("list issue comment attachments");
    assert_success_status(list_resp.status);
    assert!(attachments.iter().any(|entry| entry.id == attachment_id));

    let (loaded_attachment, get_resp) = client
        .issues()
        .get_issue_comment_attachment(&owner, &repo, comment_id, attachment_id)
        .await
        .expect("get issue comment attachment");
    assert_success_status(get_resp.status);
    assert_eq!(loaded_attachment.id, attachment_id);

    let (edited_attachment, edit_resp) = client
        .issues()
        .edit_issue_comment_attachment(
            &owner,
            &repo,
            comment_id,
            attachment_id,
            EditAttachmentOption {
                name: "renamed-live-issue-asset.txt".to_string(),
            },
        )
        .await
        .expect("edit issue comment attachment");
    assert_success_status(edit_resp.status);
    assert_eq!(edited_attachment.name, "renamed-live-issue-asset.txt");

    let delete_resp = client
        .issues()
        .delete_issue_comment_attachment(&owner, &repo, comment_id, attachment_id)
        .await
        .expect("delete issue comment attachment");
    assert_success_status(delete_resp.status);

    cleanup.run_all().await;
}
