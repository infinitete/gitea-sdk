// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Error types produced by the Gitea SDK.

use thiserror::Error;

/// Errors produced by the Gitea SDK.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// API returned a structured error with a message.
    #[error("API error {status}: {message}")]
    Api {
        /// HTTP status code.
        status: u16,
        /// Error message from the API.
        message: String,
        /// Raw response body.
        body: Vec<u8>,
    },

    /// API returned a non-2xx status without a structured error.
    #[error("unknown API error {status}: {body}")]
    UnknownApi {
        /// HTTP status code.
        status: u16,
        /// Raw response body as string.
        body: String,
    },

    /// Input validation failed.
    #[error("validation error: {0}")]
    Validation(String),

    /// Version-related error.
    #[error("version error: {0}")]
    Version(String),

    /// Server returned an unrecognized version string.
    #[error("unknown version: {0}")]
    UnknownVersion(String),

    /// SSH signing error.
    #[error("SSH signing error: {0}")]
    SshSign(String),

    /// JSON serialization or deserialization failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing failed.
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
}

/// A type alias for `Result` with the SDK's [`Error`](enum@Error) type.
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_implements_std_error() {
        fn assert_error<E: std::error::Error>(_: &E) {}
        assert_error(&Error::Validation("test".to_string()));
    }

    #[test]
    fn error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>(_: &T) {}
        assert_send_sync(&Error::Validation("test".to_string()));
    }

    #[test]
    fn result_alias_works() {
        fn returns_result() -> Result<i32> {
            Ok(42)
        }
        assert_eq!(returns_result().unwrap(), 42);

        fn returns_error() -> Result<i32> {
            Err(Error::Validation("bad".to_string()))
        }
        assert_eq!(
            returns_error().unwrap_err().to_string(),
            "validation error: bad"
        );
    }

    #[test]
    fn display_api() {
        let err = Error::Api {
            status: 404,
            message: "not found".to_string(),
            body: b"error details".to_vec(),
        };
        assert_eq!(err.to_string(), "API error 404: not found");
    }

    #[test]
    fn display_unknown_api() {
        let err = Error::UnknownApi {
            status: 500,
            body: "internal error".to_string(),
        };
        assert_eq!(err.to_string(), "unknown API error 500: internal error");
    }

    #[test]
    fn display_validation() {
        let err = Error::Validation("owner is empty".to_string());
        assert_eq!(err.to_string(), "validation error: owner is empty");
    }

    #[test]
    fn display_version() {
        let err = Error::Version("unsupported version".to_string());
        assert_eq!(err.to_string(), "version error: unsupported version");
    }

    #[test]
    fn display_unknown_version() {
        let err = Error::UnknownVersion("1.99.0".to_string());
        assert_eq!(err.to_string(), "unknown version: 1.99.0");
    }

    #[test]
    fn display_ssh_sign() {
        let err = Error::SshSign("signing failed".to_string());
        assert_eq!(err.to_string(), "SSH signing error: signing failed");
    }

    #[test]
    fn display_json() {
        let err: Error = serde_json::from_str::<serde_json::Value>("invalid json")
            .unwrap_err()
            .into();
        assert!(err.to_string().contains("JSON error"));
    }

    #[tokio::test]
    async fn display_request() {
        let req_err = reqwest::Client::builder()
            .build()
            .unwrap()
            .get("http://0.0.0.0:1")
            .send()
            .await
            .expect_err("should fail to connect");
        let err = Error::from(req_err);
        assert!(err.to_string().starts_with("HTTP request failed"));
    }
}
