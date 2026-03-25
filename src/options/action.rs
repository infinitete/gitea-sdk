// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for CI/CD action API endpoints.

use crate::pagination::{ListOptions, QueryEncode, push_query_segment};

#[derive(Debug, Clone, Default)]
/// Options for List Repo Action Runs Option.
pub struct ListRepoActionRunsOptions {
    pub list_options: ListOptions,
    pub branch: Option<String>,
    pub event: Option<String>,
    pub status: Option<String>,
    pub actor: Option<String>,
    pub head_sha: Option<String>,
}

impl QueryEncode for ListRepoActionRunsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(ref branch) = self.branch {
            push_query_segment(&mut out, &format!("branch={branch}"));
        }
        if let Some(ref event) = self.event {
            push_query_segment(&mut out, &format!("event={event}"));
        }
        if let Some(ref status) = self.status {
            push_query_segment(&mut out, &format!("status={status}"));
        }
        if let Some(ref actor) = self.actor {
            push_query_segment(&mut out, &format!("actor={actor}"));
        }
        if let Some(ref head_sha) = self.head_sha {
            push_query_segment(&mut out, &format!("head_sha={head_sha}"));
        }
        out
    }
}

#[derive(Debug, Clone, Default)]
/// Options for List Repo Action Jobs Option.
pub struct ListRepoActionJobsOptions {
    pub list_options: ListOptions,
    pub status: Option<String>,
}

impl QueryEncode for ListRepoActionJobsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(ref status) = self.status {
            push_query_segment(&mut out, &format!("status={status}"));
        }
        out
    }
}
