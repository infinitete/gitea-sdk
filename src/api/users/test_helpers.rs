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

pub fn gpg_key_json(id: i64) -> serde_json::Value {
    serde_json::json!({
        "id": id,
        "primary_key_id": "0",
        "key_id": format!("KEY{id}"),
        "public_key": "-----BEGIN PGP PUBLIC KEY BLOCK-----",
        "emails": [],
        "subs_key": [],
        "can_sign": true,
        "can_encrypt_comms": false,
        "can_encrypt_storage": false,
        "can_certify": true,
        "verified": true
    })
}
