// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.
use crate::Client;
use wiremock::MockServer;

#[allow(dead_code)]
pub fn create_test_client(server: &MockServer) -> Client {
    Client::builder(&server.uri())
        .token("test-token")
        .gitea_version("")
        .build()
        .unwrap()
}

pub fn minimal_repo_json(id: i64, name: &str, owner_name: &str) -> serde_json::Value {
    let ts = "2024-01-01T00:00:00Z";
    let owner = serde_json::json!({
        "id": 1, "login": owner_name, "full_name": owner_name,
        "email": "", "login_name": "", "source_id": 0,
        "avatar_url": "", "html_url": "", "language": "",
        "is_admin": false, "restricted": false, "active": false,
        "prohibit_login": false, "location": "", "website": "",
        "description": "", "visibility": "public",
        "followers_count": 0, "following_count": 0, "starred_repos_count": 0,
    });
    let base = serde_json::json!({
        "id": id,
        "owner": owner,
        "name": name,
        "full_name": format!("{}/{}", owner_name, name),
        "default_branch": "main",
        "archived": false,
        "archived_at": ts,
        "created_at": ts,
        "updated_at": ts,
        "has_issues": true,
        "has_code": true,
        "has_wiki": true,
        "has_pull_requests": true,
        "default_merge_style": "merge",
        "object_format_name": "sha1",
    });
    let mut map = base.as_object().unwrap().clone();
    let extra: Vec<(String, serde_json::Value)> = vec![
        ("description".into(), serde_json::json!("")),
        ("empty".into(), serde_json::json!(true)),
        ("private".into(), serde_json::json!(false)),
        ("fork".into(), serde_json::json!(false)),
        ("template".into(), serde_json::json!(false)),
        ("mirror".into(), serde_json::json!(false)),
        ("size".into(), serde_json::json!(0)),
        ("language".into(), serde_json::json!("")),
        ("languages_url".into(), serde_json::json!("")),
        ("html_url".into(), serde_json::json!("")),
        ("url".into(), serde_json::json!("")),
        ("link".into(), serde_json::json!("")),
        ("ssh_url".into(), serde_json::json!("")),
        ("clone_url".into(), serde_json::json!("")),
        ("original_url".into(), serde_json::json!("")),
        ("website".into(), serde_json::json!("")),
        ("stars_count".into(), serde_json::json!(0)),
        ("forks_count".into(), serde_json::json!(0)),
        ("watchers_count".into(), serde_json::json!(0)),
        ("open_issues_count".into(), serde_json::json!(0)),
        ("open_pr_counter".into(), serde_json::json!(0)),
        ("release_counter".into(), serde_json::json!(0)),
        (
            "ignore_whitespace_conflicts".into(),
            serde_json::json!(false),
        ),
        (
            "allow_fast_forward_only_merge".into(),
            serde_json::json!(false),
        ),
        ("allow_merge_commits".into(), serde_json::json!(true)),
        ("allow_rebase".into(), serde_json::json!(true)),
        ("allow_rebase_explicit".into(), serde_json::json!(true)),
        ("allow_rebase_update".into(), serde_json::json!(false)),
        ("allow_squash_merge".into(), serde_json::json!(true)),
        (
            "default_allow_maintainer_edit".into(),
            serde_json::json!(false),
        ),
        ("has_projects".into(), serde_json::json!(true)),
        ("avatar_url".into(), serde_json::json!("")),
        ("internal".into(), serde_json::json!(false)),
        ("mirror_interval".into(), serde_json::json!("")),
        (
            "default_delete_branch_after_merge".into(),
            serde_json::json!(false),
        ),
    ];
    for (key, val) in extra {
        map.insert(key, val);
    }
    serde_json::Value::Object(map)
}

// ── Batch C: Branch/Tag Protection, Transfer, Team, Deploy Keys,
//    Forks, Blob, Git Hooks, Refs, Compare, Notes, Action Secrets/Variables ──

#[allow(dead_code)]
pub fn minimal_branch_protection_json() -> serde_json::Value {
    let ts = "2024-01-01T00:00:00Z";
    serde_json::json!({
        "branch_name": "main",
        "rule_name": "main",
        "enable_push": false,
        "enable_push_whitelist": false,
        "push_whitelist_usernames": [],
        "push_whitelist_teams": [],
        "push_whitelist_deploy_keys": false,
        "enable_merge_whitelist": false,
        "merge_whitelist_usernames": [],
        "merge_whitelist_teams": [],
        "enable_status_check": false,
        "status_check_contexts": [],
        "required_approvals": 0,
        "enable_approvals_whitelist": false,
        "approvals_whitelist_username": [],
        "approvals_whitelist_teams": [],
        "block_on_rejected_reviews": false,
        "block_on_official_review_requests": false,
        "block_on_outdated_branch": false,
        "dismiss_stale_approvals": false,
        "require_signed_commits": false,
        "protected_file_patterns": "",
        "unprotected_file_patterns": "",
        "created_at": ts,
        "updated_at": ts,
    })
}

pub fn minimal_tag_protection_json(id: i64) -> serde_json::Value {
    let ts = "2024-01-01T00:00:00Z";
    serde_json::json!({
        "id": id,
        "name_pattern": "v*",
        "whitelist_usernames": [],
        "whitelist_teams": [],
        "created_at": ts,
        "updated_at": ts,
    })
}

pub fn minimal_team_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": "developers",
        "description": "Dev team",
        "permission": "write",
        "can_create_org_repo": true,
        "includes_all_repositories": false,
        "units": [],
    })
}

pub fn minimal_deploy_key_json(id: i64) -> serde_json::Value {
    let ts = "2024-01-01T00:00:00Z";
    serde_json::json!({
        "id": id,
        "key_id": id,
        "key": "ssh-rsa AAAAB3...",
        "url": "",
        "title": "deploy-key",
        "fingerprint": "ab:cd:ef",
        "created_at": ts,
        "read_only": true,
    })
}

pub fn minimal_git_hook_json() -> serde_json::Value {
    serde_json::json!({
        "name": "pre-receive",
        "is_active": true,
        "content": "#!/bin/sh\necho hello",
    })
}

pub fn minimal_reference_json() -> serde_json::Value {
    serde_json::json!({
        "ref": "refs/heads/main",
        "url": "https://example.com/api/v1/repos/o/r/git/refs/heads/main",
        "object": {
            "type": "commit",
            "sha": "abc123",
            "url": "https://example.com/api/v1/repos/o/r/git/commits/abc123",
        },
    })
}

pub fn minimal_git_blob_json() -> serde_json::Value {
    serde_json::json!({
        "content": "SGVsbG8gV29ybGQ=",
        "encoding": "base64",
        "url": "https://example.com/api/v1/repos/o/r/git/blobs/abc123",
        "sha": "abc123",
        "size": 11,
    })
}

pub fn minimal_compare_json() -> serde_json::Value {
    serde_json::json!({
        "total_commits": 1,
        "commits": [],
    })
}

pub fn minimal_note_json() -> serde_json::Value {
    serde_json::json!({
        "message": "Test note",
    })
}

pub fn minimal_secret_json(name: &str) -> serde_json::Value {
    let ts = "2024-01-01T00:00:00Z";
    serde_json::json!({
        "name": name,
        "data": "",
        "description": "",
        "created": ts,
    })
}

pub fn minimal_repo_action_variable_json(name: &str, value: &str) -> serde_json::Value {
    serde_json::json!({
        "owner_id": 1,
        "repo_id": 1,
        "name": name,
        "data": value,
    })
}

#[allow(dead_code)]
pub fn minimal_user_json(id: i64, login: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "login": login,
        "login_name": "",
        "source_id": 0,
        "full_name": login,
        "email": "",
        "avatar_url": "",
        "html_url": "",
        "language": "",
        "is_admin": false,
        "restricted": false,
        "active": false,
        "prohibit_login": false,
        "location": "",
        "website": "",
        "description": "",
        "visibility": "public",
        "followers_count": 0,
        "following_count": 0,
        "starred_repos_count": 0,
    })
}

pub fn minimal_commit_json(sha: &str) -> serde_json::Value {
    serde_json::json!({
        "url": format!("https://gitea.example.com/api/v1/repos/owner/repo/git/commits/{sha}"),
        "sha": sha,
        "created": "2024-01-01T00:00:00Z",
        "html_url": "",
    })
}

#[allow(dead_code)]
pub fn minimal_push_mirror_json() -> serde_json::Value {
    serde_json::json!({
        "created": "2024-01-01T00:00:00Z",
        "interval": "8h",
        "last_error": "",
        "last_update": "2024-01-01T00:00:00Z",
        "remote_address": "https://example.com/repo.git",
        "remote_name": "origin",
        "repo_name": "repo",
        "sync_on_commit": false,
    })
}

#[allow(dead_code)]
pub fn minimal_label_json(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "color": "ff0000",
        "description": "",
        "exclusive": false,
        "is_archived": false,
        "url": "",
    })
}
