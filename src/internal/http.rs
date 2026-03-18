// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use reqwest::header::HeaderMap;

use crate::Client;
use crate::Response;
use crate::response::response_from_reqwest;

// ── HTTP request pipeline (mirrors go-sdk/gitea/client.go) ─────────

#[derive(Clone)]
struct RequestPipelineConfig {
    base_url: String,
    access_token: String,
    otp: String,
    username: String,
    password: String,
    sudo: String,
    user_agent: String,
    debug: bool,
}

impl From<crate::client::ClientConfig> for RequestPipelineConfig {
    fn from(config: crate::client::ClientConfig) -> Self {
        Self {
            base_url: config.base_url,
            access_token: config.access_token,
            otp: config.otp,
            username: config.username,
            password: config.password,
            sudo: config.sudo,
            user_agent: config.user_agent,
            debug: config.debug,
        }
    }
}

enum RequestPayload {
    Empty,
    Body(reqwest::Body),
    Multipart(reqwest::multipart::Form),
}

fn build_pipeline_request(
    http_client: &reqwest::Client,
    config: &RequestPipelineConfig,
    method: reqwest::Method,
    path: &str,
    headers: Option<&HeaderMap>,
    payload: RequestPayload,
) -> crate::Result<reqwest::Request> {
    let url = format!("{}/api/v1{path}", config.base_url);

    let mut req = http_client
        .request(method, &url)
        .header("Accept", "application/json");

    // Auth header injection — exact order from Go SDK doRequest:
    // 1. Token → Authorization: token {access_token}
    if !config.access_token.is_empty() {
        req = req.header("Authorization", format!("token {}", config.access_token));
    }
    // 2. OTP → X-GITEA-OTP
    if !config.otp.is_empty() {
        req = req.header("X-GITEA-OTP", &config.otp);
    }
    // 3. Basic Auth
    if !config.username.is_empty() {
        req = req.basic_auth(&config.username, Some(&config.password));
    }
    // 4. Sudo
    if !config.sudo.is_empty() {
        req = req.header("Sudo", &config.sudo);
    }
    // 5. User-Agent
    if !config.user_agent.is_empty() {
        req = req.header("User-Agent", &config.user_agent);
    }

    if let Some(hdrs) = headers {
        for (k, v) in hdrs {
            req = req.header(k, v);
        }
    }

    req = match payload {
        RequestPayload::Empty => req,
        RequestPayload::Body(body) => req.body(body),
        RequestPayload::Multipart(form) => req.multipart(form),
    };

    Ok(req.build()?)
}

impl Client {
    async fn do_request_raw_with_payload(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        payload: RequestPayload,
    ) -> crate::Result<reqwest::Response> {
        let http_client = self.http_client();
        let request_config = RequestPipelineConfig::from(self.config_snapshot());
        let mut built_req = build_pipeline_request(
            &http_client,
            &request_config,
            method,
            path,
            headers,
            payload,
        )?;

        if request_config.debug {
            tracing::debug!("{}: {}", built_req.method(), built_req.url());
        }

        {
            // sign_request() is synchronous; do NOT add .await inside this scope
            let signer = self.ssh_signer();
            if let Some(ref signer) = *signer {
                let use_legacy = self.should_use_legacy_ssh();
                crate::auth::ssh_sign::sign_request(&mut built_req, signer, use_legacy)?;
            }
        }

        let resp = http_client.execute(built_req).await?;
        Ok(resp)
    }

    /// Layer 0 (internal): Build and send a request, returning the raw
    /// `reqwest::Response` so that higher layers can decide how to consume
    /// the body.
    ///
    /// Auth header injection order matches Go SDK `doRequest` exactly:
    /// token → OTP → basic auth → sudo → user-agent → caller headers.
    async fn do_request_raw<B: Into<reqwest::Body>>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        body: Option<B>,
    ) -> crate::Result<reqwest::Response> {
        let payload = match body {
            Some(value) => RequestPayload::Body(value.into()),
            None => RequestPayload::Empty,
        };
        self.do_request_raw_with_payload(method, path, headers, payload)
            .await
    }

    /// Layer 1: Status check only, discards the body.
    ///
    /// Used for DELETE and other operations where only the status code matters.
    /// Matches Go SDK `doRequestWithStatusHandle`.
    #[allow(dead_code)]
    pub(crate) async fn do_request_with_status_handle<B: Into<reqwest::Body>>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        body: Option<B>,
    ) -> crate::Result<Response> {
        let resp = self.do_request_raw(method, path, headers, body).await?;
        let response = response_from_reqwest(&resp);

        // Check for errors — reads and discards the body on error.
        let status = resp.status().as_u16();
        if status / 100 != 2 {
            let err_bytes = resp.bytes().await.unwrap_or_default();
            status_code_to_err(status, &err_bytes)?;
        }

        Ok(response)
    }

    /// Layer 2: Return status code without checking for errors.
    ///
    /// Matches Go SDK `getStatusCode`.
    #[allow(dead_code)]
    pub(crate) async fn get_status_code<B: Into<reqwest::Body>>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        body: Option<B>,
    ) -> crate::Result<(u16, Response)> {
        let resp = self.do_request_raw(method, path, headers, body).await?;
        let response = response_from_reqwest(&resp);
        let status = resp.status().as_u16();
        Ok((status, response))
    }

    /// Layer 3: Read response body and check for errors.
    ///
    /// Returns `(body_bytes, Response)` on success (2xx).
    /// Matches Go SDK `getResponse`.
    pub(crate) async fn get_response<B: Into<reqwest::Body>>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        body: Option<B>,
    ) -> crate::Result<(Vec<u8>, Response)> {
        let resp = self.do_request_raw(method, path, headers, body).await?;
        let response = response_from_reqwest(&resp);
        let status = resp.status().as_u16();

        if status / 100 != 2 {
            let err_bytes = resp.bytes().await.unwrap_or_default();
            status_code_to_err(status, &err_bytes)?;
            // Unreachable: status_code_to_err returns Err for non-2xx.
            unreachable!()
        }

        let data = resp.bytes().await?.to_vec();
        Ok((data, response))
    }

    /// Layer 4: Read response body, check for errors, and deserialize JSON.
    ///
    /// Returns `(T, Response)` on success.
    /// Matches Go SDK `getParsedResponse`.
    #[allow(dead_code)]
    pub(crate) async fn get_parsed_response<
        T: serde::de::DeserializeOwned,
        B: Into<reqwest::Body>,
    >(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        body: Option<B>,
    ) -> crate::Result<(T, Response)> {
        let (data, response) = self.get_response(method, path, headers, body).await?;
        let value: T = serde_json::from_slice(&data)?;
        Ok((value, response))
    }

    /// Layer 5: Send a multipart request, check for errors, and deserialize JSON.
    ///
    /// Returns `(T, Response)` on success.
    /// Used for file upload endpoints (e.g. release attachments).
    pub(crate) async fn get_parsed_response_multipart<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        headers: Option<&HeaderMap>,
        form: reqwest::multipart::Form,
    ) -> crate::Result<(T, Response)> {
        let resp = self
            .do_request_raw_with_payload(method, path, headers, RequestPayload::Multipart(form))
            .await?;
        let response = response_from_reqwest(&resp);
        let status = resp.status().as_u16();

        if status / 100 != 2 {
            let err_bytes = resp.bytes().await.unwrap_or_default();
            status_code_to_err(status, &err_bytes)?;
            unreachable!()
        }

        let data = resp.bytes().await?.to_vec();
        let value: T = serde_json::from_slice(&data)?;
        Ok((value, response))
    }
}

// ── Error mapping (pure function, no Client dependency) ─────────────

/// Convert an HTTP status code and response body into an appropriate
/// [`crate::Error`] variant.
///
/// Returns `Ok(())` for 2xx status codes. For non-2xx:
/// - If the body is valid JSON with a `"message"` field → [`Error::Api`](crate::Error::Api)
/// - Otherwise → [`Error::UnknownApi`](crate::Error::UnknownApi)
///
/// Matches Go SDK `statusCodeToErr`.
fn status_code_to_err(status: u16, body: &[u8]) -> crate::Result<()> {
    if status / 100 == 2 {
        return Ok(());
    }

    if let Ok(err_map) = serde_json::from_slice::<serde_json::Value>(body)
        && let Some(message) = err_map.get("message").and_then(|v| v.as_str())
    {
        return Err(crate::Error::Api {
            status,
            message: message.to_string(),
            body: body.to_vec(),
        });
    }

    Err(crate::Error::UnknownApi {
        status,
        body: String::from_utf8_lossy(body).to_string(),
    })
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_to_err_success() {
        assert!(status_code_to_err(200, b"").is_ok());
        assert!(status_code_to_err(201, b"created").is_ok());
        assert!(status_code_to_err(299, b"").is_ok());
    }

    #[test]
    fn test_status_code_to_err_api_error() {
        let body = br#"{"message":"Not Found"}"#;
        let err = status_code_to_err(404, body).unwrap_err();
        match err {
            crate::Error::Api {
                status,
                message,
                body: err_body,
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
                assert_eq!(err_body, body.as_slice());
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
    }

    #[test]
    fn test_status_code_to_err_unknown_api() {
        let body = b"Internal Server Error";
        let err = status_code_to_err(500, body).unwrap_err();
        match err {
            crate::Error::UnknownApi {
                status,
                body: err_body,
            } => {
                assert_eq!(status, 500);
                assert_eq!(err_body, "Internal Server Error");
            }
            other => panic!("expected Error::UnknownApi, got: {other}"),
        }
    }

    #[test]
    fn test_status_code_to_err_json_no_message() {
        let body = br#"{"error":"bad request"}"#;
        let err = status_code_to_err(400, body).unwrap_err();
        match err {
            crate::Error::UnknownApi {
                status,
                body: err_body,
            } => {
                assert_eq!(status, 400);
                assert_eq!(err_body, r#"{"error":"bad request"}"#);
            }
            other => panic!("expected Error::UnknownApi, got: {other}"),
        }
    }

    #[test]
    fn test_status_code_to_err_empty_body() {
        let body = b"";
        let err = status_code_to_err(500, body).unwrap_err();
        match err {
            crate::Error::UnknownApi {
                status,
                body: err_body,
            } => {
                assert_eq!(status, 500);
                assert!(err_body.is_empty());
            }
            other => panic!("expected Error::UnknownApi, got: {other}"),
        }
    }

    #[test]
    fn test_status_code_to_err_array_body() {
        // Valid JSON array but not an object with "message".
        let body = b"[]";
        let err = status_code_to_err(500, body).unwrap_err();
        match err {
            crate::Error::UnknownApi { status, .. } => {
                assert_eq!(status, 500);
            }
            other => panic!("expected Error::UnknownApi, got: {other}"),
        }
    }

    #[test]
    fn test_status_code_to_err_message_with_number() {
        // "message" is not a string — should fall through to UnknownApi.
        let body = br#"{"message":42}"#;
        let err = status_code_to_err(422, body).unwrap_err();
        assert!(
            matches!(err, crate::Error::UnknownApi { .. }),
            "expected Error::UnknownApi when message is not a string, got: {err}"
        );
    }

    #[tokio::test]
    async fn test_do_request_raw_signs_when_ssh_signer_present() {
        use wiremock::matchers::{header_exists, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .and(header_exists("Signature"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
            )
            .mount(&server)
            .await;

        let tmp = std::env::temp_dir().join("gitea_sdk_test_ssh_wiremock_sign");
        std::fs::write(
            &tmp,
            include_bytes!("../../tests/ssh_fixtures/id_ed25519_test"),
        )
        .expect("write temp key");

        let client = crate::Client::builder(&server.uri())
            .ssh_cert("test-principal", &tmp, None::<&str>)
            .expect("ssh_cert should succeed")
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
    async fn test_do_request_raw_no_signature_when_no_ssh_signer() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/v1/version"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"version": "1.22.0"})),
            )
            .mount(&server)
            .await;

        let client = crate::Client::builder(&server.uri())
            .build()
            .expect("build should succeed");

        let (version, _resp) = client
            .miscellaneous()
            .get_version()
            .await
            .expect("get_version should succeed");
        assert_eq!(version, "1.22.0");
    }

    #[test]
    fn test_request_pipeline_builder_applies_common_headers_for_body_and_multipart() {
        let http_client = reqwest::Client::new();
        let mut extra_headers = HeaderMap::new();
        extra_headers.insert("X-Test-Header", "from-caller".parse().unwrap());

        let body_request = build_pipeline_request(
            &http_client,
            &RequestPipelineConfig {
                base_url: "https://example.com".to_string(),
                access_token: "token-123".to_string(),
                otp: "654321".to_string(),
                username: "".to_string(),
                password: "".to_string(),
                sudo: "sudo-user".to_string(),
                user_agent: "gitea-rust-sdk/test".to_string(),
                debug: false,
            },
            reqwest::Method::POST,
            "/repos/owner/repo",
            Some(&extra_headers),
            RequestPayload::Body(reqwest::Body::from("payload")),
        )
        .expect("body request should build");

        let multipart_request = build_pipeline_request(
            &http_client,
            &RequestPipelineConfig {
                base_url: "https://example.com".to_string(),
                access_token: "token-123".to_string(),
                otp: "654321".to_string(),
                username: "".to_string(),
                password: "".to_string(),
                sudo: "sudo-user".to_string(),
                user_agent: "gitea-rust-sdk/test".to_string(),
                debug: false,
            },
            reqwest::Method::POST,
            "/repos/owner/repo/avatar",
            Some(&extra_headers),
            RequestPayload::Multipart(reqwest::multipart::Form::new().text("name", "avatar")),
        )
        .expect("multipart request should build");

        for request in [&body_request, &multipart_request] {
            assert_eq!(request.headers()["accept"], "application/json");
            assert_eq!(request.headers()["authorization"], "token token-123");
            assert_eq!(request.headers()["x-gitea-otp"], "654321");
            assert_eq!(request.headers()["sudo"], "sudo-user");
            assert_eq!(request.headers()["user-agent"], "gitea-rust-sdk/test");
            assert_eq!(request.headers()["x-test-header"], "from-caller");
        }
    }
}
