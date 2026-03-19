// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};

// ── org_member.go ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ListOrgMembershipOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgMembershipOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── org_block.go ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub struct ListOrgBlocksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListOrgBlocksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}
