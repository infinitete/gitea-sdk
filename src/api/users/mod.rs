// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! User API endpoints for managing Gitea user accounts, keys, and settings.

use crate::Client;

/// API methods for users. Access via [`Client::users()`](crate::Client::users).
pub struct UsersApi<'a> {
    client: &'a Client,
}

impl<'a> UsersApi<'a> {
    /// Create a new `UsersApi` view.
    #[must_use]
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}

pub mod activity;
pub mod app;
pub mod block;
pub mod core;
pub mod email;
pub mod follow;
pub mod gpg;
pub mod key;
pub mod search;
pub mod settings;
pub mod social;
#[cfg(test)]
pub(crate) mod test_helpers;
