// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;

pub struct ReposApi<'a> {
    client: &'a Client,
}

impl<'a> ReposApi<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }
}

pub mod action;
pub mod branch;
pub mod collaborator;
pub mod commit;
pub mod core;
pub mod file;
pub mod file_ext;
pub mod file_write;
pub mod git;
pub mod key;
pub mod label;
pub mod meta;
pub mod misc;
pub mod misc2;
pub mod protection;
pub mod repo_ops;
pub mod starred;
pub mod tag;
pub mod tag_protection;
pub mod team;
#[cfg(test)]
pub(crate) mod test_helpers;
pub mod topics;
pub mod watch;
pub mod wiki;
