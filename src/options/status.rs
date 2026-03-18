// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for commit status API endpoints.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::StatusState;
use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Status Option.
pub struct CreateStatusOption {
    pub state: StatusState,
    #[serde(rename = "target_url", skip_serializing_if = "Option::is_none")]
    pub target_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
}

#[derive(Debug, Clone, Default)]
/// Options for List Statuses Option.
pub struct ListStatusesOption {
    pub list_options: ListOptions,
}

impl QueryEncode for ListStatusesOption {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}
