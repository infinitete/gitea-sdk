// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Request option types for issue API endpoints.

use crate::internal::request::urlencoding;
use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::StateType;
use crate::types::serde_helpers::nullable_rfc3339;
use crate::{Deserialize, Serialize};
use time::OffsetDateTime;

// ── issue.go ─────────────────────────────────────────────────────

/// ListIssueOption list issue options
#[derive(Debug, Clone, Default)]
/// Options for List Issue Option.
pub struct ListIssueOption {
    pub list_options: ListOptions,
    pub state: Option<StateType>,
    pub r#type: Option<crate::types::enums::IssueType>,
    pub labels: Vec<String>,
    pub milestones: Vec<String>,
    pub key_word: String,
    pub since: Option<OffsetDateTime>,
    pub before: Option<OffsetDateTime>,
    /// filter by created by username
    pub created_by: String,
    /// filter by assigned to username
    pub assigned_by: String,
    /// filter by username mentioned
    pub mentioned_by: String,
    /// filter by owner (only works on ListIssues on User)
    pub owner: String,
    /// filter by team (requires organization owner parameter)
    pub team: String,
}

impl QueryEncode for ListIssueOption {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();

        if let Some(ref state) = self.state {
            out.push_str(&format!("&state={}", state.as_ref()));
        }

        if !self.labels.is_empty() {
            out.push_str(&format!("&labels={}", self.labels.join(",")));
        }

        if !self.key_word.is_empty() {
            out.push_str(&format!("&q={}", urlencoding(&self.key_word)));
        }

        if let Some(ref t) = self.r#type {
            out.push_str(&format!("&type={}", t.as_ref()));
        }

        if !self.milestones.is_empty() {
            out.push_str(&format!("&milestones={}", self.milestones.join(",")));
        }

        if let Some(since) = self.since {
            let formatted = since
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            out.push_str(&format!("&since={}", urlencoding(&formatted)));
        }
        if let Some(before) = self.before {
            let formatted = before
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            out.push_str(&format!("&before={}", urlencoding(&formatted)));
        }

        if !self.created_by.is_empty() {
            out.push_str(&format!("&created_by={}", urlencoding(&self.created_by)));
        }
        if !self.assigned_by.is_empty() {
            out.push_str(&format!("&assigned_by={}", urlencoding(&self.assigned_by)));
        }
        if !self.mentioned_by.is_empty() {
            out.push_str(&format!(
                "&mentioned_by={}",
                urlencoding(&self.mentioned_by)
            ));
        }
        if !self.owner.is_empty() {
            out.push_str(&format!("&owner={}", urlencoding(&self.owner)));
        }
        if !self.team.is_empty() {
            out.push_str(&format!("&team={}", urlencoding(&self.team)));
        }

        out
    }
}

/// CreateIssueOption options to create one issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Issue Option.
pub struct CreateIssueOption {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub r#ref: String,
    #[serde(default)]
    pub assignees: Vec<String>,
    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
    /// milestone id
    #[serde(default)]
    pub milestone: i64,
    /// list of label ids
    #[serde(default)]
    pub labels: Vec<i64>,
    #[serde(default)]
    pub closed: bool,
}

impl CreateIssueOption {
    /// Validate the CreateIssueOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.title.trim().is_empty() {
            return Err(crate::Error::Validation("title is empty".to_string()));
        }
        Ok(())
    }
}

/// EditIssueOption options for editing an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Issue Option.
pub struct EditIssueOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(default)]
    pub assignees: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<StateType>,
    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
    #[serde(
        rename = "unset_due_date",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remove_deadline: Option<bool>,
}

impl EditIssueOption {
    /// Validate the EditIssueOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if let Some(ref title) = self.title
            && title.trim().is_empty()
        {
            return Err(crate::Error::Validation("title is empty".to_string()));
        }
        Ok(())
    }
}

// ── issue_comment.go ─────────────────────────────────────────────

/// ListIssueCommentOptions list comment options
#[derive(Debug, Clone, Default)]
/// Options for List Issue Comment Option.
pub struct ListIssueCommentOptions {
    pub list_options: ListOptions,
    pub since: Option<OffsetDateTime>,
    pub before: Option<OffsetDateTime>,
}

impl QueryEncode for ListIssueCommentOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(since) = self.since {
            let formatted = since
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            out.push_str(&format!("&since={}", urlencoding(&formatted)));
        }
        if let Some(before) = self.before {
            let formatted = before
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            out.push_str(&format!("&before={}", urlencoding(&formatted)));
        }
        out
    }
}

/// CreateIssueCommentOption options for creating a comment on an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Issue Comment Option.
pub struct CreateIssueCommentOption {
    pub body: String,
}

impl CreateIssueCommentOption {
    /// Validate the CreateIssueCommentOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.body.is_empty() {
            return Err(crate::Error::Validation("body is empty".to_string()));
        }
        Ok(())
    }
}

/// EditIssueCommentOption options for editing a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Issue Comment Option.
pub struct EditIssueCommentOption {
    pub body: String,
}

impl EditIssueCommentOption {
    /// Validate the EditIssueCommentOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.body.is_empty() {
            return Err(crate::Error::Validation("body is empty".to_string()));
        }
        Ok(())
    }
}

// ── issue_label.go ───────────────────────────────────────────────

/// IssueLabelsOption a collection of labels
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Issue Labels Option.
pub struct IssueLabelsOption {
    /// list of label IDs
    #[serde(default)]
    pub labels: Vec<i64>,
}

// ── issue_milestone.go ───────────────────────────────────────────

/// ListMilestoneOption list milestone options
#[derive(Debug, Clone, Default)]
/// Options for List Milestone Option.
pub struct ListMilestoneOption {
    pub list_options: ListOptions,
    /// open, closed, all
    pub state: Option<StateType>,
    pub name: String,
}

impl QueryEncode for ListMilestoneOption {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(ref state) = self.state {
            out.push_str(&format!("&state={}", state.as_ref()));
        }
        if !self.name.is_empty() {
            out.push_str(&format!("&name={}", urlencoding(&self.name)));
        }
        out
    }
}

/// CreateMilestoneOption options for creating a milestone
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Milestone Option.
pub struct CreateMilestoneOption {
    pub title: String,
    #[serde(default)]
    pub description: String,
    pub state: StateType,
    #[serde(
        rename = "due_on",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
}

impl CreateMilestoneOption {
    /// Validate the CreateMilestoneOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.title.trim().is_empty() {
            return Err(crate::Error::Validation("title is empty".to_string()));
        }
        Ok(())
    }
}

/// EditMilestoneOption options for editing a milestone
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
/// Options for Edit Milestone Option.
pub struct EditMilestoneOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<StateType>,
    #[serde(
        rename = "due_on",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
}

impl EditMilestoneOption {
    /// Validate the EditMilestoneOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if let Some(ref title) = self.title
            && title.trim().is_empty()
        {
            return Err(crate::Error::Validation("title is empty".to_string()));
        }
        Ok(())
    }
}

// ── issue_reaction.go ────────────────────────────────────────────

/// ListIssueReactionsOptions options for listing issue reactions
#[derive(Debug, Clone, Default)]
/// Options for List Issue Reactions Option.
pub struct ListIssueReactionsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListIssueReactionsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── issue_subscription.go ────────────────────────────────────────

/// ListIssueSubscribersOptions options for listing issue subscribers
#[derive(Debug, Clone, Default)]
/// Options for List Issue Subscribers Option.
pub struct ListIssueSubscribersOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListIssueSubscribersOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── issue_stopwatch.go ───────────────────────────────────────────

/// ListStopwatchesOptions options for listing stopwatches
#[derive(Debug, Clone, Default)]
/// Options for List Stopwatches Option.
pub struct ListStopwatchesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListStopwatchesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── issue_tracked_time.go ────────────────────────────────────────

/// ListTrackedTimesOptions options for listing repository's tracked times
#[derive(Debug, Clone, Default)]
/// Options for List Tracked Times Option.
pub struct ListTrackedTimesOptions {
    pub list_options: ListOptions,
    pub since: Option<OffsetDateTime>,
    pub before: Option<OffsetDateTime>,
    /// User filter is only used by ListRepoTrackedTimes
    pub user: String,
}

impl QueryEncode for ListTrackedTimesOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if let Some(since) = self.since {
            let formatted = since
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            out.push_str(&format!("&since={}", urlencoding(&formatted)));
        }
        if let Some(before) = self.before {
            let formatted = before
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_default();
            out.push_str(&format!("&before={}", urlencoding(&formatted)));
        }
        if !self.user.is_empty() {
            out.push_str(&format!("&user={}", urlencoding(&self.user)));
        }
        out
    }
}

/// AddTimeOption options for adding time to an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Add Time Option.
pub struct AddTimeOption {
    /// time in seconds
    pub time: i64,
    /// optional
    #[serde(
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub created: Option<OffsetDateTime>,
    /// optional
    #[serde(default, rename = "user_name")]
    pub user: String,
}

impl AddTimeOption {
    /// Validate the AddTimeOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.time == 0 {
            return Err(crate::Error::Validation("no time to add".to_string()));
        }
        Ok(())
    }
}

// ── issue_ext.go ─────────────────────────────────────────────────

/// ListIssueBlocksOptions options for listing issue blocks
#[derive(Debug, Clone, Default)]
/// Options for List Issue Blocks Option.
pub struct ListIssueBlocksOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListIssueBlocksOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListIssueDependenciesOptions options for listing issue dependencies
#[derive(Debug, Clone, Default)]
/// Options for List Issue Dependencies Option.
pub struct ListIssueDependenciesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListIssueDependenciesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// LockIssueOption represents options for locking an issue
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Lock Issue Option.
pub struct LockIssueOption {
    #[serde(default, rename = "lock_reason")]
    pub lock_reason: String,
}

/// EditDeadlineOption represents options for updating issue deadline
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Edit Deadline Option.
pub struct EditDeadlineOption {
    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_issue_option_validate_success() {
        let opt = CreateIssueOption {
            title: "bug report".to_string(),
            body: String::new(),
            r#ref: String::new(),
            assignees: Vec::new(),
            deadline: None,
            milestone: 0,
            labels: Vec::new(),
            closed: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_issue_option_validate_empty_title() {
        let opt = CreateIssueOption {
            title: String::new(),
            body: String::new(),
            r#ref: String::new(),
            assignees: Vec::new(),
            deadline: None,
            milestone: 0,
            labels: Vec::new(),
            closed: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_issue_option_validate_whitespace_title() {
        let opt = CreateIssueOption {
            title: "   ".to_string(),
            body: String::new(),
            r#ref: String::new(),
            assignees: Vec::new(),
            deadline: None,
            milestone: 0,
            labels: Vec::new(),
            closed: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_issue_option_validate_success() {
        let opt = EditIssueOption {
            title: Some("new title".to_string()),
            body: None,
            r#ref: None,
            assignees: Vec::new(),
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_issue_option_validate_empty_title() {
        let opt = EditIssueOption {
            title: Some("   ".to_string()),
            body: None,
            r#ref: None,
            assignees: Vec::new(),
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_issue_comment_option_validate_success() {
        let opt = CreateIssueCommentOption {
            body: "comment".to_string(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_issue_comment_option_validate_empty_body() {
        let opt = CreateIssueCommentOption {
            body: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_issue_comment_option_validate_success() {
        let opt = EditIssueCommentOption {
            body: "updated comment".to_string(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_issue_comment_option_validate_empty_body() {
        let opt = EditIssueCommentOption {
            body: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_milestone_option_validate_success() {
        let opt = CreateMilestoneOption {
            title: "v1.0".to_string(),
            description: String::new(),
            state: StateType::Open,
            deadline: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_milestone_option_validate_empty_title() {
        let opt = CreateMilestoneOption {
            title: String::new(),
            description: String::new(),
            state: StateType::Open,
            deadline: None,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_edit_milestone_option_validate_success() {
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_milestone_option_validate_empty_title() {
        let opt = EditMilestoneOption {
            title: Some("   ".to_string()),
            ..Default::default()
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_add_time_option_validate_success() {
        let opt = AddTimeOption {
            time: 3600,
            created: None,
            user: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_add_time_option_validate_zero_time() {
        let opt = AddTimeOption {
            time: 0,
            created: None,
            user: String::new(),
        };
        assert!(opt.validate().is_err());
    }
}
