// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use wiremock::MockServer;

pub(crate) fn create_test_client(server: &MockServer) -> Client {
    Client::builder(&server.uri())
        .token("test-token")
        .gitea_version("")
        .build()
        .unwrap()
}

pub(crate) fn pr_json(id: i64, number: i64, title: &str, state: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "url": "",
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
        "title": title,
        "body": "",
        "labels": [],
        "milestone": null,
        "assignee": null,
        "assignees": [],
        "requested_reviewers": [],
        "requested_reviewers_teams": [],
        "state": state,
        "draft": false,
        "is_locked": false,
        "comments": 0,
        "html_url": "",
        "diff_url": "",
        "patch_url": "",
        "mergeable": false,
        "merged": false,
        "merged_at": null,
        "merge_commit_sha": null,
        "merged_by": null,
        "allow_maintainer_edit": false,
        "base": {
            "label": "main",
            "ref": "main",
            "sha": "abc123",
            "repo_id": 1,
            "repo": {
                "id": 1,
                "name": "testrepo",
                "full_name": "testowner/testrepo",
                "owner": {
                    "id": 1,
                    "login": "testowner"
                }
            }
        },
        "head": {
            "label": "feature",
            "ref": "feature",
            "sha": "def456",
            "repo_id": 2,
            "repo": null
        },
        "merge_base": "",
        "due_date": null,
        "created_at": "2024-01-15T10:00:00Z",
        "updated_at": "2024-01-15T10:00:00Z",
        "closed_at": null,
        "pin_order": 0
    })
}

pub(crate) fn review_json(id: i64, state: &str, body: &str) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "user": {
            "id": 1,
            "login": "reviewer",
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
        "team": null,
        "state": state,
        "body": body,
        "commit_id": "abc123",
        "stale": false,
        "official": false,
        "dismissed": false,
        "comments_count": 0,
        "submitted_at": "2024-01-15T10:00:00Z",
        "html_url": "",
        "pull_request_url": ""
    })
}
