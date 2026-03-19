// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for organization API endpoints.

pub mod action;
pub mod activity;
pub mod core;
pub mod label;
pub mod team;

pub use action::*;
pub use activity::*;
pub use core::*;
pub use label::*;
pub use team::*;

// ── helpers ─────────────────────────────────────────────────────────────

pub(crate) fn percent_encode(s: &str) -> String {
    use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
    utf8_percent_encode(s, NON_ALPHANUMERIC).to_string()
}
