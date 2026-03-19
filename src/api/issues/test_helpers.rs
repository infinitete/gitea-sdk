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

pub fn issue_json(id: i64, number: i64, title: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "url": "",
        "html_url": "",
        "number": number,
        "user": {
            "id": 1,
            "login": "testuser",
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
        },
        "original_author": "",
        "original_author_id": 0,
        "title": title,
        "body": "Issue body",
        "ref": "",
        "labels": [],
        "milestone": null,
        "assignees": [],
        "state": "open",
        "is_locked": false,
        "comments": 0,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T10:00:00Z",
        "closed_at": null,
        "due_date": null,
        "pull_request": null,
        "repository": null
    })
}

pub fn comment_json(id: i64, body: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "html_url": "",
        "pull_request_url": "",
        "issue_url": "",
        "poster": {
            "id": 1,
            "login": "testuser",
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
        },
        "original_author": "",
        "original_author_id": 0,
        "body": body,
        "created": "2024-01-15T10:00:00Z",
        "updated": "2024-01-15T10:00:00Z"
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

pub fn milestone_json(id: i64, title: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "title": title,
        "description": "",
        "state": "open",
        "open_issues": 0,
        "closed_issues": 0,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T10:00:00Z",
        "closed_at": null,
        "due_on": null
    })
}

pub fn reaction_json(content: &str) -> serde_json::Value {
    serde_json::json!({
        "user": null,
        "reaction": content,
        "created": "2024-01-15T10:00:00Z"
    })
}

pub fn user_json() -> serde_json::Value {
    serde_json::json!({
        "id": 1,
        "login": "testuser",
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

pub fn attachment_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "name": "file.txt",
        "size": 1024,
        "download_count": 0,
        "created": "2024-01-15T10:00:00Z",
        "uuid": "abc123",
        "browser_download_url": ""
    })
}

pub fn stopwatch_json() -> serde_json::Value {
    serde_json::json!({
        "created": "2024-01-15T10:00:00Z",
        "seconds": 3600,
        "duration": "1h0m0s",
        "issue_index": 1,
        "issue_title": "Bug fix",
        "repo_owner_name": "owner",
        "repo_name": "repo"
    })
}

pub fn tracked_time_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "created": "2024-01-15T10:00:00Z",
        "time": 1800,
        "user_id": 0,
        "user_name": "",
        "issue_id": 0
    })
}

pub fn timeline_comment_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "html_url": "",
        "pull_request_url": "",
        "issue_url": "",
        "user": null,
        "original_author": "",
        "original_author_id": 0,
        "body": "comment",
        "created": "2024-01-15T10:00:00Z",
        "updated": "2024-01-15T10:00:00Z",
        "type": "comment",
        "label": [],
        "milestone": null,
        "old_milestone": null,
        "new_title": "",
        "old_title": ""
    })
}

pub fn watch_info_json() -> serde_json::Value {
    serde_json::json!({
        "subscribed": true,
        "watching": true,
        "ignored": false,
        "reason": "",
        "created_at": null,
        "url": "",
        "repository_url": ""
    })
}

pub fn issue_template_json() -> serde_json::Value {
    serde_json::json!({
        "name": "Bug Report",
        "about": "File a bug",
        "file_name": "bug_report.md",
        "title": "Bug: ",
        "labels": ["bug"],
        "ref": "",
        "form": [],
        "content": "Describe the bug..."
    })
}

pub fn error_body() -> serde_json::Value {
    serde_json::json!({"message": "error"})
}
