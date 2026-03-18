// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for Gitea Actions: tasks, workflow runs, jobs, and steps.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::serde_helpers::null_to_default;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action Task payload type.
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
/// Action Task Response payload type.
pub struct ActionTaskResponse {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(default)]
    pub workflow_runs: Vec<ActionTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action Workflow Run payload type.
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
    #[serde(default)]
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
    #[serde(rename = "repository_id", default)]
    pub repository_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action Workflow Runs Response payload type.
pub struct ActionWorkflowRunsResponse {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(default)]
    pub workflow_runs: Vec<ActionWorkflowRun>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action Workflow Job payload type.
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
    #[serde(default)]
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
    #[serde(rename = "runner_id", default)]
    pub runner_id: i64,
    #[serde(rename = "runner_name", default)]
    pub runner_name: String,
    #[serde(default, deserialize_with = "null_to_default")]
    pub labels: Vec<String>,
    #[serde(default, deserialize_with = "null_to_default")]
    pub steps: Vec<ActionWorkflowStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action Workflow Jobs Response payload type.
pub struct ActionWorkflowJobsResponse {
    #[serde(rename = "total_count")]
    pub total_count: i64,
    #[serde(default)]
    pub jobs: Vec<ActionWorkflowJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Action Workflow Step payload type.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_time() -> OffsetDateTime {
        OffsetDateTime::new_utc(
            time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
            time::Time::from_hms(10, 0, 0).unwrap(),
        )
    }

    #[test]
    fn test_action_task_round_trip() {
        let original = ActionTask {
            id: 1,
            name: "build".to_string(),
            head_branch: "main".to_string(),
            head_sha: "abc123".to_string(),
            run_number: 42,
            event: "push".to_string(),
            display_title: "Build #42".to_string(),
            status: "completed".to_string(),
            workflow_id: "1234".to_string(),
            url: "https://gitea.example.com/runs/1".to_string(),
            created_at: test_time(),
            updated_at: test_time(),
            run_started_at: test_time(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionTask = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.head_sha, original.head_sha);
    }

    #[test]
    fn test_action_task_response_round_trip() {
        let original = ActionTaskResponse {
            total_count: 1,
            workflow_runs: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionTaskResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.total_count, 1);
        assert!(restored.workflow_runs.is_empty());
    }

    #[test]
    fn test_action_task_response_with_runs() {
        let run = ActionTask {
            id: 1,
            name: "build".to_string(),
            head_branch: "main".to_string(),
            head_sha: "abc123".to_string(),
            run_number: 1,
            event: "push".to_string(),
            display_title: "Build".to_string(),
            status: "success".to_string(),
            workflow_id: "10".to_string(),
            url: "https://example.com".to_string(),
            created_at: test_time(),
            updated_at: test_time(),
            run_started_at: test_time(),
        };
        let original = ActionTaskResponse {
            total_count: 1,
            workflow_runs: vec![run],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionTaskResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.workflow_runs.len(), 1);
    }

    #[test]
    fn test_action_workflow_run_round_trip() {
        let original = ActionWorkflowRun {
            id: 1,
            display_title: "CI".to_string(),
            event: "push".to_string(),
            head_branch: "main".to_string(),
            head_sha: "abc123".to_string(),
            path: ".gitea/workflows/ci.yml".to_string(),
            run_attempt: 1,
            run_number: 42,
            status: "completed".to_string(),
            conclusion: "success".to_string(),
            url: "https://example.com/runs/1".to_string(),
            html_url: "https://example.com/runs/1".to_string(),
            started_at: test_time(),
            completed_at: test_time(),
            actor: None,
            trigger_actor: None,
            repository: None,
            head_repository: None,
            repository_id: 1,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionWorkflowRun = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.event, original.event);
        assert!(restored.actor.is_none());
    }

    #[test]
    fn test_action_workflow_jobs_response_round_trip() {
        let original = ActionWorkflowJobsResponse {
            total_count: 0,
            jobs: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionWorkflowJobsResponse = serde_json::from_str(&json).unwrap();
        assert!(restored.jobs.is_empty());
    }

    #[test]
    fn test_action_workflow_job_round_trip() {
        let original = ActionWorkflowJob {
            id: 1,
            run_id: 10,
            run_url: "https://example.com/runs/10".to_string(),
            run_attempt: 1,
            name: "test".to_string(),
            head_branch: "main".to_string(),
            head_sha: "abc123".to_string(),
            status: "completed".to_string(),
            conclusion: "success".to_string(),
            url: "https://example.com/jobs/1".to_string(),
            html_url: "https://example.com/jobs/1".to_string(),
            created_at: test_time(),
            started_at: test_time(),
            completed_at: test_time(),
            runner_id: 1,
            runner_name: "linux".to_string(),
            labels: vec![],
            steps: vec![],
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionWorkflowJob = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert!(restored.labels.is_empty());
        assert!(restored.steps.is_empty());
    }

    #[test]
    fn test_action_workflow_step_round_trip() {
        let original = ActionWorkflowStep {
            name: "checkout".to_string(),
            number: 1,
            status: "completed".to_string(),
            conclusion: "success".to_string(),
            started_at: test_time(),
            completed_at: test_time(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: ActionWorkflowStep = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.number, original.number);
    }
}
