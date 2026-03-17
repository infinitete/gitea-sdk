// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::{MergeStyle, ReviewStateType, StateType};
use crate::{Deserialize, Serialize};

// ── pull.go ─────────────────────────────────────────────────────

/// ListPullRequestsOptions options for listing pull requests
#[derive(Debug, Clone)]
pub struct ListPullRequestsOptions {
    pub list_options: ListOptions,
    pub state: StateType,
    /// oldest, recentupdate, leastupdate, mostcomment, leastcomment, priority
    pub sort: String,
    pub milestone: i64,
}

impl Default for ListPullRequestsOptions {
    fn default() -> Self {
        Self {
            list_options: ListOptions::default(),
            state: StateType::All,
            sort: String::new(),
            milestone: 0,
        }
    }
}

impl QueryEncode for ListPullRequestsOptions {
    fn query_encode(&self) -> String {
        let mut out = self.list_options.query_encode();
        if !matches!(self.state, StateType::All) {
            out.push_str(&format!("&state={}", self.state));
        }
        if !self.sort.is_empty() {
            out.push_str(&format!("&sort={}", self.sort));
        }
        if self.milestone > 0 {
            out.push_str(&format!("&milestone={}", self.milestone));
        }
        out
    }
}

/// CreatePullRequestOption options when creating a pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullRequestOption {
    pub head: String,
    pub base: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_reviewers: Vec<String>,
    #[serde(default)]
    pub milestone: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<i64>,
    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<time::OffsetDateTime>,
}

mod nullable_rfc3339_option {
    use serde::{Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn serialize<S>(opt: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(dt) => {
                let formatted = dt
                    .format(&time::format_description::well_known::Rfc3339)
                    .map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&formatted)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::Deserialize;
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            None => Ok(None),
            Some(ref s) => {
                let dt = OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339)
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(dt))
            }
        }
    }
}

/// EditPullRequestOption options when modify pull request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditPullRequestOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub assignees: Vec<String>,
    #[serde(default)]
    pub milestone: i64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<StateType>,
    #[serde(
        rename = "due_date",
        default,
        with = "nullable_rfc3339_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub deadline: Option<time::OffsetDateTime>,
    #[serde(rename = "unset_due_date", skip_serializing_if = "Option::is_none")]
    pub remove_deadline: Option<bool>,
    #[serde(
        rename = "allow_maintainer_edit",
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_maintainer_edit: Option<bool>,
}

impl EditPullRequestOption {
    /// Validate the EditPullRequestOption struct
    pub fn validate(&self) -> crate::Result<()> {
        if let Some(ref title) = self.title
            && title.trim().is_empty()
        {
            return Err(crate::Error::Validation("title is empty".to_string()));
        }
        // TODO: version gate for base change (requires Gitea >= 1.12)
        Ok(())
    }
}

/// MergePullRequestOption options when merging a pull request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MergePullRequestOption {
    #[serde(rename = "Do", skip_serializing_if = "Option::is_none")]
    pub style: Option<MergeStyle>,
    #[serde(rename = "MergeCommitID", skip_serializing_if = "Option::is_none")]
    pub merge_commit_id: Option<String>,
    #[serde(rename = "MergeTitleField", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "MergeMessageField", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "delete_branch_after_merge")]
    pub delete_branch_after_merge: bool,
    #[serde(rename = "force_merge")]
    pub force_merge: bool,
    #[serde(rename = "head_commit_id", skip_serializing_if = "Option::is_none")]
    pub head_commit_id: Option<String>,
    #[serde(rename = "merge_when_checks_succeed")]
    pub merge_when_checks_succeed: bool,
}

/// PullRequestDiffOptions options for GET `/repos/<owner>/<repo>/pulls/<idx>.[diff|patch]`
#[derive(Debug, Clone, Default)]
pub struct PullRequestDiffOptions {
    /// Include binary file changes when requesting a .diff
    pub binary: bool,
}

impl QueryEncode for PullRequestDiffOptions {
    fn query_encode(&self) -> String {
        format!("binary={}", self.binary)
    }
}

/// ListPullRequestCommitsOptions options for listing pull request commits
#[derive(Debug, Clone, Default)]
pub struct ListPullRequestCommitsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPullRequestCommitsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// ListPullRequestFilesOptions options for listing pull request files
#[derive(Debug, Clone, Default)]
pub struct ListPullRequestFilesOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPullRequestFilesOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

// ── pull_review.go ───────────────────────────────────────────────

/// ListPullReviewsOptions options for listing PullReviews
#[derive(Debug, Clone, Default)]
pub struct ListPullReviewsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListPullReviewsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// CreatePullReviewOptions are options to create a pull review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullReviewOptions {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub state: Option<ReviewStateType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(rename = "commit_id", skip_serializing_if = "Option::is_none")]
    pub commit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub comments: Vec<CreatePullReviewComment>,
}

impl CreatePullReviewOptions {
    /// Validate the CreatePullReviewOptions struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.state != Some(ReviewStateType::Approved)
            && self.comments.is_empty()
            && self.body.as_ref().is_some_and(|b| b.trim().is_empty())
        {
            return Err(crate::Error::Validation("body is empty".to_string()));
        }
        for comment in &self.comments {
            comment.validate()?;
        }
        Ok(())
    }
}

/// CreatePullReviewComment represent a review comment for creation api
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePullReviewComment {
    /// the tree path
    pub path: String,
    pub body: String,
    /// if comment to old file line or 0
    #[serde(rename = "old_position")]
    pub old_line_num: i64,
    /// if comment to new file line or 0
    #[serde(rename = "new_position")]
    pub new_line_num: i64,
}

impl CreatePullReviewComment {
    /// Validate the CreatePullReviewComment struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.body.trim().is_empty() {
            return Err(crate::Error::Validation("body is empty".to_string()));
        }
        if self.old_line_num != 0 && self.new_line_num != 0 {
            return Err(crate::Error::Validation(
                "old and new line num are set, cant identify the code comment position".to_string(),
            ));
        }
        Ok(())
    }
}

/// SubmitPullReviewOptions are options to submit a pending pull review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitPullReviewOptions {
    #[serde(rename = "event", skip_serializing_if = "Option::is_none")]
    pub state: Option<ReviewStateType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl SubmitPullReviewOptions {
    /// Validate the SubmitPullReviewOptions struct
    pub fn validate(&self) -> crate::Result<()> {
        if self.state != Some(ReviewStateType::Approved)
            && self.body.as_ref().is_some_and(|b| b.trim().is_empty())
        {
            return Err(crate::Error::Validation("body is empty".to_string()));
        }
        Ok(())
    }
}

/// DismissPullReviewOptions are options to dismiss a pull review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DismissPullReviewOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// PullReviewRequestOptions are options to add or remove pull review requests
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PullReviewRequestOptions {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reviewers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub team_reviewers: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_pull_request_option_validate_success() {
        let opt = EditPullRequestOption {
            title: Some("new title".to_string()),
            ..Default::default()
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_edit_pull_request_option_validate_empty_title() {
        let opt = EditPullRequestOption {
            title: Some("   ".to_string()),
            ..Default::default()
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_pull_review_options_validate_approved() {
        let opt = CreatePullReviewOptions {
            state: Some(ReviewStateType::Approved),
            body: None,
            commit_id: None,
            comments: Vec::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_pull_review_options_validate_empty_body() {
        let opt = CreatePullReviewOptions {
            state: None,
            body: Some("   ".to_string()),
            commit_id: None,
            comments: Vec::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_pull_review_options_validate_with_comments() {
        let opt = CreatePullReviewOptions {
            state: None,
            body: Some("   ".to_string()),
            commit_id: None,
            comments: vec![CreatePullReviewComment {
                path: "main.rs".to_string(),
                body: "fix this".to_string(),
                old_line_num: 0,
                new_line_num: 10,
            }],
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_pull_review_comment_validate_success() {
        let comment = CreatePullReviewComment {
            path: "main.rs".to_string(),
            body: "fix this".to_string(),
            old_line_num: 0,
            new_line_num: 10,
        };
        assert!(comment.validate().is_ok());
    }

    #[test]
    fn test_create_pull_review_comment_validate_empty_body() {
        let comment = CreatePullReviewComment {
            path: "main.rs".to_string(),
            body: String::new(),
            old_line_num: 0,
            new_line_num: 10,
        };
        assert!(comment.validate().is_err());
    }

    #[test]
    fn test_create_pull_review_comment_validate_both_lines_set() {
        let comment = CreatePullReviewComment {
            path: "main.rs".to_string(),
            body: "fix this".to_string(),
            old_line_num: 5,
            new_line_num: 10,
        };
        assert!(comment.validate().is_err());
    }

    #[test]
    fn test_submit_pull_review_options_validate_approved() {
        let opt = SubmitPullReviewOptions {
            state: Some(ReviewStateType::Approved),
            body: None,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_submit_pull_review_options_validate_empty_body() {
        let opt = SubmitPullReviewOptions {
            state: None,
            body: Some("   ".to_string()),
        };
        assert!(opt.validate().is_err());
    }
}
