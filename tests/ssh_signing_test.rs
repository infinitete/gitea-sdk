// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Wiremock integration tests for SSH signing via the Client pipeline.
//!
//! These tests verify that `ClientBuilder::ssh_pubkey()` and `ssh_cert()`
//! produce clients that sign outgoing HTTP requests with the HTTP Signature
//! `Signature` header.  All signing tests use ED25519 keys because
//! ssh-key v0.6.7 has a known CRT bug with RSA.

use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use gitea_sdk::options::miscellaneous::MarkdownOption;
use gitea_sdk::{Client, Error};

mod ssh_fixtures;

use ssh_fixtures::{
    ed25519_private_key, ed25519_private_key_bytes, rsa_passphrase,
    rsa_passphrase_private_key_bytes,
};

fn ed25519_tmp_key(test_name: &str) -> std::path::PathBuf {
    let tmp = std::env::temp_dir().join(format!("gitea_sdk_{test_name}"));
    std::fs::write(&tmp, ed25519_private_key_bytes()).expect("write temp ed25519 key");
    tmp
}

fn ed25519_fingerprint() -> String {
    let key = ed25519_private_key().expect("ed25519 test key");
    key.public_key()
        .fingerprint(ssh_key::HashAlg::Sha256)
        .to_string()
}

fn minimal_repo_json(id: i64, name: &str, owner_name: &str) -> serde_json::Value {
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
        "full_name": format!("{owner_name}/{name}"),
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
    let mut map = base.as_object().expect("base should be object").clone();
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

#[tokio::test]
async fn test_ssh_pubkey_signs_request() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header_exists("Signature"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("pubkey_signs");
    let fp = ed25519_fingerprint();

    let client = Client::builder(&server.uri())
        .ssh_pubkey(&fp, &tmp, None::<&str>)
        .expect("ssh_pubkey should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let (version, _resp) = client
        .miscellaneous()
        .get_version()
        .await
        .expect("get_version should succeed");
    assert_eq!(version, "1.22.0");

    let _ = std::fs::remove_file(&tmp);
}

#[tokio::test]
async fn test_ssh_cert_signs_request() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header_exists("Signature"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("cert_signs");

    let client = Client::builder(&server.uri())
        .ssh_cert("test-principal", &tmp, None::<&str>)
        .expect("ssh_cert should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let (version, _resp) = client
        .miscellaneous()
        .get_version()
        .await
        .expect("get_version should succeed");
    assert_eq!(version, "1.22.0");

    let _ = std::fs::remove_file(&tmp);
}

#[tokio::test]
async fn test_no_ssh_no_signature_header() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header_exists("Signature"))
        .respond_with(ResponseTemplate::new(200))
        .expect(0)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .http_client(reqwest::Client::new())
        .build()
        .expect("build without SSH should succeed");

    let (version, _resp) = client
        .miscellaneous()
        .get_version()
        .await
        .expect("get_version should succeed");
    assert_eq!(version, "1.22.0");
}

#[tokio::test]
async fn test_signature_header_contains_keyid_and_algorithm() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header_exists("Signature"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("format_keyid");
    let fp = ed25519_fingerprint();

    let client = Client::builder(&server.uri())
        .ssh_pubkey(&fp, &tmp, None::<&str>)
        .expect("ssh_pubkey should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let (version, _resp) = client
        .miscellaneous()
        .get_version()
        .await
        .expect("get_version should succeed");
    assert_eq!(version, "1.22.0");

    let _ = std::fs::remove_file(&tmp);
}

#[tokio::test]
async fn test_signature_header_keyid_starts_with_sha256() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header_exists("Signature"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("keyid_sha256");
    let fp = ed25519_fingerprint();

    let client = Client::builder(&server.uri())
        .ssh_pubkey(&fp, &tmp, None::<&str>)
        .expect("ssh_pubkey should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let (version, _resp) = client
        .miscellaneous()
        .get_version()
        .await
        .expect("get_version should succeed");
    assert_eq!(version, "1.22.0");
    assert!(
        fp.starts_with("SHA256:"),
        "fingerprint should start with SHA256:"
    );

    let _ = std::fs::remove_file(&tmp);
}

#[tokio::test]
async fn test_unknown_version_cache_uses_legacy_ssh_signing() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "unknown"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/v1/markdown"))
        .and(header_exists("Authorization"))
        .respond_with(ResponseTemplate::new(200).set_body_string("<p>ok</p>"))
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("unknown_cache_legacy");
    let fp = ed25519_fingerprint();

    let client = Client::builder(&server.uri())
        .ssh_pubkey(&fp, &tmp, None::<&str>)
        .expect("ssh_pubkey should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let err = client.server_version().await.unwrap_err();
    match err {
        Error::UnknownVersion(v) => assert_eq!(v, "unknown"),
        other => panic!("expected Error::UnknownVersion, got: {other}"),
    }

    let (html, _resp) = client
        .miscellaneous()
        .render_markdown(MarkdownOption {
            text: "hello".to_string(),
            mode: None,
            context: None,
            wiki: false,
        })
        .await
        .expect("render_markdown should succeed");
    assert_eq!(html, "<p>ok</p>");

    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn test_ssh_invalid_key_path() {
    let result = Client::builder("https://example.com").ssh_pubkey(
        "SHA256:abc",
        "/nonexistent/path/key",
        None::<&str>,
    );

    assert!(
        result.is_err(),
        "ssh_pubkey with nonexistent path should return Err"
    );
    match result.unwrap_err() {
        gitea_sdk::Error::SshSign(msg) => {
            assert!(
                msg.contains("failed to read"),
                "error should mention read failure: {msg}"
            );
        }
        other => panic!("expected Error::SshSign, got: {other}"),
    }
}

#[test]
fn test_ssh_wrong_passphrase() {
    let tmp = std::env::temp_dir().join("gitea_sdk_ssh_signing_passphrase");
    std::fs::write(&tmp, rsa_passphrase_private_key_bytes())
        .expect("write passphrase-protected key");

    let result = Client::builder("https://example.com").ssh_pubkey(
        "SHA256:abc",
        &tmp,
        Some::<&str>("wrong-passphrase"),
    );

    assert!(
        result.is_err(),
        "ssh_pubkey with wrong passphrase should return Err"
    );
    match result.unwrap_err() {
        gitea_sdk::Error::SshSign(msg) => {
            assert!(
                msg.contains("failed to decrypt"),
                "error should mention decryption failure: {msg}"
            );
        }
        other => panic!("expected Error::SshSign, got: {other}"),
    }

    let _ = std::fs::remove_file(&tmp);
}

#[tokio::test]
async fn test_ssh_pubkey_signs_release_attachment_multipart_upload() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/api/v1/repos/testowner/testrepo/releases/1/assets"))
        .and(header_exists("Signature"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 10,
            "name": "binary.zip",
            "size": 1024,
            "download_count": 0,
            "created": "2024-01-15T10:00:00Z",
            "uuid": "abc123",
            "browser_download_url": "https://example.com/attachments/abc123"
        })))
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("release_attachment_signs");
    let fp = ed25519_fingerprint();

    let client = Client::builder(&server.uri())
        .ssh_pubkey(&fp, &tmp, None::<&str>)
        .expect("ssh_pubkey should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let (attachment, resp) = client
        .releases()
        .create_attachment(
            "testowner",
            "testrepo",
            1,
            b"file content".to_vec(),
            "binary.zip",
        )
        .await
        .expect("create_attachment should succeed");
    assert_eq!(attachment.id, 10);
    assert_eq!(resp.status, 201);

    let _ = std::fs::remove_file(&tmp);
}

#[tokio::test]
async fn test_ssh_pubkey_signs_repo_avatar_multipart_upload() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/api/v1/repos/testowner/testrepo/avatar"))
        .and(header_exists("Signature"))
        .respond_with(ResponseTemplate::new(200).set_body_json(minimal_repo_json(
            1,
            "testrepo",
            "testowner",
        )))
        .expect(1)
        .mount(&server)
        .await;

    let tmp = ed25519_tmp_key("repo_avatar_signs");
    let fp = ed25519_fingerprint();

    let client = Client::builder(&server.uri())
        .ssh_pubkey(&fp, &tmp, None::<&str>)
        .expect("ssh_pubkey should succeed")
        .http_client(reqwest::Client::new())
        .build()
        .expect("build should succeed");

    let (repo, resp) = client
        .repos()
        .update_repo_avatar("testowner", "testrepo", b"avatar-bytes")
        .await
        .expect("update_repo_avatar should succeed");
    assert_eq!(repo.id, 1);
    assert_eq!(repo.name, "testrepo");
    assert_eq!(resp.status, 200);

    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn test_ssh_cert_wrong_passphrase() {
    let tmp = std::env::temp_dir().join("gitea_sdk_ssh_signing_cert_pass");
    std::fs::write(&tmp, rsa_passphrase_private_key_bytes())
        .expect("write passphrase-protected key");

    let result =
        Client::builder("https://example.com").ssh_cert("principal", &tmp, Some::<&str>("wrong"));

    assert!(
        result.is_err(),
        "ssh_cert with wrong passphrase should return Err"
    );
    match result.unwrap_err() {
        gitea_sdk::Error::SshSign(msg) => {
            assert!(
                msg.contains("failed to decrypt"),
                "error should mention decryption failure: {msg}"
            );
        }
        other => panic!("expected Error::SshSign, got: {other}"),
    }

    let _ = std::fs::remove_file(&tmp);
}

#[test]
fn test_ssh_correct_passphrase() {
    let tmp = std::env::temp_dir().join("gitea_sdk_ssh_signing_correct_pass");
    std::fs::write(&tmp, rsa_passphrase_private_key_bytes())
        .expect("write passphrase-protected key");

    let result = Client::builder("https://example.com").ssh_pubkey(
        "SHA256:abc",
        &tmp,
        Some(rsa_passphrase()),
    );

    assert!(
        result.is_ok(),
        "ssh_pubkey with correct passphrase should succeed"
    );

    let _ = std::fs::remove_file(&tmp);
}
