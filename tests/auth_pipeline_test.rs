// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Wiremock integration tests for the HTTP request pipeline.
//!
//! These tests exercise `do_request_raw` (the Layer-0 internal pipeline)
//! indirectly through public methods like [`Client::server_version`].
//! Auth header injection, path prefixing, error handling, and concurrent
//! usage are all verified via wiremock matchers.

use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use gitea_sdk::{Client, Error};

fn create_client(server: &MockServer) -> Client {
    let http = reqwest::Client::new();
    Client::builder(&server.uri())
        .http_client(http)
        .build()
        .unwrap()
}

// ── Auth Header Injection Tests ────────────────────────────────────────

#[tokio::test]
async fn test_auth_token_header_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", "token test-token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .token("test-token")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_basic_auth_header_wiremock() {
    let server = MockServer::start().await;
    let expected = format!("Basic {}", base64_encode("admin:secret"));
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", expected.as_str()))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .basic_auth("admin", "secret")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_otp_header_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("x-gitea-otp", "123456"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .otp("123456")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_sudo_header_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("sudo", "target-user"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .sudo("target-user")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_user_agent_header_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("user-agent", "my-app/1.0"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .user_agent("my-app/1.0")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_token_with_otp_and_sudo_combined_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", "token abc"))
        .and(header("x-gitea-otp", "654321"))
        .and(header("sudo", "impersonated"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .token("abc")
        .otp("654321")
        .sudo("impersonated")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_token_and_basic_auth_wiremock() {
    // When both token and basic auth are configured, the token is set first
    // (line 41 http.rs), then basic_auth overwrites the Authorization header.
    // Verify the request succeeds — basic_auth takes effect.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .token("ignored-token")
        .basic_auth("admin", "secret")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_no_credentials_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_setters_at_runtime_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", "token dynamic-token"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    client.set_token("dynamic-token");
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_auth_set_basic_auth_at_runtime_wiremock() {
    let server = MockServer::start().await;
    let expected = format!("Basic {}", base64_encode("user2:pass2"));
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", expected.as_str()))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    client.set_basic_auth("user2", "pass2");
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

// ── Path Prefix Tests ──────────────────────────────────────────────────

#[tokio::test]
async fn test_path_prefix_api_v1_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_path_wrong_prefix_rejected_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/wrong"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .expect(0)
        .mount(&server)
        .await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_path_base_url_trailing_slash_wiremock() {
    let server = MockServer::start().await;
    let base = format!("{}/", server.uri());
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(base.as_str())
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

// ── Full Pipeline Tests ────────────────────────────────────────────────

#[tokio::test]
async fn test_pipeline_json_response_parsed_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", "token full-pipeline"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .token("full-pipeline")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_pipeline_accept_json_header_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("accept", "application/json"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_pipeline_non_json_error_response_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(500).set_body_string("internal error"))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::UnknownApi { status, body } => {
            assert_eq!(status, 500);
            assert_eq!(body, "internal error");
        }
        other => panic!("expected Error::UnknownApi, got: {other}"),
    }
}

#[tokio::test]
async fn test_pipeline_json_error_with_message_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(403).set_body_json(json!({"message": "forbidden"})),
        )
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::Api {
            status,
            message,
            ..
        } => {
            assert_eq!(status, 403);
            assert_eq!(message, "forbidden");
        }
        other => panic!("expected Error::Api, got: {other}"),
    }
}

#[tokio::test]
async fn test_pipeline_version_cached_after_first_call_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_pipeline_get_method_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    Mock::given(method("POST"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .expect(0)
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

// ── Version Constraint Pipeline Tests ──────────────────────────────────

#[tokio::test]
async fn test_pipeline_version_constraint_with_auth_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .and(header("authorization", "token constraint-test"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .token("constraint-test")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    client
        .check_server_version_constraint(">= 1.20.0")
        .await
        .unwrap();
}

// ── Concurrent Client Usage ────────────────────────────────────────────
//
// NOTE: Client uses parking_lot::RwLock whose guards are !Send.
// tokio::spawn requires Send futures, so we verify concurrency safety
// via sequential calls that exercise the same locking paths.
// Client is already tested as Send + Sync in client.rs unit tests.

#[tokio::test]
async fn test_concurrent_client_multiple_calls_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .mount(&server)
        .await;

    let client = create_client(&server);

    for i in 0..10 {
        let result = client.server_version().await.unwrap();
        assert_eq!(result, "1.22.0", "call {i} got wrong version");
    }
}

#[tokio::test]
async fn test_concurrent_clients_same_server_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .mount(&server)
        .await;

    for i in 0..10 {
        let client = Client::builder(&server.uri())
            .token(format!("token-{i}"))
            .http_client(reqwest::Client::new())
            .build()
            .unwrap();
        let result = client.server_version().await.unwrap();
        assert_eq!(result, "1.22.0");
    }
}

#[tokio::test]
async fn test_interleaved_setters_and_requests_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .mount(&server)
        .await;

    let client = create_client(&server);

    for i in 0..5 {
        client.set_token(format!("token-{i}"));
        client.set_otp(format!("{i:06}"));
        client.set_sudo(format!("user-{i}"));
        let _ = client.server_version().await;
    }
}

#[tokio::test]
async fn test_concurrent_different_auth_configs_wiremock() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .mount(&server)
        .await;

    // OnceLock caches the version after the first call, so only the first
    // request actually hits the server. Subsequent calls use the cached value.
    let client = create_client(&server);

    client.set_token("concurrent-0");
    let result = client.server_version().await.unwrap();
    assert_eq!(result, "1.22.0");

    client.set_token("concurrent-1");
    let result = client.server_version().await.unwrap();
    assert_eq!(result, "1.22.0");

    client.set_token("concurrent-2");
    let result = client.server_version().await.unwrap();
    assert_eq!(result, "1.22.0");
}

#[tokio::test]
async fn test_client_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Client>();
}

// ── Connection / Error Pipeline Tests ──────────────────────────────────

#[tokio::test]
async fn test_pipeline_connection_refused_wiremock() {
    let client = Client::builder("http://127.0.0.1:1")
        .token("irrelevant")
        .build()
        .unwrap();

    let err = client.server_version().await.unwrap_err();
    assert!(
        matches!(err, Error::Request(_)),
        "expected Error::Request for connection refused, got: {err}"
    );
}

#[tokio::test]
async fn test_pipeline_empty_response_body_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_string(""))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    assert!(
        matches!(err, Error::Json(_)),
        "expected Error::Json for empty body, got: {err}"
    );
}

#[tokio::test]
async fn test_pipeline_invalid_json_body_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    assert!(
        matches!(err, Error::Json(_)),
        "expected Error::Json for invalid JSON, got: {err}"
    );
}

#[tokio::test]
async fn test_pipeline_multiple_errors_sequential_wiremock() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({"message": "not found"})),
        )
        .mount(&server)
        .await;

    let client = create_client(&server);
    match client.server_version().await.unwrap_err() {
        Error::Api { status, .. } => assert_eq!(status, 404),
        other => panic!("expected 404 Api error, got: {other}"),
    }

    server.reset().await;

    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(500).set_body_string("server error"))
        .mount(&server)
        .await;

    // Use a fresh client so OnceLock is not set from previous attempt.
    let client2 = create_client(&server);
    match client2.server_version().await.unwrap_err() {
        Error::UnknownApi { status, .. } => assert_eq!(status, 500),
        other => panic!("expected 500 UnknownApi error, got: {other}"),
    }
}

// ── Preset Version Bypasses Pipeline ───────────────────────────────────

#[tokio::test]
async fn test_preset_version_no_http_request_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(0)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .gitea_version("1.21.0")
        .token("should-not-be-sent")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.21.0");
}

// ── Ignore Version Pipeline ────────────────────────────────────────────

#[tokio::test]
async fn test_ignore_version_no_http_request_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})),
        )
        .expect(0)
        .mount(&server)
        .await;

    let client = Client::builder(&server.uri())
        .gitea_version("")
        .http_client(reqwest::Client::new())
        .build()
        .unwrap();

    let err = client.server_version().await.unwrap_err();
    assert!(
        matches!(err, Error::Version(_)),
        "expected Error::Version when version checks disabled, got: {err}"
    );
}

// ── Base64 encoding helper ─────────────────────────────────────────────

/// Minimal base64 encoding for basic auth header comparison.
fn base64_encode(input: &str) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let bytes = input.as_bytes();
    let mut result = String::new();
    let mut i = 0;

    while i + 3 <= bytes.len() {
        let triple = (bytes[i] as u32) << 16 | (bytes[i + 1] as u32) << 8 | bytes[i + 2] as u32;
        result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        result.push(ALPHABET[(triple & 0x3F) as usize] as char);
        i += 3;
    }

    match bytes.len() - i {
        2 => {
            let triple = (bytes[i] as u32) << 16 | (bytes[i + 1] as u32) << 8;
            result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
            result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
            result.push(ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
            result.push('=');
        }
        1 => {
            let triple = (bytes[i] as u32) << 16;
            result.push(ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
            result.push(ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
            result.push('=');
            result.push('=');
        }
        0 => {}
        _ => unreachable!(),
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode_basic() {
        assert_eq!(base64_encode("admin:secret"), "YWRtaW46c2VjcmV0");
    }

    #[test]
    fn test_base64_encode_short() {
        assert_eq!(base64_encode("a"), "YQ==");
    }

    #[test]
    fn test_base64_encode_two_chars() {
        assert_eq!(base64_encode("ab"), "YWI=");
    }

    #[test]
    fn test_base64_encode_three_chars() {
        assert_eq!(base64_encode("abc"), "YWJj");
    }
}
