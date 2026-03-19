// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Admin API endpoints for Gitea instance administration tasks.

use crate::Client;

/// API methods for admin tasks. Access via [`Client::admin()`](crate::Client::admin).
pub struct AdminApi<'a> {
    client: &'a Client,
}

impl<'a> AdminApi<'a> {
    /// Create a new `AdminApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}

pub mod badges;
pub mod core;
pub mod cron;
pub mod email;
pub mod hooks;
pub mod org;
pub mod repo;
#[cfg(test)]
pub(crate) mod test_helpers;
