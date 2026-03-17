// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use reqwest::header::HeaderMap;

use crate::response::response_from_reqwest;
use crate::Client;
use crate::Response;

// ── HTTP request pipeline (mirrors go-sdk/gitea/client.go) ─────────

impl Client {
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
        let config = self.read_config();
        let base_url = &config.base_url;
        let debug = config.debug;

        let url = format!("{base_url}/api/v1{path}");

        let mut req = self
            .http_client()
            .request(method.clone(), &url)
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
        drop(config); // Release read lock before sending request.

        if let Some(hdrs) = headers {
            for (k, v) in hdrs.iter() {
                req = req.header(k, v);
            }
        }

        if let Some(b) = body {
            req = req.body(b);
        }

        if debug {
            tracing::debug!("{}: {}", method, url);
        }

        // SSH signing placeholder — Phase 1c.
        let resp = self.http_client().execute(req.build()?).await?;
        Ok(resp)
    }

    /// Layer 1: Status check only, discards the body.
    ///
    /// Used for DELETE and other operations where only the status code matters.
    /// Matches Go SDK `doRequestWithStatusHandle`.
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
    pub(crate) async fn get_parsed_response<T: serde::de::DeserializeOwned, B: Into<reqwest::Body>>(
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

    if let Ok(err_map) = serde_json::from_slice::<serde_json::Value>(body) {
        if let Some(message) = err_map.get("message").and_then(|v| v.as_str()) {
            return Err(crate::Error::Api {
                status,
                message: message.to_string(),
                body: body.to_vec(),
            });
        }
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
            crate::Error::UnknownApi { status, body: err_body } => {
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
            crate::Error::UnknownApi { status, body: err_body } => {
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
            crate::Error::UnknownApi { status, body: err_body } => {
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
}
