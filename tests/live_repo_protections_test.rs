// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use gitea_sdk::Error;
use gitea_sdk::options::repo::{
    CreateBranchProtectionOption, CreateTagProtectionOption, EditBranchProtectionOption,
    EditTagProtectionOption, ListBranchProtectionsOptions, ListRepoTagProtectionsOptions,
};

use live::{CleanupRegistry, create_repo_fixture, live_client, load_live_env};

fn assert_success_status(status: u16) {
    assert!(
        (200..300).contains(&status),
        "expected 2xx status, got {status}"
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_branch_and_tag_protections() {
    let client = live_client();
    let env = load_live_env();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-protection")
        .await
        .expect("create repo fixture");
    let owner = repo_fixture.owner.clone();
    let repo = repo_fixture.repository.name.clone();
    let branch_name = repo_fixture.repository.default_branch.clone();

    let created_branch = match client
        .repos()
        .create_branch_protection(
            &owner,
            &repo,
            CreateBranchProtectionOption {
                branch_name: branch_name.clone(),
                rule_name: branch_name.clone(),
                enable_push: true,
                enable_push_whitelist: false,
                push_whitelist_usernames: Vec::new(),
                push_whitelist_teams: Vec::new(),
                push_whitelist_deploy_keys: false,
                enable_merge_whitelist: false,
                merge_whitelist_usernames: Vec::new(),
                merge_whitelist_teams: Vec::new(),
                enable_status_check: false,
                status_check_contexts: Vec::new(),
                required_approvals: 0,
                enable_approvals_whitelist: false,
                approvals_whitelist_usernames: Vec::new(),
                approvals_whitelist_teams: Vec::new(),
                block_on_rejected_reviews: false,
                block_on_official_review_requests: false,
                block_on_outdated_branch: false,
                dismiss_stale_approvals: false,
                require_signed_commits: false,
                protected_file_patterns: String::new(),
                unprotected_file_patterns: String::new(),
            },
        )
        .await
    {
        Ok(result) => result,
        Err(Error::Api {
            status, message, ..
        }) if status == 403
            || (status == 404
                && (message.contains("protected branch")
                    || message.contains("branch protections")
                    || message.contains("Not Found"))) =>
        {
            println!(
                "[repo protections] live instance denied or lacks branch protection support (status {status}, message: {message}); keeping protection methods blocked"
            );
            cleanup.run_all().await;
            return;
        }
        Err(other) => panic!("create branch protection: {other}"),
    };
    assert_success_status(created_branch.1.status);

    let (branch_protections, list_branch_resp) = client
        .repos()
        .list_branch_protections(&owner, &repo, ListBranchProtectionsOptions::default())
        .await
        .expect("list branch protections");
    assert_success_status(list_branch_resp.status);
    assert!(
        branch_protections
            .iter()
            .any(|item| item.branch_name == branch_name)
    );

    let (loaded_branch, get_branch_resp) = client
        .repos()
        .get_branch_protection(&owner, &repo, &branch_name)
        .await
        .expect("get branch protection");
    assert_success_status(get_branch_resp.status);
    assert_eq!(loaded_branch.branch_name, branch_name);

    let (edited_branch, edit_branch_resp) = client
        .repos()
        .edit_branch_protection(
            &owner,
            &repo,
            &branch_name,
            EditBranchProtectionOption {
                enable_push: Some(true),
                enable_push_whitelist: Some(false),
                push_whitelist_usernames: Vec::new(),
                push_whitelist_teams: Vec::new(),
                push_whitelist_deploy_keys: Some(false),
                enable_merge_whitelist: Some(false),
                merge_whitelist_usernames: Vec::new(),
                merge_whitelist_teams: Vec::new(),
                enable_status_check: Some(false),
                status_check_contexts: Vec::new(),
                required_approvals: Some(0),
                enable_approvals_whitelist: Some(false),
                approvals_whitelist_usernames: Vec::new(),
                approvals_whitelist_teams: Vec::new(),
                block_on_rejected_reviews: Some(false),
                block_on_official_review_requests: Some(false),
                block_on_outdated_branch: Some(false),
                dismiss_stale_approvals: Some(false),
                require_signed_commits: Some(false),
                protected_file_patterns: Some(String::new()),
                unprotected_file_patterns: Some(String::new()),
            },
        )
        .await
        .expect("edit branch protection");
    assert_success_status(edit_branch_resp.status);
    assert_eq!(edited_branch.branch_name, branch_name);

    let delete_branch_resp = client
        .repos()
        .delete_branch_protection(&owner, &repo, &branch_name)
        .await
        .expect("delete branch protection");
    assert_success_status(delete_branch_resp.status);

    let created_tag = match client
        .repos()
        .create_tag_protection(
            &owner,
            &repo,
            CreateTagProtectionOption {
                name_pattern: "live-protected-*".to_string(),
                whitelist_usernames: vec![env.user_name.clone()],
                whitelist_teams: Vec::new(),
            },
        )
        .await
    {
        Ok(result) => result,
        Err(Error::Api {
            status, message, ..
        }) if status == 403 || status == 404 => {
            println!(
                "[repo protections] live instance denied or lacks tag protection support (status {status}, message: {message}); leaving tag-protection methods blocked"
            );
            cleanup.run_all().await;
            return;
        }
        Err(other) => panic!("create tag protection: {other}"),
    };
    assert_success_status(created_tag.1.status);
    let tag_protection_id = created_tag.0.id;

    let (tag_protections, list_tag_resp) = client
        .repos()
        .list_tag_protections(&owner, &repo, ListRepoTagProtectionsOptions::default())
        .await
        .expect("list tag protections");
    assert_success_status(list_tag_resp.status);
    assert!(
        tag_protections
            .iter()
            .any(|item| item.id == tag_protection_id)
    );

    let (loaded_tag, get_tag_resp) = client
        .repos()
        .get_tag_protection(&owner, &repo, tag_protection_id)
        .await
        .expect("get tag protection");
    assert_success_status(get_tag_resp.status);
    assert_eq!(loaded_tag.id, tag_protection_id);

    let (edited_tag, edit_tag_resp) = client
        .repos()
        .edit_tag_protection(
            &owner,
            &repo,
            tag_protection_id,
            EditTagProtectionOption {
                name_pattern: Some("live-protected-updated-*".to_string()),
                whitelist_usernames: vec![env.user_name.clone()],
                whitelist_teams: Vec::new(),
            },
        )
        .await
        .expect("edit tag protection");
    assert_success_status(edit_tag_resp.status);
    assert_eq!(edited_tag.id, tag_protection_id);

    let delete_tag_resp = client
        .repos()
        .delete_tag_protection(&owner, &repo, tag_protection_id)
        .await
        .expect("delete tag protection");
    assert_success_status(delete_tag_resp.status);

    cleanup.run_all().await;
}
