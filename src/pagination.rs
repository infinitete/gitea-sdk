// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Pagination options for list endpoints.

/// Options for Gitea API pagination.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ListOptions {
    /// Page number. `None` omits the page parameter, `Some(0)` disables pagination,
    /// and positive values request an explicit page.
    pub page: Option<i32>,
    /// Page size. `None` omits the limit parameter.
    pub page_size: Option<i32>,
}

impl ListOptions {
    /// Validate pagination inputs before sending them to the server.
    ///
    /// Non-positive `page_size` values are only valid when pagination is being
    /// explicitly disabled via a non-positive `page`.
    pub fn validate(&self) -> crate::Result<()> {
        if self.page_size.is_some_and(|size| size <= 0) && self.page.is_none_or(|page| page > 0) {
            return Err(crate::Error::Validation(
                "page_size must be positive unless pagination is disabled".to_string(),
            ));
        }
        Ok(())
    }

    /// Normalize pagination values into the wire format expected by Gitea.
    ///
    /// Non-positive page numbers are normalized to `page=0`.
    ///
    /// When callers also provide a positive `page_size`, that explicit limit is
    /// preserved. Non-positive page sizes are still coerced to `limit=0`.
    #[must_use]
    pub fn with_defaults(&self) -> Self {
        if self.page_size.is_some_and(|size| size <= 0) {
            return Self {
                page: Some(0),
                page_size: Some(0),
            };
        }

        let page = if self.page.is_some_and(|page| page <= 0) {
            Some(0)
        } else {
            self.page
        };

        let page_size = if page == Some(0) {
            self.page_size.or(Some(0))
        } else {
            self.page_size
        };

        Self { page, page_size }
    }
}

/// Encode a value as a URL query string (without leading `?`).
pub trait QueryEncode {
    /// Returns URL query string (without leading `?`)
    fn query_encode(&self) -> String;
}

pub(crate) fn push_query_segment(query: &mut String, segment: &str) {
    if segment.is_empty() {
        return;
    }
    if !query.is_empty() {
        query.push('&');
    }
    query.push_str(segment);
}

impl QueryEncode for ListOptions {
    fn query_encode(&self) -> String {
        let defaulted = self.with_defaults();
        let mut out = String::new();
        if let Some(page) = defaulted.page {
            push_query_segment(&mut out, &format!("page={page}"));
            if let Some(size) = defaulted.page_size {
                push_query_segment(&mut out, &format!("limit={size}"));
            }
        } else if let Some(size) = defaulted.page_size {
            push_query_segment(&mut out, &format!("limit={size}"));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_defaults_idempotent() {
        let d = ListOptions::default();
        let once = d.with_defaults();
        let twice = once.with_defaults();
        assert_eq!(once, twice);
    }

    #[test]
    fn test_with_defaults_negative_page() {
        let opts = ListOptions {
            page: Some(-1),
            page_size: Some(10),
        };
        assert_eq!(
            opts.with_defaults(),
            ListOptions {
                page: Some(0),
                page_size: Some(10),
            }
        );
    }

    #[test]
    fn test_with_defaults_zero_page() {
        let opts = ListOptions {
            page: Some(0),
            page_size: None,
        };
        assert_eq!(
            opts.with_defaults(),
            ListOptions {
                page: Some(0),
                page_size: Some(0),
            }
        );
    }

    #[test]
    fn test_with_defaults_none_page() {
        let opts = ListOptions {
            page: None,
            page_size: Some(20),
        };
        assert_eq!(
            opts.with_defaults(),
            ListOptions {
                page: None,
                page_size: Some(20),
            }
        );
    }

    #[test]
    fn test_with_defaults_explicit_page() {
        let opts = ListOptions {
            page: Some(3),
            page_size: Some(50),
        };
        assert_eq!(
            opts.with_defaults(),
            ListOptions {
                page: Some(3),
                page_size: Some(50),
            }
        );
    }

    #[test]
    fn test_query_encode_normal() {
        let opts = ListOptions {
            page: Some(1),
            page_size: Some(20),
        };
        assert_eq!(opts.query_encode(), "page=1&limit=20");
    }

    #[test]
    fn test_query_encode_disable() {
        let opts = ListOptions {
            page: Some(0),
            page_size: Some(0),
        };
        assert_eq!(opts.query_encode(), "page=0&limit=0");
    }

    #[test]
    fn test_query_encode_empty() {
        let opts = ListOptions::default();
        assert_eq!(opts.query_encode(), "");
    }

    #[test]
    fn test_query_encode_page_size_only() {
        let opts = ListOptions {
            page: None,
            page_size: Some(25),
        };
        assert_eq!(opts.query_encode(), "limit=25");
    }

    #[test]
    fn test_query_encode_negative_page_disables_pagination() {
        let opts = ListOptions {
            page: Some(-2),
            page_size: Some(25),
        };
        assert_eq!(opts.query_encode(), "page=0&limit=25");
    }

    #[test]
    fn test_query_encode_non_positive_page_size_disables_pagination() {
        let opts = ListOptions {
            page: Some(2),
            page_size: Some(0),
        };
        assert_eq!(opts.query_encode(), "page=0&limit=0");

        let opts = ListOptions {
            page: None,
            page_size: Some(-5),
        };
        assert_eq!(opts.query_encode(), "page=0&limit=0");
    }

    #[test]
    fn test_validate_rejects_non_positive_page_size_without_disabled_pagination() {
        let opts = ListOptions {
            page: Some(2),
            page_size: Some(0),
        };
        assert!(opts.validate().is_err());

        let opts = ListOptions {
            page: None,
            page_size: Some(-5),
        };
        assert!(opts.validate().is_err());
    }

    #[test]
    fn test_validate_allows_non_positive_page_size_when_pagination_disabled() {
        let opts = ListOptions {
            page: Some(0),
            page_size: Some(0),
        };
        assert!(opts.validate().is_ok());
    }
}
