// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::internal::request::urlencoding;
use crate::pagination::{ListOptions, QueryEncode, push_query_segment};
use crate::{Deserialize, Serialize};

// ── repo_key.go ─────────────────────────────────────────────────

/// `ListDeployKeysOptions` options for listing a repository's deploy keys
#[derive(Debug, Clone, Default)]
/// Options for List Deploy Keys Option.
pub struct ListDeployKeysOptions {
    pub list_options: ListOptions,
    pub key_id: i64,
    pub fingerprint: String,
}

impl QueryEncode for ListDeployKeysOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();

        if self.key_id > 0 {
            push_query_segment(&mut out, &format!("key_id={}", self.key_id));
        }
        if !self.fingerprint.is_empty() {
            push_query_segment(
                &mut out,
                &format!("fingerprint={}", urlencoding(&self.fingerprint)),
            );
        }

        out
    }
}

/// `CreateKeyOption` options when creating a deploy key
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Key Option.
pub struct CreateKeyOption {
    pub title: String,
    /// An armored SSH key to add
    pub key: String,
    /// Describe if the key has only read access or read/write
    #[serde(rename = "read_only")]
    pub read_only: bool,
}
