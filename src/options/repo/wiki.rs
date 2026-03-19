// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::{Deserialize, Serialize};

// ── repo_wiki.go ────────────────────────────────────────────────

/// CreateWikiPageOptions options for creating or editing a wiki page
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Create Wiki Page Option.
pub struct CreateWikiPageOptions {
    pub title: String,
    #[serde(rename = "content_base64")]
    pub content_base64: String,
    pub message: String,
}

/// ListWikiPagesOptions options for listing wiki pages
#[derive(Debug, Clone, Default)]
/// Options for List Wiki Pages Option.
pub struct ListWikiPagesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListWikiPagesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListWikiPageRevisionsOptions options for listing wiki page revisions
#[derive(Debug, Clone, Default)]
/// Options for List Wiki Page Revisions Option.
pub struct ListWikiPageRevisionsOptions {
    pub page: i32,
}
