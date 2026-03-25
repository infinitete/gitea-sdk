// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Organization API endpoints for managing Gitea organizations, teams, and members.

use crate::Client;

/// API methods for organizations. Access via [`Client::orgs()`](crate::Client::orgs).
pub struct OrgsApi<'a> {
    client: &'a Client,
}

impl<'a> OrgsApi<'a> {
    /// Create a new `OrgsApi` view.
    #[must_use]
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}

pub mod action;
pub mod action_secret;
pub mod block;
pub mod core;
pub mod label;
pub mod member;
pub mod social;
pub mod team;
pub mod team_member;
pub mod team_repo;
#[cfg(test)]
pub(crate) mod test_helpers;
