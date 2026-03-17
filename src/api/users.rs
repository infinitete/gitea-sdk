// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;

/// API methods for user operations.
pub struct UsersApi<'a> {
    client: &'a Client,
}

impl<'a> UsersApi<'a> {
    /// Create a new `UsersApi` for the given client.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get a reference to the underlying client.
    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}
