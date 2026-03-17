// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTask {
    pub id: i64,
    pub name: String,
    #[serde(rename = "head_branch")]
    pub head_branch: String,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    #[serde(rename = "run_number")]
    pub run_number: i64,
    pub event: String,
    #[serde(rename = "display_title")]
    pub display_title: String,
    pub status: String,
    #[serde(rename = "workflow_id")]
    pub workflow_id: String,
    pub url: String,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub updated_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub run_started_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionTaskResponse {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(default)]
    pub workflow_runs: Vec<ActionTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWorkflowRun {
    pub id: i64,
    #[serde(rename = "display_title")]
    pub display_title: String,
    pub event: String,
    #[serde(rename = "head_branch")]
    pub head_branch: String,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    pub path: String,
    #[serde(rename = "run_attempt")]
    pub run_attempt: i64,
    #[serde(rename = "run_number")]
    pub run_number: i64,
    pub status: String,
    pub conclusion: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(with = "rfc3339")]
    pub started_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub completed_at: OffsetDateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actor: Option<serde_json::Value>,
    #[serde(
        rename = "trigger_actor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_actor: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    #[serde(
        rename = "head_repository",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub head_repository: Option<serde_json::Value>,
    #[serde(rename = "repository_id")]
    pub repository_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWorkflowRunsResponse {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(default)]
    pub workflow_runs: Vec<ActionWorkflowRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWorkflowJob {
    pub id: i64,
    #[serde(rename = "run_id")]
    pub run_id: i64,
    #[serde(rename = "run_url")]
    pub run_url: String,
    #[serde(rename = "run_attempt")]
    pub run_attempt: i64,
    pub name: String,
    #[serde(rename = "head_branch")]
    pub head_branch: String,
    #[serde(rename = "head_sha")]
    pub head_sha: String,
    pub status: String,
    pub conclusion: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(with = "rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub started_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub completed_at: OffsetDateTime,
    #[serde(rename = "runner_id")]
    pub runner_id: i64,
    #[serde(rename = "runner_name")]
    pub runner_name: String,
    #[serde(default)]
    pub labels: Vec<String>,
    #[serde(default)]
    pub steps: Vec<ActionWorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWorkflowJobsResponse {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(default)]
    pub jobs: Vec<ActionWorkflowJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionWorkflowStep {
    pub name: String,
    pub number: i64,
    pub status: String,
    pub conclusion: String,
    #[serde(with = "rfc3339")]
    pub started_at: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub completed_at: OffsetDateTime,
}
