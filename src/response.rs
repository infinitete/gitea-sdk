// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! HTTP response metadata and pagination link parsing.

use reqwest::header::HeaderMap;

/// Pagination links extracted from the `Link` response header.
///
/// Gitea returns RFC 5988 `Link` headers for paginated endpoints, e.g.:
/// `<https://example.com/repos?page=2>; rel="next", <https://example.com/repos?page=5>; rel="last"`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PageLinks {
    /// The first page number.
    pub first: Option<u32>,
    /// The previous page number.
    pub prev: Option<u32>,
    /// The next page number.
    pub next: Option<u32>,
    /// The last page number.
    pub last: Option<u32>,
}

/// HTTP response metadata without the body.
///
/// Stores the status code, headers, and parsed pagination links.
/// The response body is consumed separately by the caller or returned
/// in [`Error`](crate::Error) variants.
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code (e.g. 200, 404).
    pub status: u16,
    /// Response headers.
    pub headers: HeaderMap,
    /// Parsed pagination links, if present.
    pub page_links: Option<PageLinks>,
}

/// Parse RFC 5988 `Link` header into [`PageLinks`].
///
/// Expects format: `<url?page=N>; rel="rel_name"` with entries separated by `,`.
/// Malformed entries are silently ignored. Returns `None` if no valid links are found.
fn parse_link_header(headers: &HeaderMap) -> Option<PageLinks> {
    let link = headers.get("link")?.to_str().ok()?;
    if link.is_empty() {
        return None;
    }

    let mut first: Option<u32> = None;
    let mut prev: Option<u32> = None;
    let mut next: Option<u32> = None;
    let mut last: Option<u32> = None;

    for entry in link.split(',') {
        let (url_part, param_part) = match entry.split_once(';') {
            Some(parts) => parts,
            None => continue,
        };

        let url = url_part
            .trim()
            .trim_start_matches('<')
            .trim_end_matches('>');

        let param = param_part.trim();
        let (key, value) = match param.split_once('=') {
            Some(parts) => parts,
            None => continue,
        };
        if key != "rel" {
            continue;
        }

        let rel = value.trim_matches('"');

        let parsed_url = match url::Url::parse(url) {
            Ok(u) => u,
            Err(_) => continue,
        };

        let page_str = parsed_url.query_pairs().find_map(|(k, v)| {
            if k == "page" {
                Some(v.into_owned())
            } else {
                None
            }
        });
        let page_str = match page_str {
            Some(p) => p,
            None => continue,
        };
        let page: u32 = match page_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };
        // Go's strconv.Atoi returns 0 for invalid; 0 means "not set".
        // Since we use Option<u32>, 0 is a valid page value — but for
        // consistency with Go behavior, skip page=0 (treated as unset).
        if page == 0 {
            continue;
        }

        match rel {
            "first" => first = Some(page),
            "prev" => prev = Some(page),
            "next" => next = Some(page),
            "last" => last = Some(page),
            _ => {}
        }
    }

    if first.is_none() && prev.is_none() && next.is_none() && last.is_none() {
        return None;
    }

    Some(PageLinks {
        first,
        prev,
        next,
        last,
    })
}

/// Create a [`Response`] from a [`reqwest::Response`].
///
/// Clones the headers and status, then parses the `Link` header for pagination.
pub fn response_from_reqwest(resp: &reqwest::Response) -> Response {
    let status = resp.status().as_u16();
    let headers = resp.headers().clone();
    let page_links = parse_link_header(&headers);

    Response {
        status,
        headers,
        page_links,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_header(link: &str) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert("link", link.parse().unwrap());
        h
    }

    #[test]
    fn test_parse_link_header_normal() {
        let h = make_header(
            r#"<https://example.com/repos?page=2>; rel="next", <https://example.com/repos?page=5>; rel="last""#,
        );
        let links = parse_link_header(&h).unwrap();
        assert_eq!(links.first, None);
        assert_eq!(links.prev, None);
        assert_eq!(links.next, Some(2));
        assert_eq!(links.last, Some(5));
    }

    #[test]
    fn test_parse_link_header_all_four() {
        let h = make_header(
            r#"<https://example.com/repos?page=1>; rel="first", <https://example.com/repos?page=3>; rel="prev", <https://example.com/repos?page=5>; rel="next", <https://example.com/repos?page=10>; rel="last""#,
        );
        let links = parse_link_header(&h).unwrap();
        assert_eq!(links.first, Some(1));
        assert_eq!(links.prev, Some(3));
        assert_eq!(links.next, Some(5));
        assert_eq!(links.last, Some(10));
    }

    #[test]
    fn test_parse_link_header_empty() {
        let h = HeaderMap::new();
        assert!(parse_link_header(&h).is_none());
    }

    #[test]
    fn test_parse_link_header_empty_link_value() {
        let mut h = HeaderMap::new();
        h.insert("link", "".parse().unwrap());
        assert!(parse_link_header(&h).is_none());
    }

    #[test]
    fn test_parse_link_header_malformed_page() {
        let h = make_header(r#"<https://example.com/repos?page=abc>; rel="next""#);
        let links = parse_link_header(&h);
        assert!(links.is_none());
    }

    #[test]
    fn test_parse_link_header_same_rel_last_wins() {
        let h = make_header(
            r#"<https://example.com/repos?page=2>; rel="next", <https://example.com/repos?page=4>; rel="next""#,
        );
        let links = parse_link_header(&h).unwrap();
        assert_eq!(links.next, Some(4));
    }

    #[test]
    fn test_parse_link_header_no_page_param() {
        let h = make_header(r#"<https://example.com/repos>; rel="next""#);
        assert!(parse_link_header(&h).is_none());
    }

    #[test]
    fn test_parse_link_header_page_zero_ignored() {
        let h = make_header(r#"<https://example.com/repos?page=0>; rel="next""#);
        assert!(parse_link_header(&h).is_none());
    }

    #[test]
    fn test_parse_link_header_malformed_entry_ignored() {
        // Mix of valid and malformed entries
        let h = make_header(r#"not-a-link, <https://example.com/repos?page=3>; rel="next""#);
        let links = parse_link_header(&h).unwrap();
        assert_eq!(links.next, Some(3));
    }

    #[test]
    fn test_parse_link_header_unknown_rel_ignored() {
        let h = make_header(r#"<https://example.com/repos?page=1>; rel="unknown""#);
        assert!(parse_link_header(&h).is_none());
    }
}
