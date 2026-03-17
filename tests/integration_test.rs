// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use serde_json::json;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use gitea_sdk::{Client, Error, ListOptions, PageLinks, QueryEncode, Response};

fn create_client(server: &MockServer) -> Client {
    let http = reqwest::Client::new();
    Client::builder(&server.uri())
        .http_client(http)
        .build()
        .unwrap()
}

// ── Version Detection ──────────────────────────────────────────────

#[tokio::test]
async fn test_version_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_version_with_v_prefix_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "v1.21.3"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.21.3");
}

#[tokio::test]
async fn test_version_cached_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

#[tokio::test]
async fn test_version_preset_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.99.0"})))
        .expect(0)
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = Client::builder(&server.uri())
        .gitea_version("1.22.0")
        .http_client(http)
        .build()
        .unwrap();
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

// ── Version Constraint Checks ──────────────────────────────────────

#[tokio::test]
async fn test_version_constraint_passes_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    client
        .check_server_version_constraint(">= 1.11.0")
        .await
        .unwrap();
}

#[tokio::test]
async fn test_version_constraint_fails_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.10.5"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client
        .check_server_version_constraint(">= 1.11.0")
        .await
        .unwrap_err();
    match err {
        Error::Version(msg) => {
            assert!(msg.contains("1.10.5"), "expected version in message: {msg}");
            assert!(
                msg.contains(">= 1.11.0"),
                "expected constraint in message: {msg}"
            );
        }
        other => panic!("expected Error::Version, got: {other}"),
    }
}

#[tokio::test]
async fn test_version_unknown_format_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"version": "unknown-format"})),
        )
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::UnknownVersion(v) => assert_eq!(v, "unknown-format"),
        other => panic!("expected Error::UnknownVersion, got: {other}"),
    }
}

#[tokio::test]
async fn test_version_ignore_version_returns_error() {
    let client = Client::builder("https://localhost:1")
        .gitea_version("")
        .build()
        .unwrap();
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::Version(msg) => {
            assert!(
                msg.contains("disabled"),
                "expected 'disabled' in message: {msg}"
            );
        }
        other => panic!("expected Error::Version, got: {other}"),
    }
}

#[tokio::test]
async fn test_version_invalid_constraint_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client
        .check_server_version_constraint("not a valid constraint")
        .await
        .unwrap_err();
    match err {
        Error::Version(msg) => {
            assert!(
                msg.contains("invalid constraint"),
                "expected 'invalid constraint': {msg}"
            );
        }
        other => panic!("expected Error::Version, got: {other}"),
    }
}

// ── Error Handling ─────────────────────────────────────────────────

#[tokio::test]
async fn test_error_api_json_message_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "Not Found"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::Api {
            status,
            message,
            body,
        } => {
            assert_eq!(status, 404);
            assert_eq!(message, "Not Found");
            let parsed: serde_json::Value = serde_json::from_slice(&body).unwrap();
            assert_eq!(parsed["message"], "Not Found");
        }
        other => panic!("expected Error::Api, got: {other}"),
    }
}

#[tokio::test]
async fn test_error_unknown_api_non_json_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(500).set_body_string("internal server error"))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::UnknownApi { status, body } => {
            assert_eq!(status, 500);
            assert_eq!(body, "internal server error");
        }
        other => panic!("expected Error::UnknownApi, got: {other}"),
    }
}

#[tokio::test]
async fn test_error_json_no_message_field_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"error": "bad request"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::UnknownApi { status, body } => {
            assert_eq!(status, 400);
            assert!(body.contains("bad request"));
        }
        other => panic!("expected Error::UnknownApi, got: {other}"),
    }
}

#[tokio::test]
async fn test_error_empty_body_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(503).set_body_string(""))
        .mount(&server)
        .await;

    let client = create_client(&server);
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::UnknownApi { status, body } => {
            assert_eq!(status, 503);
            assert!(body.is_empty());
        }
        other => panic!("expected Error::UnknownApi, got: {other}"),
    }
}

#[tokio::test]
async fn test_success_parse_json_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.22.0"})))
        .mount(&server)
        .await;

    let client = create_client(&server);
    assert_eq!(client.server_version().await.unwrap(), "1.22.0");
}

// ── Link Header ────────────────────────────────────────────────────

#[test]
fn test_link_header_empty_response() {
    let resp = Response {
        status: 200,
        headers: reqwest::header::HeaderMap::new(),
        page_links: None,
    };
    assert!(resp.page_links.is_none());
}

#[test]
fn test_page_links_equality() {
    let a = PageLinks {
        first: Some(1),
        prev: None,
        next: Some(2),
        last: Some(5),
    };
    let b = PageLinks {
        first: Some(1),
        prev: None,
        next: Some(2),
        last: Some(5),
    };
    assert_eq!(a, b);
}

#[test]
fn test_page_links_clone() {
    let links = PageLinks {
        first: Some(1),
        prev: Some(2),
        next: Some(3),
        last: Some(10),
    };
    let cloned = links.clone();
    assert_eq!(links, cloned);
    assert_eq!(cloned.first, Some(1));
    assert_eq!(cloned.last, Some(10));
}

// ── Pagination ─────────────────────────────────────────────────────

#[test]
fn test_pagination_query_encode_default() {
    assert_eq!(ListOptions::default().query_encode(), "page=1");
}

#[test]
fn test_pagination_query_encode_with_page_size() {
    let opts = ListOptions {
        page: Some(2),
        page_size: Some(50),
    };
    assert_eq!(opts.query_encode(), "page=2&limit=50");
}

#[test]
fn test_pagination_query_encode_disable_pagination() {
    let opts = ListOptions {
        page: Some(-1),
        page_size: Some(0),
    };
    assert_eq!(opts.query_encode(), "page=0&limit=0");
}

#[test]
fn test_pagination_with_defaults_idempotent() {
    let once = ListOptions::default().with_defaults();
    let twice = once.with_defaults();
    assert_eq!(once, twice);
}

#[test]
fn test_pagination_with_defaults_negative_page() {
    let opts = ListOptions {
        page: Some(-1),
        page_size: Some(10),
    };
    let defaulted = opts.with_defaults();
    assert_eq!(defaulted.page, Some(0));
    assert_eq!(defaulted.page_size, Some(10));
}

// ── Client Builder ─────────────────────────────────────────────────

#[tokio::test]
async fn test_client_builder_with_token_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.20.0"})))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = Client::builder(&server.uri())
        .token("test-token")
        .http_client(http)
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.20.0");
}

#[tokio::test]
async fn test_client_builder_with_basic_auth_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.18.0"})))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = Client::builder(&server.uri())
        .basic_auth("admin", "secret")
        .http_client(http)
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.18.0");
}

#[tokio::test]
async fn test_client_set_http_client_wiremock() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/v1/version"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"version": "1.15.0"})))
        .mount(&server)
        .await;

    let http = reqwest::Client::new();
    let client = Client::builder(&server.uri())
        .http_client(http)
        .build()
        .unwrap();

    assert_eq!(client.server_version().await.unwrap(), "1.15.0");
}

// ── Connection Errors ──────────────────────────────────────────────

#[tokio::test]
async fn test_connection_error_returns_request_error() {
    let client = Client::builder("http://localhost:1").build().unwrap();
    let err = client.server_version().await.unwrap_err();
    match err {
        Error::Request(e) => {
            assert!(
                !e.to_string().is_empty(),
                "expected request error, got: {e}"
            );
        }
        other => panic!("expected Error::Request, got: {other}"),
    }
}
