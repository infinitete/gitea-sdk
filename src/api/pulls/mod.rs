// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Pull request API endpoints for managing Gitea pull requests and reviews.

use crate::Client;

/// API methods for pull requests. Access via [`Client::pulls()`](crate::Client::pulls).
pub struct PullsApi<'a> {
    client: &'a Client,
}

impl<'a> PullsApi<'a> {
    /// Create a new `PullsApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}

pub mod core;
pub mod merge;
pub mod request;
pub mod review;
pub mod submit;
#[cfg(test)]
pub(crate) mod test_helpers;

// Re-export public types
pub use request::{CommitMeta, CommitUser, PullRequestCommit};
