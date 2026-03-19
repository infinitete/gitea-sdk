// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Issue API endpoints for managing Gitea issues, labels, and milestones.

use crate::Client;

/// API methods for issues. Access via [`Client::issues()`](crate::Client::issues).
pub struct IssuesApi<'a> {
    client: &'a Client,
}

impl<'a> IssuesApi<'a> {
    /// Create a new `IssuesApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}

pub mod comment;
pub mod core;
pub mod label;
pub mod milestone;
pub mod pin;
pub mod reaction;
pub mod subscription;
pub mod template;
#[cfg(test)]
pub(crate) mod test_helpers;
pub mod time;
pub mod timeline;
