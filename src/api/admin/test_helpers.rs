// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use wiremock::MockServer;

pub fn create_test_client(server: &MockServer) -> Client {
    Client::builder(&server.uri())
        .token("test-token")
        .gitea_version("")
        .build()
        .unwrap()
}

pub fn user_json(id: i64, login: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "login": login,
        "login_name": "",
        "source_id": 0,
        "full_name": "",
        "email": "",
        "avatar_url": "",
        "html_url": "",
        "language": "",
        "is_admin": false,
        "restricted": false,
        "active": true,
        "prohibit_login": false,
        "location": "",
        "website": "",
        "description": "",
        "visibility": "public",
        "followers_count": 0,
        "following_count": 0,
        "starred_repos_count": 0
    })
}

pub fn org_json() -> serde_json::Value {
    serde_json::json!({
        "id": 99,
        "name": "neworg",
        "username": "neworg",
        "full_name": "New Org",
        "email": "org@example.com",
        "avatar_url": "https://example.com/avatar.png",
        "description": "Org desc",
        "website": "https://example.com",
        "location": "Earth",
        "visibility": "public",
        "repo_admin_change_team_access": true
    })
}

pub fn repo_json() -> serde_json::Value {
    let mut repo = serde_json::Map::new();
    repo.insert("id".to_string(), serde_json::Value::from(2));
    repo.insert("owner".to_string(), serde_json::Value::Null);
    repo.insert("name".to_string(), serde_json::Value::from("newrepo"));
    repo.insert(
        "full_name".to_string(),
        serde_json::Value::from("janedoe/newrepo"),
    );
    repo.insert(
        "description".to_string(),
        serde_json::Value::from("A new repo"),
    );
    repo.insert("empty".to_string(), serde_json::Value::from(true));
    repo.insert("private".to_string(), serde_json::Value::from(false));
    repo.insert("fork".to_string(), serde_json::Value::from(false));
    repo.insert("template".to_string(), serde_json::Value::from(false));
    repo.insert("parent".to_string(), serde_json::Value::Null);
    repo.insert("mirror".to_string(), serde_json::Value::from(false));
    repo.insert("size".to_string(), serde_json::Value::from(0));
    repo.insert("language".to_string(), serde_json::Value::from("Rust"));
    repo.insert(
        "languages_url".to_string(),
        serde_json::Value::from("https://example.com/langs"),
    );
    repo.insert(
        "html_url".to_string(),
        serde_json::Value::from("https://example.com/janedoe/newrepo"),
    );
    repo.insert(
        "url".to_string(),
        serde_json::Value::from("https://api.example.com/repos/janedoe/newrepo"),
    );
    repo.insert(
        "link".to_string(),
        serde_json::Value::from("https://example.com/janedoe/newrepo"),
    );
    repo.insert(
        "ssh_url".to_string(),
        serde_json::Value::from("git@example.com:janedoe/newrepo.git"),
    );
    repo.insert(
        "clone_url".to_string(),
        serde_json::Value::from("https://example.com/janedoe/newrepo.git"),
    );
    repo.insert(
        "original_url".to_string(),
        serde_json::Value::from("https://example.com/janedoe/newrepo.git"),
    );
    repo.insert("website".to_string(), serde_json::Value::from(""));
    repo.insert("stars_count".to_string(), serde_json::Value::from(0));
    repo.insert("forks_count".to_string(), serde_json::Value::from(0));
    repo.insert("watchers_count".to_string(), serde_json::Value::from(0));
    repo.insert("open_issues_count".to_string(), serde_json::Value::from(0));
    repo.insert("open_pr_counter".to_string(), serde_json::Value::from(0));
    repo.insert("release_counter".to_string(), serde_json::Value::from(0));
    repo.insert(
        "default_branch".to_string(),
        serde_json::Value::from("main"),
    );
    repo.insert("archived".to_string(), serde_json::Value::from(false));
    repo.insert(
        "archived_at".to_string(),
        serde_json::Value::from("2026-01-01T00:00:00Z"),
    );
    repo.insert(
        "created_at".to_string(),
        serde_json::Value::from("2026-01-01T00:00:00Z"),
    );
    repo.insert(
        "updated_at".to_string(),
        serde_json::Value::from("2026-01-02T00:00:00Z"),
    );
    repo.insert("permissions".to_string(), serde_json::Value::Null);
    repo.insert("has_issues".to_string(), serde_json::Value::from(true));
    repo.insert("has_code".to_string(), serde_json::Value::from(true));
    repo.insert("internal_tracker".to_string(), serde_json::Value::Null);
    repo.insert("external_tracker".to_string(), serde_json::Value::Null);
    repo.insert("has_wiki".to_string(), serde_json::Value::from(true));
    repo.insert("external_wiki".to_string(), serde_json::Value::Null);
    repo.insert(
        "has_pull_requests".to_string(),
        serde_json::Value::from(true),
    );
    repo.insert("has_projects".to_string(), serde_json::Value::from(true));
    repo.insert("has_releases".to_string(), serde_json::Value::from(true));
    repo.insert("has_packages".to_string(), serde_json::Value::from(false));
    repo.insert("has_actions".to_string(), serde_json::Value::from(false));
    repo.insert(
        "ignore_whitespace_conflicts".to_string(),
        serde_json::Value::from(false),
    );
    repo.insert(
        "allow_fast_forward_only_merge".to_string(),
        serde_json::Value::from(false),
    );
    repo.insert(
        "allow_merge_commits".to_string(),
        serde_json::Value::from(true),
    );
    repo.insert("allow_rebase".to_string(), serde_json::Value::from(true));
    repo.insert(
        "allow_rebase_explicit".to_string(),
        serde_json::Value::from(true),
    );
    repo.insert(
        "allow_rebase_update".to_string(),
        serde_json::Value::from(true),
    );
    repo.insert(
        "allow_squash_merge".to_string(),
        serde_json::Value::from(true),
    );
    repo.insert(
        "default_allow_maintainer_edit".to_string(),
        serde_json::Value::from(true),
    );
    repo.insert("avatar_url".to_string(), serde_json::Value::from(""));
    repo.insert("internal".to_string(), serde_json::Value::from(false));
    repo.insert("mirror_interval".to_string(), serde_json::Value::from(""));
    repo.insert("mirror_updated".to_string(), serde_json::Value::Null);
    repo.insert(
        "default_merge_style".to_string(),
        serde_json::Value::from("merge"),
    );
    repo.insert("projects_mode".to_string(), serde_json::Value::Null);
    repo.insert(
        "default_delete_branch_after_merge".to_string(),
        serde_json::Value::from(false),
    );
    repo.insert(
        "object_format_name".to_string(),
        serde_json::Value::from(""),
    );
    repo.insert("topics".to_string(), serde_json::Value::Array(vec![]));
    repo.insert("licenses".to_string(), serde_json::Value::Array(vec![]));
    repo.insert("repo_transfer".to_string(), serde_json::Value::Null);
    serde_json::Value::Object(repo)
}

pub fn hook_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "type": "slack",
        "branch_filter": "",
        "config": {"url": "https://example.com/hook"},
        "events": ["push"],
        "authorization_header": "",
        "active": true,
        "updated_at": "2024-01-15T10:00:00Z",
        "created_at": "2024-01-15T10:00:00Z"
    })
}

pub fn public_key_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "key": "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC...",
        "title": "my-key",
        "fingerprint": "SHA256:abc123",
        "created": "2024-01-15T10:00:00Z",
        "last_used_at": "2024-01-16T10:00:00Z"
    })
}

pub fn cron_task_json(name: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "schedule": "@daily",
        "next": "2024-02-01T00:00:00Z",
        "prev": "2024-01-31T00:00:00Z",
        "exec_times": 10
    })
}

pub fn email_json(email: &str) -> serde_json::Value {
    serde_json::json!({
        "email": email,
        "verified": true,
        "primary": false,
        "user_id": 1,
        "username": "testuser"
    })
}

pub fn badge_json(id: i64, slug: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "slug": slug,
        "description": "A badge",
        "image_url": "https://example.com/badge.png"
    })
}
