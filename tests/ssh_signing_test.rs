// Copyright 2026 infinitete. All rights reserved.
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

use gitea_rs::Client;

mod ssh_fixtures;

use ssh_fixtures::{
    ed25519_private_key, ed25519_private_key_bytes, rsa_passphrase,
    rsa_passphrase_private_key_bytes,
};

fn ed25519_tmp_key(test_name: &str) -> std::path::PathBuf {
    let tmp = std::env::temp_dir().join(format!("gitea_rs_{test_name}"));
    std::fs::write(&tmp, ed25519_private_key_bytes()).expect("write temp ed25519 key");
    tmp
}

fn ed25519_fingerprint() -> String {
    let key = ed25519_private_key().expect("ed25519 test key");
    key.public_key()
        .fingerprint(ssh_key::HashAlg::Sha256)
        .to_string()
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
        gitea_rs::Error::SshSign(msg) => {
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
    let tmp = std::env::temp_dir().join("gitea_rs_ssh_signing_passphrase");
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
        gitea_rs::Error::SshSign(msg) => {
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
fn test_ssh_cert_wrong_passphrase() {
    let tmp = std::env::temp_dir().join("gitea_rs_ssh_signing_cert_pass");
    std::fs::write(&tmp, rsa_passphrase_private_key_bytes())
        .expect("write passphrase-protected key");

    let result =
        Client::builder("https://example.com").ssh_cert("principal", &tmp, Some::<&str>("wrong"));

    assert!(
        result.is_err(),
        "ssh_cert with wrong passphrase should return Err"
    );
    match result.unwrap_err() {
        gitea_rs::Error::SshSign(msg) => {
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
    let tmp = std::env::temp_dir().join("gitea_rs_ssh_signing_correct_pass");
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
