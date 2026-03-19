// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

mod live;

use base64::Engine;
use gitea_rs::options::repo::{
    CreateBranchOption, CreateFileOptions, DeleteFileOptions, FileOptions, GetContentsExtOptions,
    GetRepoNoteOptions, ListCommitOptions, ListRepoBranchesOptions, ListTreeOptions,
    UpdateFileOptions,
};
use gitea_rs::types::repository::{CommitDateOptions, Identity};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use time::OffsetDateTime;

use live::{CleanupRegistry, create_repo_fixture, live_client, load_live_env, unique_name};

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

fn run_git(args: &[&str], cwd: Option<&Path>) {
    let mut cmd = Command::new("git");
    cmd.args(args).env("GIT_TERMINAL_PROMPT", "0");
    if let Some(cwd) = cwd {
        cmd.current_dir(cwd);
    }
    let output = cmd.output().expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

#[tokio::test]
#[ignore = "requires a live Gitea instance configured in ../.env"]
async fn live_repo_content_and_refs_flow() {
    let client = live_client();
    let mut cleanup = CleanupRegistry::new();
    let repo_fixture = create_repo_fixture(&client, &mut cleanup, "live-repo-content")
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

    let editor_config_path = ".editorconfig";
    let editor_config_content =
        base64::engine::general_purpose::STANDARD.encode("root = true\n[*]\ncharset = utf-8\n");
    let (_, editor_config_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            editor_config_path,
            CreateFileOptions {
                file_options: file_options("create editorconfig", &default_branch),
                content: editor_config_content,
            },
        )
        .await
        .expect("create editorconfig");
    assert_success_status(editor_config_resp.status);

    let path = "docs/live-content.txt";
    let initial_content =
        base64::engine::general_purpose::STANDARD.encode("hello from live test\n");
    let (created_file, create_resp) = client
        .repos()
        .create_file(
            &owner,
            &repo,
            path,
            CreateFileOptions {
                file_options: file_options("create live content", &default_branch),
                content: initial_content,
            },
        )
        .await
        .expect("create file");
    assert_success_status(create_resp.status);
    let initial_sha = created_file
        .content
        .as_ref()
        .expect("created content")
        .sha
        .clone();
    let initial_commit_sha = created_file
        .commit
        .as_ref()
        .expect("created commit")
        .commit_meta
        .sha
        .clone();

    let (file, get_file_resp) = client
        .repos()
        .get_file(&owner, &repo, path, &default_branch)
        .await
        .expect("get file");
    assert_success_status(get_file_resp.status);
    assert!(!file.is_empty());

    let (contents, get_contents_resp) = client
        .repos()
        .get_contents(&owner, &repo, path, &default_branch)
        .await
        .expect("get contents");
    assert_success_status(get_contents_resp.status);
    assert_eq!(contents.path, path);

    let (raw_file, raw_resp) = client
        .repos()
        .get_raw_file(&owner, &repo, path, &default_branch)
        .await
        .expect("get raw file");
    assert_success_status(raw_resp.status);
    assert_eq!(String::from_utf8_lossy(&raw_file), "hello from live test\n");

    let (raw_or_lfs_file, raw_or_lfs_resp) = client
        .repos()
        .get_raw_file_or_lfs(&owner, &repo, path, &default_branch)
        .await
        .expect("get raw file or lfs");
    assert_success_status(raw_or_lfs_resp.status);
    assert_eq!(
        String::from_utf8_lossy(&raw_or_lfs_file),
        "hello from live test\n"
    );

    let (file_reader, file_reader_resp) = client
        .repos()
        .get_file_reader(&owner, &repo, path, &default_branch)
        .await
        .expect("get file reader");
    assert_success_status(file_reader_resp.status);
    assert_eq!(
        String::from_utf8_lossy(&file_reader),
        "hello from live test\n"
    );

    let (contents_ext, contents_ext_resp) = client
        .repos()
        .get_contents_ext(
            &owner,
            &repo,
            path,
            &default_branch,
            GetContentsExtOptions::default(),
        )
        .await
        .expect("get contents ext");
    assert_success_status(contents_ext_resp.status);
    assert_eq!(
        contents_ext
            .file_contents
            .as_ref()
            .expect("extended file contents")
            .path,
        path
    );

    match client
        .repos()
        .get_editor_config(&owner, &repo, path, &default_branch)
        .await
    {
        Ok((editor_config, editor_config_read_resp)) => {
            assert_success_status(editor_config_read_resp.status);
            assert!(
                String::from_utf8_lossy(&editor_config).contains("charset = utf-8"),
                "editor config should apply to the seeded file"
            );
        }
        Err(gitea_rs::Error::UnknownApi { status: 404, body }) => {
            println!(
                "[repo content capability] live editorconfig endpoint returned 404 ({body:?}); keeping get_editor_config blocked on this instance"
            );
        }
        Err(other) => panic!("get editor config: {other}"),
    }

    let (listings, list_contents_resp) = client
        .repos()
        .list_contents(&owner, &repo, "docs", &default_branch)
        .await
        .expect("list contents");
    assert_success_status(list_contents_resp.status);
    assert!(listings.iter().any(|entry| entry.path == path));

    let updated_content =
        base64::engine::general_purpose::STANDARD.encode("hello from updated live test\n");
    let (updated_file, update_resp) = client
        .repos()
        .update_file(
            &owner,
            &repo,
            path,
            UpdateFileOptions {
                file_options: file_options("update live content", &default_branch),
                sha: initial_sha,
                content: updated_content,
                from_path: String::new(),
            },
        )
        .await
        .expect("update file");
    assert_success_status(update_resp.status);

    let updated_sha = updated_file
        .content
        .as_ref()
        .expect("updated content")
        .sha
        .clone();
    let updated_commit_sha = updated_file
        .commit
        .as_ref()
        .expect("updated commit")
        .commit_meta
        .sha
        .clone();
    let env = load_live_env();
    let clone_dir = env::temp_dir().join(unique_name("live-repo-note"));
    if clone_dir.exists() {
        let _ = fs::remove_dir_all(&clone_dir);
    }
    let clone_path_str = clone_dir
        .to_str()
        .expect("repo note clone path ASCII")
        .to_string();
    let git_url = format!(
        "http://{user}:{pass}@{host}:{port}/{owner}/{repo}.git",
        user = env.user_name,
        pass = env.user_pass,
        host = env.host,
        port = env.http_port,
        owner = owner,
        repo = repo,
    );
    run_git(&["clone", &git_url, &clone_path_str], None);
    let note_message = format!("live repo note {}", unique_name("repo-note"));
    run_git(
        &["notes", "add", "-m", &note_message, &updated_commit_sha],
        Some(clone_dir.as_path()),
    );
    run_git(
        &["push", "origin", "refs/notes/commits"],
        Some(clone_dir.as_path()),
    );
    let (note, note_resp) = client
        .repos()
        .get_repo_note(
            &owner,
            &repo,
            &updated_commit_sha,
            GetRepoNoteOptions::default(),
        )
        .await
        .expect("get repo note");
    assert_success_status(note_resp.status);
    assert_eq!(note.message.trim_end(), note_message);
    run_git(
        &["notes", "remove", &updated_commit_sha],
        Some(clone_dir.as_path()),
    );
    run_git(
        &["push", "origin", "refs/notes/commits"],
        Some(clone_dir.as_path()),
    );
    fs::remove_dir_all(&clone_dir).expect("remove repo note clone");
    let (blob, blob_resp) = client
        .repos()
        .get_blob(&owner, &repo, &updated_sha)
        .await
        .expect("get blob");
    assert_success_status(blob_resp.status);
    assert!(!blob.content.is_empty());

    let (repo_commits, repo_commits_resp) = client
        .repos()
        .list_commits(
            &owner,
            &repo,
            ListCommitOptions {
                sha: default_branch.clone(),
                ..Default::default()
            },
        )
        .await
        .expect("list repo commits");
    assert_success_status(repo_commits_resp.status);
    assert!(
        repo_commits
            .iter()
            .any(|commit| commit.commit_meta.sha == updated_commit_sha),
        "updated commit should be visible in repo commit list"
    );

    let (single_commit, single_commit_resp) = client
        .repos()
        .get_single_commit(&owner, &repo, &updated_commit_sha)
        .await
        .expect("get single commit");
    assert_success_status(single_commit_resp.status);
    assert_eq!(single_commit.commit_meta.sha, updated_commit_sha);

    let (commit_diff, commit_diff_resp) = client
        .repos()
        .get_commit_diff(&owner, &repo, &updated_commit_sha)
        .await
        .expect("get commit diff");
    assert_success_status(commit_diff_resp.status);
    assert!(
        String::from_utf8_lossy(&commit_diff).contains("live-content.txt"),
        "commit diff should mention the changed file"
    );

    let (commit_patch, commit_patch_resp) = client
        .repos()
        .get_commit_patch(&owner, &repo, &updated_commit_sha)
        .await
        .expect("get commit patch");
    assert_success_status(commit_patch_resp.status);
    assert!(
        String::from_utf8_lossy(&commit_patch).contains("live-content.txt"),
        "commit patch should mention the changed file"
    );

    let (head_ref, head_ref_resp) = client
        .repos()
        .get_repo_ref(&owner, &repo, &format!("refs/heads/{default_branch}"))
        .await
        .expect("get repo ref");
    assert_success_status(head_ref_resp.status);
    let head_sha = head_ref.object.as_ref().expect("head object").sha.clone();
    assert_eq!(head_sha, updated_commit_sha);

    let branch_name = unique_name("live-branch");
    let (_, create_branch_resp) = client
        .repos()
        .create_branch(
            &owner,
            &repo,
            CreateBranchOption {
                branch_name: branch_name.clone(),
                old_branch_name: default_branch.clone(),
            },
        )
        .await
        .expect("create branch");
    assert_success_status(create_branch_resp.status);

    let (branches, list_branch_resp) = client
        .repos()
        .list_branches(&owner, &repo, ListRepoBranchesOptions::default())
        .await
        .expect("list branches");
    assert_success_status(list_branch_resp.status);
    assert!(branches.iter().any(|entry| entry.name == branch_name));

    let (branch, get_branch_resp) = client
        .repos()
        .get_branch(&owner, &repo, &branch_name)
        .await
        .expect("get branch");
    assert_success_status(get_branch_resp.status);
    assert_eq!(branch.name, branch_name);

    let (refs, refs_resp) = client
        .repos()
        .get_repo_refs(&owner, &repo, "refs/heads")
        .await
        .expect("get repo refs");
    assert_success_status(refs_resp.status);
    assert!(refs.iter().any(|entry| entry.ref_.ends_with(&branch_name)));

    let (all_refs, all_refs_resp) = client
        .repos()
        .list_all_git_refs(&owner, &repo)
        .await
        .expect("list all refs");
    assert_success_status(all_refs_resp.status);
    assert!(
        all_refs
            .iter()
            .any(|entry| entry.ref_.ends_with(&branch_name))
    );

    let (comparison, compare_resp) = client
        .repos()
        .compare_commits(&owner, &repo, &initial_commit_sha, &head_sha)
        .await
        .expect("compare commits");
    assert_success_status(compare_resp.status);
    assert!(comparison.total_commits >= 0);

    let (tree, tree_resp) = client
        .repos()
        .get_trees(
            &owner,
            &repo,
            &head_sha,
            ListTreeOptions {
                r#ref: default_branch.clone(),
                recursive: true,
                ..Default::default()
            },
        )
        .await
        .expect("get repository tree");
    assert_success_status(tree_resp.status);
    assert!(
        tree.tree.iter().any(|entry| entry.path == path),
        "repository tree should include the seeded file"
    );

    let delete_branch_resp = client
        .repos()
        .delete_branch(&owner, &repo, &branch_name)
        .await
        .expect("delete branch");
    assert_success_status(delete_branch_resp.status);

    let delete_file_resp = client
        .repos()
        .delete_file(
            &owner,
            &repo,
            path,
            DeleteFileOptions {
                file_options: file_options("delete live content", &default_branch),
                sha: updated_sha,
            },
        )
        .await
        .expect("delete file");
    assert_success_status(delete_file_resp.status);

    cleanup.run_all().await;
}
