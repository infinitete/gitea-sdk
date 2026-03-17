/// Options for Gitea API pagination.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListOptions {
    /// Page number. None=server default, Some(0)=disable pagination, Some(n)=explicit page (n >= 1)
    pub page: Option<i32>,
    /// Page size. None=server default
    pub page_size: Option<i32>,
}

impl Default for ListOptions {
    fn default() -> Self {
        Self {
            page: None,
            page_size: None,
        }
    }
}

impl ListOptions {
    /// Apply defaults and return a new value. Idempotent.
    pub fn with_defaults(&self) -> Self {
        let page = match self.page {
            None => Some(1),
            Some(-1) => Some(0),
            Some(n) => Some(n),
        };
        Self {
            page,
            page_size: self.page_size,
        }
    }
}

/// Encode a value as a URL query string (without leading `?`).
pub trait QueryEncode {
    /// Returns URL query string (without leading `?`)
    fn query_encode(&self) -> String;
}

impl QueryEncode for ListOptions {
    fn query_encode(&self) -> String {
        let defaulted = self.with_defaults();
        let mut out = String::new();
        if defaulted.page == Some(0) {
            out.push_str("page=0&limit=0");
        } else {
            out.push_str(&format!("page={}", defaulted.page.unwrap()));
            if let Some(size) = defaulted.page_size {
                out.push_str(&format!("&limit={size}"));
            }
        }
        out
    }
}

/// Trait for types that carry pagination options.
///
/// Extended ListXxxOptions (Phase 1b) will implement this.
pub trait PaginationOptions: QueryEncode {
    /// Apply defaults, returning a new value without mutation.
    fn set_defaults(self) -> Self
    where
        Self: Sized;
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
                page_size: None,
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
                page: Some(1),
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
        assert_eq!(opts.query_encode(), "page=1");
    }
}
