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

pub fn org_json(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "username": name,
        "full_name": "",
        "email": "",
        "avatar_url": "",
        "description": "",
        "website": "",
        "location": "",
        "visibility": "public",
        "repo_admin_change_team_access": false
    })
}

pub fn team_json(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "description": "",
        "organization": null,
        "permission": "read",
        "can_create_org_repo": false,
        "includes_all_repositories": false,
        "units": []
    })
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

pub fn label_json(id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": name,
        "color": "ff0000",
        "description": "",
        "exclusive": false,
        "is_archived": false,
        "url": ""
    })
}

pub fn secret_json(name: &str) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "data": "secret-value",
        "description": "",
        "created": "2026-01-15T10:00:00Z"
    })
}

pub fn org_action_variable_json(owner_id: i64, name: &str) -> serde_json::Value {
    serde_json::json!({
        "owner_id": owner_id,
        "repo_id": 0,
        "name": name,
        "data": "var-value",
        "description": ""
    })
}

pub fn activity_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "act_user_id": 1,
        "act_user": null,
        "op_type": "create_repo",
        "content": "",
        "repo_id": 1,
        "repo": null,
        "comment_id": 0,
        "comment": null,
        "ref_name": "",
        "is_private": false,
        "user_id": 1,
        "created": "2026-01-15T10:00:00Z"
    })
}

pub fn org_permissions_json() -> serde_json::Value {
    serde_json::json!({
        "can_create_repository": true,
        "can_read": true,
        "can_write": false,
        "is_admin": false,
        "is_owner": true
    })
}

pub fn make_minimal_repo_json() -> serde_json::Value {
    let mut repo_json: serde_json::Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/repository.json")).unwrap();
    if let serde_json::Value::Object(map) = &mut repo_json {
        map.insert("owner".to_string(), serde_json::Value::Null);
        map.insert("template".to_string(), serde_json::Value::Bool(false));
        map.insert("mirror".to_string(), serde_json::Value::Bool(false));
        map.insert("size".to_string(), serde_json::Value::from(0));
        map.insert(
            "language".to_string(),
            serde_json::Value::String(String::new()),
        );
        map.insert(
            "languages_url".to_string(),
            serde_json::Value::String(
                "https://example.com/api/v1/repos/test/languages".to_string(),
            ),
        );
        map.insert(
            "url".to_string(),
            serde_json::Value::String(
                "https://example.com/api/v1/repos/testuser/test-repo".to_string(),
            ),
        );
        map.insert("link".to_string(), serde_json::Value::String(String::new()));
        map.insert(
            "original_url".to_string(),
            serde_json::Value::String(String::new()),
        );
        map.insert(
            "website".to_string(),
            serde_json::Value::String(String::new()),
        );
        map.insert("stars_count".to_string(), serde_json::Value::from(0));
        map.insert("forks_count".to_string(), serde_json::Value::from(0));
        map.insert("watchers_count".to_string(), serde_json::Value::from(0));
        map.insert("open_issues_count".to_string(), serde_json::Value::from(0));
        map.insert("open_pr_counter".to_string(), serde_json::Value::from(0));
        map.insert("release_counter".to_string(), serde_json::Value::from(0));
        map.insert("archived".to_string(), serde_json::Value::Bool(false));
        map.insert(
            "archived_at".to_string(),
            serde_json::Value::String("2026-01-01T00:00:00Z".to_string()),
        );
        map.insert(
            "mirror_interval".to_string(),
            serde_json::Value::String(String::new()),
        );
        map.insert(
            "mirror_updated".to_string(),
            serde_json::Value::String("2026-01-15T10:30:00Z".to_string()),
        );
        map.insert("repo_transfer".to_string(), serde_json::Value::Null);
        map.insert("permissions".to_string(), serde_json::Value::Null);
        map.insert("has_issues".to_string(), serde_json::Value::Bool(true));
        map.insert("has_code".to_string(), serde_json::Value::Bool(true));
        map.insert("internal_tracker".to_string(), serde_json::Value::Null);
        map.insert("external_tracker".to_string(), serde_json::Value::Null);
        map.insert("has_wiki".to_string(), serde_json::Value::Bool(true));
        map.insert("external_wiki".to_string(), serde_json::Value::Null);
        map.insert(
            "has_pull_requests".to_string(),
            serde_json::Value::Bool(true),
        );
        map.insert("has_projects".to_string(), serde_json::Value::Bool(false));
        map.insert("has_releases".to_string(), serde_json::Value::Bool(true));
        map.insert("has_packages".to_string(), serde_json::Value::Bool(false));
        map.insert("has_actions".to_string(), serde_json::Value::Bool(false));
        map.insert(
            "ignore_whitespace_conflicts".to_string(),
            serde_json::Value::Bool(false),
        );
        map.insert(
            "allow_merge_commits".to_string(),
            serde_json::Value::Bool(true),
        );
        map.insert("allow_rebase".to_string(), serde_json::Value::Bool(true));
        map.insert(
            "allow_rebase_explicit".to_string(),
            serde_json::Value::Bool(true),
        );
        map.insert(
            "allow_rebase_update".to_string(),
            serde_json::Value::Bool(true),
        );
        map.insert(
            "allow_squash_merge".to_string(),
            serde_json::Value::Bool(true),
        );
        map.insert(
            "allow_fast_forward_only_merge".to_string(),
            serde_json::Value::Bool(false),
        );
        map.insert(
            "default_allow_maintainer_edit".to_string(),
            serde_json::Value::Bool(true),
        );
        map.insert(
            "default_delete_branch_after_merge".to_string(),
            serde_json::Value::Bool(false),
        );
        map.insert(
            "default_merge_style".to_string(),
            serde_json::Value::String("merge".to_string()),
        );
        map.insert(
            "avatar_url".to_string(),
            serde_json::Value::String(String::new()),
        );
        map.insert("internal".to_string(), serde_json::Value::Bool(false));
        map.insert(
            "mirror_updated_unix".to_string(),
            serde_json::Value::from(0),
        );
        map.insert("projects_mode".to_string(), serde_json::Value::Null);
        map.insert(
            "created_at".to_string(),
            serde_json::Value::String("2026-01-01T00:00:00Z".to_string()),
        );
        map.insert(
            "updated_at".to_string(),
            serde_json::Value::String("2026-01-15T10:30:00Z".to_string()),
        );
        map.insert(
            "object_format_name".to_string(),
            serde_json::Value::String(String::new()),
        );
        map.insert("topics".to_string(), serde_json::Value::Array(vec![]));
        map.insert("licenses".to_string(), serde_json::Value::Array(vec![]));
    }
    repo_json
}
