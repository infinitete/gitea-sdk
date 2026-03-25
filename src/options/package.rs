// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for package API endpoints.

use crate::pagination::{ListOptions, QueryEncode};

#[derive(Debug, Clone, Default)]
/// Options for listing packages.
pub struct ListPackagesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPackagesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_packages_options_query_encode_default() {
        let opt = ListPackagesOptions::default();
        assert_eq!(opt.query_encode(), "");
    }

    #[test]
    fn test_list_packages_options_query_encode_with_page_size() {
        let opt = ListPackagesOptions {
            list_options: ListOptions {
                page: Some(2),
                page_size: Some(50),
            },
        };
        assert_eq!(opt.query_encode(), "page=2&limit=50");
    }
}
