// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

/// Build a full URL path with optional query string.
///
/// When `query` is non-empty, returns `base_path?query`.
/// When `query` is empty, returns `base_path` unchanged.
#[allow(dead_code)]
pub(crate) fn build_query_string(base_path: &str, query: &str) -> String {
    if query.is_empty() {
        base_path.to_string()
    } else {
        format!("{base_path}?{query}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_string_with_params() {
        assert_eq!(
            build_query_string("/repos", "page=1&limit=20"),
            "/repos?page=1&limit=20"
        );
    }

    #[test]
    fn test_query_string_empty() {
        assert_eq!(build_query_string("/repos", ""), "/repos");
    }

    #[test]
    fn test_query_string_base_empty() {
        assert_eq!(build_query_string("", "page=1"), "?page=1");
    }
}
