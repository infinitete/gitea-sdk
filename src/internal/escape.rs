// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Path segment validation and escaping.

use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};

#[allow(dead_code)]
pub(crate) fn path_escape_segments(path: &str) -> String {
    path.split('/')
        .map(|seg| utf8_percent_encode(seg, NON_ALPHANUMERIC).to_string())
        .collect::<Vec<_>>()
        .join("/")
}

pub(crate) fn validate_and_escape_segments(segments: &[&str]) -> crate::Result<Vec<String>> {
    segments
        .iter()
        .enumerate()
        .map(|(i, &segment)| {
            if segment.is_empty() {
                return Err(crate::Error::Validation(format!(
                    "path segment [{}] is empty",
                    i
                )));
            }
            if segment.contains('\0') {
                return Err(crate::Error::Validation(format!(
                    "path segment [{}] contains null byte",
                    i
                )));
            }
            // Block path traversal patterns before encoding.
            // After percent-encoding, `..` becomes `%2E%2E` which is safe.
            // But we reject `..` before encoding to be explicit about intent.
            let trimmed = segment.trim_end();
            if trimmed == ".." || trimmed.starts_with("../") || trimmed.contains("/..") {
                return Err(crate::Error::Validation(format!(
                    "path segment [{}] contains path traversal pattern",
                    i
                )));
            }
            if segment.len() > 4096 {
                return Err(crate::Error::Validation(format!(
                    "path segment [{}] exceeds maximum length of 4096",
                    i
                )));
            }
            Ok(utf8_percent_encode(segment, NON_ALPHANUMERIC).to_string())
        })
        .collect()
}

#[allow(dead_code)]
pub(crate) fn validate_path_segments(segments: &[&str]) -> crate::Result<()> {
    for (i, segment) in segments.iter().enumerate() {
        if segment.is_empty() {
            return Err(crate::Error::Validation(format!(
                "path segment [{}] is empty",
                i
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_escape_segments_safe() {
        assert_eq!(path_escape_segments("org/repo/file"), "org/repo/file");
    }

    #[test]
    fn test_path_escape_segments_special() {
        let result = path_escape_segments("org/repo/path file");
        assert!(result.contains("path%20file"), "got: {result}");
    }

    #[test]
    fn test_path_escape_segments_slash_preserved() {
        let result = path_escape_segments("a/b/c");
        assert_eq!(result, "a/b/c");
        assert!(
            !result.contains("%2F"),
            "slashes must not be escaped: {result}"
        );
    }

    #[test]
    fn test_validate_and_escape_segments_ok() {
        let result = validate_and_escape_segments(&["owner", "repo"]).unwrap();
        assert_eq!(result, vec!["owner", "repo"]);
    }

    #[test]
    fn test_validate_and_escape_segments_empty() {
        let err = validate_and_escape_segments(&["owner", ""]).unwrap_err();
        assert!(err.to_string().contains("path segment [1] is empty"));
    }

    #[test]
    fn test_validate_and_escape_segments_special() {
        let result = validate_and_escape_segments(&["a/b", "repo"]).unwrap();
        assert!(result[0].contains("%2F"), "got: {}", result[0]);
        assert_eq!(result[1], "repo");
    }

    #[test]
    fn test_validate_path_segments_ok() {
        assert!(validate_path_segments(&["owner", "repo"]).is_ok());
    }

    #[test]
    fn test_validate_path_segments_empty() {
        let err = validate_path_segments(&["owner", ""]).unwrap_err();
        assert!(err.to_string().contains("path segment [1] is empty"));
    }

    #[test]
    fn test_validate_and_escape_segments_null_byte() {
        let err = validate_and_escape_segments(&["owner", "repo\x00name"]).unwrap_err();
        assert!(err.to_string().contains("null byte"));
    }

    #[test]
    fn test_validate_and_escape_segments_dotdot() {
        let err = validate_and_escape_segments(&["owner", ".."]).unwrap_err();
        assert!(err.to_string().contains("path traversal"));
    }

    #[test]
    fn test_validate_and_escape_segments_dotdot_prefix() {
        let err = validate_and_escape_segments(&["owner", "../etc/passwd"]).unwrap_err();
        assert!(err.to_string().contains("path traversal"));
    }

    #[test]
    fn test_validate_and_escape_segments_dotdot_infix() {
        let err = validate_and_escape_segments(&["owner", "foo/../bar"]).unwrap_err();
        assert!(err.to_string().contains("path traversal"));
    }

    #[test]
    fn test_validate_and_escape_segments_max_length() {
        let long_segment = "a".repeat(4097);
        let err = validate_and_escape_segments(&["owner", &long_segment]).unwrap_err();
        assert!(err.to_string().contains("maximum length"));
    }

    #[test]
    fn test_validate_and_escape_segments_max_length_ok() {
        let max_segment = "a".repeat(4096);
        let result = validate_and_escape_segments(&["owner", &max_segment]).unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_validate_and_escape_segments_dotted_name_ok() {
        let result = validate_and_escape_segments(&["user.name", "repo.name"]).unwrap();
        assert_eq!(result, vec!["user%2Ename", "repo%2Ename"]);
    }
}
