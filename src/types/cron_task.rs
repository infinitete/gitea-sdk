// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

/// CronTask represents a Cron task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronTask {
    pub name: String,
    pub schedule: String,
    #[serde(with = "rfc3339")]
    pub next: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub prev: OffsetDateTime,
    #[serde(rename = "exec_times")]
    pub exec_times: i64,
}
