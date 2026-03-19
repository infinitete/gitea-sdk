// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Shared HTTP request helpers for API modules.

pub(crate) fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

pub(crate) fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

pub(crate) fn urlencoding(value: &str) -> String {
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
    utf8_percent_encode(value, NON_ALPHANUMERIC).to_string()
}
