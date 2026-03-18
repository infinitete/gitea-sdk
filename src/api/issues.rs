// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Issue API endpoints for managing Gitea issues, labels, and milestones.

use crate::Client;
use crate::Response;
use crate::internal::request::{json_body, json_header};
use crate::options::issue::*;
use crate::pagination::QueryEncode;
use crate::types::{
    Attachment, Comment, Issue, IssueMeta, IssueTemplate, Milestone, StopWatch, TimelineComment,
    TrackedTime, WatchInfo,
};

/// API methods for issues. Access via [`Client::issues()`](crate::Client::issues).
pub struct IssuesApi<'a> {
    client: &'a Client,
}

impl<'a> IssuesApi<'a> {
    /// Create a new `IssuesApi` view.
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub(crate) fn client(&self) -> &'a Client {
        self.client
    }

    // ── issue.go ──────────────────────────────────────────────────
    // 6 methods

    /// ListIssues returns all issues assigned the authenticated user
    pub async fn list_issues(&self, opt: ListIssueOption) -> crate::Result<(Vec<Issue>, Response)> {
        let path = format!("/repos/issues/search?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListRepoIssues returns all issues for a given repository
    pub async fn list_repo_issues(
        &self,
        owner: &str,
        repo: &str,
        opt: ListIssueOption,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssue returns a single issue for a given repository
    pub async fn get_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateIssue create a new issue for a given repository
    pub async fn create_issue(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateIssueOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditIssue modify an existing issue for a given repository
    pub async fn edit_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: EditIssueOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssue delete an issue from a repository
    pub async fn delete_issue(&self, owner: &str, repo: &str, id: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── issue_comment.go ──────────────────────────────────────────
    // 10 methods

    /// ListIssueComments list comments on an issue
    pub async fn list_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueCommentOptions,
    ) -> crate::Result<(Vec<Comment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/comments?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// ListRepoIssueComments list comments for a given repo
    pub async fn list_repo_issue_comments(
        &self,
        owner: &str,
        repo: &str,
        opt: ListIssueCommentOptions,
    ) -> crate::Result<(Vec<Comment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssueComment get a comment for a given repo by id
    pub async fn get_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Comment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/comments/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateIssueComment create comment on an issue
    pub async fn create_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: CreateIssueCommentOption,
    ) -> crate::Result<(Comment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/comments",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditIssueComment edits an issue comment
    pub async fn edit_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        opt: EditIssueCommentOption,
    ) -> crate::Result<(Comment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssueComment deletes an issue comment
    pub async fn delete_issue_comment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// ListIssueCommentAttachments lists all attachments for a comment
    pub async fn list_issue_comment_attachments(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> crate::Result<(Vec<Attachment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssueCommentAttachment gets a comment attachment
    pub async fn get_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        attachment_id: i64,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets/{attachment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// EditIssueCommentAttachment updates a comment attachment
    pub async fn edit_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        attachment_id: i64,
        form: crate::options::release::EditAttachmentOption,
    ) -> crate::Result<(Attachment, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        form.validate()?;
        let body = json_body(&form)?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets/{attachment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssueCommentAttachment deletes a comment attachment
    pub async fn delete_issue_comment_attachment(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        attachment_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/assets/{attachment_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── issue_label.go ────────────────────────────────────────────
    // 5 methods

    /// GetIssueLabels get labels of one issue via issue id
    pub async fn get_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: crate::ListOptions,
    ) -> crate::Result<(Vec<crate::Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/labels?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddIssueLabels add one or more labels to one issue
    pub async fn add_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueLabelsOption,
    ) -> crate::Result<(Vec<crate::Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/labels", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ReplaceIssueLabels replace old labels of issue with new labels
    pub async fn replace_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueLabelsOption,
    ) -> crate::Result<(Vec<crate::Label>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/labels", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssueLabel delete one label of one issue by issue id and label id
    pub async fn delete_issue_label(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        label: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/labels/{label}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// ClearIssueLabels delete all the labels of one issue
    pub async fn clear_issue_labels(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/labels", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── issue_milestone.go ────────────────────────────────────────
    // 8 methods

    /// ListRepoMilestones list all the milestones of one repository
    pub async fn list_repo_milestones(
        &self,
        owner: &str,
        repo: &str,
        opt: ListMilestoneOption,
    ) -> crate::Result<(Vec<Milestone>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/milestones?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetMilestone get one milestone by repo name and milestone id
    pub async fn get_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/milestones/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetMilestoneByName get one milestone by repo and milestone name
    pub async fn get_milestone_by_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// CreateMilestone create one milestone with options
    pub async fn create_milestone(
        &self,
        owner: &str,
        repo: &str,
        opt: CreateMilestoneOption,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/milestones", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditMilestone modify milestone with options
    pub async fn edit_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
        opt: EditMilestoneOption,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/milestones/{id}", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// EditMilestoneByName modify milestone with options
    pub async fn edit_milestone_by_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
        opt: EditMilestoneOption,
    ) -> crate::Result<(Milestone, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteMilestone delete one milestone by id
    pub async fn delete_milestone(
        &self,
        owner: &str,
        repo: &str,
        id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/milestones/{id}", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// DeleteMilestoneByName delete one milestone by name
    pub async fn delete_milestone_by_name(
        &self,
        owner: &str,
        repo: &str,
        name: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, name])?;
        let path = format!(
            "/repos/{}/{}/milestones/{}",
            escaped[0], escaped[1], escaped[2]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    // ── issue_reaction.go ─────────────────────────────────────────
    // 6 methods (excluding deprecated GetIssueReactions)

    /// ListIssueReactions get a list of reactions for an issue with pagination
    pub async fn list_issue_reactions(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueReactionsOptions,
    ) -> crate::Result<(Vec<crate::Reaction>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/reactions?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// GetIssueCommentReactions get a list of reactions from a comment of an issue
    pub async fn get_issue_comment_reactions(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
    ) -> crate::Result<(Vec<crate::Reaction>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// PostIssueReaction add a reaction to an issue
    pub async fn post_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        reaction: &str,
    ) -> crate::Result<(crate::Reaction, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssueReaction remove a reaction from an issue
    pub async fn delete_issue_reaction(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        reaction: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// PostIssueCommentReaction add a reaction to a comment of an issue
    pub async fn post_issue_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction: &str,
    ) -> crate::Result<(crate::Reaction, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// DeleteIssueCommentReaction remove a reaction from a comment of an issue
    pub async fn delete_issue_comment_reaction(
        &self,
        owner: &str,
        repo: &str,
        comment_id: i64,
        reaction: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        #[derive(serde::Serialize)]
        struct ReactionBody {
            content: String,
        }
        let body = json_body(&ReactionBody {
            content: reaction.to_string(),
        })?;
        let path = format!(
            "/repos/{}/{}/issues/comments/{comment_id}/reactions",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    // ── issue_subscription.go ─────────────────────────────────────
    // 7 methods (excluding deprecated GetIssueSubscribers)

    /// ListIssueSubscribers get list of users who subscribed on an issue with pagination
    pub async fn list_issue_subscribers(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueSubscribersOptions,
    ) -> crate::Result<(Vec<crate::User>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// AddIssueSubscription subscribe user to issue
    pub async fn add_issue_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        user: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, user])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::PUT, &path, None, None::<&str>)
            .await?;
        if status == 201 {
            return Ok(resp);
        }
        if status == 200 {
            return Err(crate::Error::Validation("already subscribed".to_string()));
        }
        Err(crate::Error::UnknownApi {
            status,
            body: format!("unexpected status: {status}"),
        })
    }

    /// DeleteIssueSubscription unsubscribe user from issue
    pub async fn delete_issue_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        user: &str,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo, user])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions/{}",
            escaped[0], escaped[1], escaped[2]
        );
        let (status, resp) = self
            .client()
            .get_status_code(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await?;
        if status == 201 {
            return Ok(resp);
        }
        if status == 200 {
            return Err(crate::Error::Validation("already unsubscribed".to_string()));
        }
        Err(crate::Error::UnknownApi {
            status,
            body: format!("unexpected status: {status}"),
        })
    }

    /// CheckIssueSubscription check if current user is subscribed to an issue
    pub async fn check_issue_subscription(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<(WatchInfo, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/subscriptions/check",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// IssueSubscribe subscribe current user to an issue
    pub async fn issue_subscribe(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let (user, _) = self.client().users().get_my_info().await?;
        self.add_issue_subscription(owner, repo, index, &user.user_name)
            .await
    }

    /// IssueUnsubscribe unsubscribe current user from an issue
    pub async fn issue_unsubscribe(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let (user, _) = self.client().users().get_my_info().await?;
        self.delete_issue_subscription(owner, repo, index, &user.user_name)
            .await
    }

    // ── issue_stopwatch.go ────────────────────────────────────────
    // 4 methods (excluding deprecated GetMyStopwatches)

    /// ListMyStopwatches list all stopwatches with pagination
    pub async fn list_my_stopwatches(
        &self,
        opt: ListStopwatchesOptions,
    ) -> crate::Result<(Vec<StopWatch>, Response)> {
        let path = format!("/user/stopwatches?{}", opt.query_encode());
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    /// DeleteIssueStopwatch delete / cancel a specific stopwatch
    pub async fn delete_issue_stopwatch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/stopwatch/delete",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::DELETE, &path, None, None::<&str>)
            .await
    }

    /// StartIssueStopWatch starts a stopwatch for an existing issue
    pub async fn start_issue_stopwatch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/stopwatch/start",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    /// StopIssueStopWatch stops an existing stopwatch for an issue
    pub async fn stop_issue_stopwatch(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/stopwatch/stop",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(reqwest::Method::POST, &path, None, None::<&str>)
            .await
    }

    // ── issue_tracked_time.go ─────────────────────────────────────
    // 7 methods (excluding deprecated GetMyTrackedTimes)

    /// ListRepoTrackedTimes list tracked times of a repository
    pub async fn list_repo_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        opt: ListTrackedTimesOptions,
    ) -> crate::Result<(Vec<TrackedTime>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/times?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ListMyTrackedTimes list tracked times of the current user with pagination and filtering
    pub async fn list_my_tracked_times(
        &self,
        opt: ListTrackedTimesOptions,
    ) -> crate::Result<(Vec<TrackedTime>, Response)> {
        let path = format!("/user/times?{}", opt.query_encode());
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// AddTime adds time to issue with the given index
    pub async fn add_time(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: AddTimeOption,
    ) -> crate::Result<(TrackedTime, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        opt.validate()?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/times", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListIssueTrackedTimes list tracked times of a single issue
    pub async fn list_issue_tracked_times(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListTrackedTimesOptions,
    ) -> crate::Result<(Vec<TrackedTime>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/times?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// ResetIssueTime reset tracked time of a single issue
    pub async fn reset_issue_time(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/times", escaped[0], escaped[1]);
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// DeleteTime delete a specific tracked time by id
    pub async fn delete_time(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        time_id: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/times/{time_id}",
            escaped[0], escaped[1]
        );
        self.client()
            .do_request_with_status_handle(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    // ── issue_timeline.go ─────────────────────────────────────────
    // 1 method

    /// ListIssueTimeline list timeline on an issue
    pub async fn list_issue_timeline(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueCommentOptions,
    ) -> crate::Result<(Vec<TimelineComment>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/timeline?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }

    // ── issue_pin.go ──────────────────────────────────────────────
    // 4 methods

    /// ListRepoPinnedIssues lists a repo's pinned issues
    pub async fn list_repo_pinned_issues(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/pinned", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// PinIssue pins an issue
    pub async fn pin_issue(&self, owner: &str, repo: &str, index: i64) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/pin", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    /// UnpinIssue unpins an issue
    pub async fn unpin_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/pin", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    /// MoveIssuePin moves a pinned issue to the given position
    pub async fn move_issue_pin(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        position: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/pin/{position}",
            escaped[0], escaped[1]
        );
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::PATCH,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    // ── issue_ext.go ──────────────────────────────────────────────
    // 9 methods

    /// ListIssueBlocks lists issues that are blocked by the specified issue
    pub async fn list_issue_blocks(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueBlocksOptions,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/blocks?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CreateIssueBlocking blocks an issue with another issue
    pub async fn create_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/blocks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// RemoveIssueBlocking removes an issue block
    pub async fn remove_issue_blocking(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/blocks", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// ListIssueDependencies lists issues that block the specified issue (its dependencies)
    pub async fn list_issue_dependencies(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: ListIssueDependenciesOptions,
    ) -> crate::Result<(Vec<Issue>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/dependencies?{}",
            escaped[0],
            escaped[1],
            opt.query_encode()
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::GET,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await
    }

    /// CreateIssueDependency creates a new issue dependency
    pub async fn create_issue_dependency(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/dependencies",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// RemoveIssueDependency removes an issue dependency
    pub async fn remove_issue_dependency(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: IssueMeta,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/dependencies",
            escaped[0], escaped[1]
        );
        self.client()
            .get_parsed_response(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
    }

    /// LockIssue locks an issue
    pub async fn lock_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: LockIssueOption,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!("/repos/{}/{}/issues/{index}/lock", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::PUT,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    /// UnlockIssue unlocks an issue
    pub async fn unlock_issue(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
    ) -> crate::Result<Response> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issues/{index}/lock", escaped[0], escaped[1]);
        let (status, resp) = self
            .client()
            .get_status_code(
                reqwest::Method::DELETE,
                &path,
                Some(&json_header()),
                None::<&str>,
            )
            .await?;
        if status != 204 {
            return Err(crate::Error::UnknownApi {
                status,
                body: format!("unexpected status: {status}"),
            });
        }
        Ok(resp)
    }

    /// UpdateIssueDeadline updates an issue's deadline
    pub async fn update_issue_deadline(
        &self,
        owner: &str,
        repo: &str,
        index: i64,
        opt: EditDeadlineOption,
    ) -> crate::Result<(Issue, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let body = json_body(&opt)?;
        let path = format!(
            "/repos/{}/{}/issues/{index}/deadline",
            escaped[0], escaped[1]
        );
        let (data, resp) = self
            .client()
            .get_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await?;
        if let Ok(issue) = serde_json::from_slice::<Issue>(&data) {
            return Ok((issue, resp));
        }
        if serde_json::from_slice::<serde_json::Value>(&data)
            .ok()
            .and_then(|value| value.get("due_date").cloned())
            .is_some()
        {
            let (issue, _) = self.get_issue(owner, repo, index).await?;
            return Ok((issue, resp));
        }
        Err(crate::Error::Json(
            serde_json::from_slice::<Issue>(&data).unwrap_err(),
        ))
    }

    // ── issue_template.go ─────────────────────────────────────────
    // 1 method

    /// GetIssueTemplates lists all issue templates of the repository
    pub async fn get_issue_templates(
        &self,
        owner: &str,
        repo: &str,
    ) -> crate::Result<(Vec<IssueTemplate>, Response)> {
        let escaped = crate::internal::escape::validate_and_escape_segments(&[owner, repo])?;
        let path = format!("/repos/{}/{}/issue_templates", escaped[0], escaped[1]);
        self.client()
            .get_parsed_response(reqwest::Method::GET, &path, None, None::<&str>)
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path, path_regex};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn create_test_client(server: &MockServer) -> Client {
        Client::builder(&server.uri())
            .token("test-token")
            .gitea_version("")
            .build()
            .unwrap()
    }

    fn issue_json(id: i64, number: i64, title: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "url": "",
            "html_url": "",
            "number": number,
            "user": {
                "id": 1,
                "login": "testuser",
                "login_name": "",
                "source_id": 0,
                "full_name": "",
                "email": "",
                "avatar_url": "",
                "html_url": "",
                "language": "",
                "is_admin": false,
                "restricted": false,
                "active": true,
                "prohibit_login": false,
                "location": "",
                "website": "",
                "description": "",
                "visibility": "public",
                "followers_count": 0,
                "following_count": 0,
                "starred_repos_count": 0
            },
            "original_author": "",
            "original_author_id": 0,
            "title": title,
            "body": "Issue body",
            "ref": "",
            "labels": [],
            "milestone": null,
            "assignees": [],
            "state": "open",
            "is_locked": false,
            "comments": 0,
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T10:00:00Z",
            "closed_at": null,
            "due_date": null,
            "pull_request": null,
            "repository": null
        })
    }

    fn comment_json(id: i64, body: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "html_url": "",
            "pull_request_url": "",
            "issue_url": "",
            "poster": {
                "id": 1,
                "login": "testuser",
                "login_name": "",
                "source_id": 0,
                "full_name": "",
                "email": "",
                "avatar_url": "",
                "html_url": "",
                "language": "",
                "is_admin": false,
                "restricted": false,
                "active": true,
                "prohibit_login": false,
                "location": "",
                "website": "",
                "description": "",
                "visibility": "public",
                "followers_count": 0,
                "following_count": 0,
                "starred_repos_count": 0
            },
            "original_author": "",
            "original_author_id": 0,
            "body": body,
            "created": "2024-01-15T10:00:00Z",
            "updated": "2024-01-15T10:00:00Z"
        })
    }

    fn label_json(id: i64, name: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": name,
            "color": "ff0000",
            "description": "",
            "exclusive": false,
            "is_archived": false,
            "url": ""
        })
    }

    fn milestone_json(id: i64, title: &str) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "title": title,
            "description": "",
            "state": "open",
            "open_issues": 0,
            "closed_issues": 0,
            "created_at": "2024-01-15T10:00:00Z",
            "updated_at": "2024-01-15T10:00:00Z",
            "closed_at": null,
            "due_on": null
        })
    }

    fn reaction_json(content: &str) -> serde_json::Value {
        serde_json::json!({
            "user": null,
            "reaction": content,
            "created": "2024-01-15T10:00:00Z"
        })
    }

    fn user_json() -> serde_json::Value {
        serde_json::json!({
            "id": 1,
            "login": "testuser",
            "login_name": "",
            "source_id": 0,
            "full_name": "",
            "email": "",
            "avatar_url": "",
            "html_url": "",
            "language": "",
            "is_admin": false,
            "restricted": false,
            "active": true,
            "prohibit_login": false,
            "location": "",
            "website": "",
            "description": "",
            "visibility": "public",
            "followers_count": 0,
            "following_count": 0,
            "starred_repos_count": 0
        })
    }

    fn attachment_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "name": "file.txt",
            "size": 1024,
            "download_count": 0,
            "created": "2024-01-15T10:00:00Z",
            "uuid": "abc123",
            "browser_download_url": ""
        })
    }

    fn stopwatch_json() -> serde_json::Value {
        serde_json::json!({
            "created": "2024-01-15T10:00:00Z",
            "seconds": 3600,
            "duration": "1h0m0s",
            "issue_index": 1,
            "issue_title": "Bug fix",
            "repo_owner_name": "owner",
            "repo_name": "repo"
        })
    }

    fn tracked_time_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "created": "2024-01-15T10:00:00Z",
            "time": 1800,
            "user_id": 0,
            "user_name": "",
            "issue_id": 0
        })
    }

    fn timeline_comment_json(id: i64) -> serde_json::Value {
        serde_json::json!({
            "id": id,
            "html_url": "",
            "pull_request_url": "",
            "issue_url": "",
            "user": null,
            "original_author": "",
            "original_author_id": 0,
            "body": "comment",
            "created": "2024-01-15T10:00:00Z",
            "updated": "2024-01-15T10:00:00Z",
            "type": "comment",
            "label": [],
            "milestone": null,
            "old_milestone": null,
            "new_title": "",
            "old_title": ""
        })
    }

    fn watch_info_json() -> serde_json::Value {
        serde_json::json!({
            "subscribed": true,
            "watching": true,
            "ignored": false,
            "reason": "",
            "created_at": null,
            "url": "",
            "repository_url": ""
        })
    }

    fn issue_template_json() -> serde_json::Value {
        serde_json::json!({
            "name": "Bug Report",
            "about": "File a bug",
            "file_name": "bug_report.md",
            "title": "Bug: ",
            "labels": ["bug"],
            "ref": "",
            "form": [],
            "content": "Describe the bug..."
        })
    }

    fn error_body() -> serde_json::Value {
        serde_json::json!({"message": "error"})
    }

    // ── issue.go ──────────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issues_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/issues/search"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(1, 1, "Issue 1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_issues(Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issues_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/issues/search"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().list_issues(Default::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_issues_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(1, 1, "Issue 1")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_repo_issues("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_issues_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_issues("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(1, 1, "Bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issue, resp) = client
            .issues()
            .get_issue("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue("testowner", "testrepo", 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(1, 1, "Bug fix")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueOption {
            title: "Bug fix".to_string(),
            body: "Fix the bug".to_string(),
            r#ref: String::new(),
            assignees: vec![],
            deadline: None,
            milestone: 0,
            labels: vec![],
            closed: false,
        };
        let (issue, resp) = client
            .issues()
            .create_issue("testowner", "testrepo", opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueOption {
            title: "Bug fix".to_string(),
            body: String::new(),
            r#ref: String::new(),
            assignees: vec![],
            deadline: None,
            milestone: 0,
            labels: vec![],
            closed: false,
        };
        let result = client
            .issues()
            .create_issue("testowner", "testrepo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateIssueOption {
            title: "  ".to_string(),
            body: String::new(),
            r#ref: String::new(),
            assignees: vec![],
            deadline: None,
            milestone: 0,
            labels: vec![],
            closed: false,
        };
        let result = client
            .issues()
            .create_issue("testowner", "testrepo", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(1, 1, "Updated")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("Updated".to_string()),
            body: None,
            r#ref: None,
            assignees: vec![],
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        let (issue, resp) = client
            .issues()
            .edit_issue("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.title, "Updated");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("Updated".to_string()),
            body: None,
            r#ref: None,
            assignees: vec![],
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        let result = client
            .issues()
            .edit_issue("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("   ".to_string()),
            body: None,
            r#ref: None,
            assignees: vec![],
            milestone: None,
            state: None,
            deadline: None,
            remove_deadline: None,
        };
        let result = client
            .issues()
            .edit_issue("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue("testowner", "testrepo", 1)
            .await;
        assert!(result.is_err());
    }

    // ── issue_comment.go ──────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_comments_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/comments"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([comment_json(1, "Nice")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comments, resp) = client
            .issues()
            .list_issue_comments("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_comments_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/comments"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_comments("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_repo_issue_comments_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/comments"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([comment_json(1, "comment")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comments, resp) = client
            .issues()
            .list_repo_issue_comments("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_issue_comments_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/comments"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_issue_comments("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(comment_json(1, "hello")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comment, resp) = client
            .issues()
            .get_issue_comment("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(comment.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/999"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_comment("testowner", "testrepo", 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(comment_json(1, "Nice issue")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueCommentOption {
            body: "Nice issue".to_string(),
        };
        let (comment, resp) = client
            .issues()
            .create_issue_comment("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(comment.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1/comments"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateIssueCommentOption {
            body: "body".to_string(),
        };
        let result = client
            .issues()
            .create_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_comment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateIssueCommentOption {
            body: String::new(),
        };
        let result = client
            .issues()
            .create_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(comment_json(1, "updated")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueCommentOption {
            body: "updated".to_string(),
        };
        let (comment, resp) = client
            .issues()
            .edit_issue_comment("testowner", "testrepo", 1, opt)
            .await
            .unwrap();
        assert_eq!(comment.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditIssueCommentOption {
            body: "updated".to_string(),
        };
        let result = client
            .issues()
            .edit_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = EditIssueCommentOption {
            body: String::new(),
        };
        let result = client
            .issues()
            .edit_issue_comment("testowner", "testrepo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_comment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_comment("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_comment_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/comments/1"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_comment("testowner", "testrepo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_issue_comment_attachments_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([attachment_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (attachments, resp) = client
            .issues()
            .list_issue_comment_attachments("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(attachments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_comment_attachments_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_comment_attachments("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_comment_attachment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(attachment_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (att, resp) = client
            .issues()
            .get_issue_comment_attachment("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(att.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_comment_attachment_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_comment_attachment("owner", "repo", 1, 999)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_attachment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(attachment_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let form = crate::options::release::EditAttachmentOption {
            name: "new_name.txt".to_string(),
        };
        let (att, resp) = client
            .issues()
            .edit_issue_comment_attachment("owner", "repo", 1, 1, form)
            .await
            .unwrap();
        assert_eq!(att.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_issue_comment_attachment_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let form = crate::options::release::EditAttachmentOption {
            name: "new_name.txt".to_string(),
        };
        let result = client
            .issues()
            .edit_issue_comment_attachment("owner", "repo", 1, 1, form)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_issue_comment_attachment_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let form = crate::options::release::EditAttachmentOption {
            name: "  ".to_string(),
        };
        let result = client
            .issues()
            .edit_issue_comment_attachment("owner", "repo", 1, 1, form)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_comment_attachment_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_comment_attachment("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_comment_attachment_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/assets/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_comment_attachment("owner", "repo", 1, 1)
            .await;
        assert!(result.is_err());
    }

    // ── issue_label.go ────────────────────────────────────────────

    #[tokio::test]
    async fn test_get_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([label_json(1, "bug")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (labels, resp) = client
            .issues()
            .get_issue_labels("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_labels("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([label_json(1, "bug")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![1] };
        let (labels, resp) = client
            .issues()
            .add_issue_labels("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_add_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![1] };
        let result = client
            .issues()
            .add_issue_labels("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_replace_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([label_json(2, "feature")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![2] };
        let (labels, resp) = client
            .issues()
            .replace_issue_labels("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(labels.len(), 1);
        assert_eq!(labels[0].name, "feature");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_replace_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueLabelsOption { labels: vec![2] };
        let result = client
            .issues()
            .replace_issue_labels("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_label_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_label("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_label_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_label("owner", "repo", 1, 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_clear_issue_labels_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .clear_issue_labels("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_clear_issue_labels_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/labels"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().clear_issue_labels("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    // ── issue_milestone.go ────────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_milestones_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([milestone_json(1, "v1.0")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (milestones, resp) = client
            .issues()
            .list_repo_milestones("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(milestones.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_milestones_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_milestones("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v1.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (ms, resp) = client
            .issues()
            .get_milestone("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(ms.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().get_milestone("owner", "repo", 999).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_milestone_by_name_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v1.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (ms, resp) = client
            .issues()
            .get_milestone_by_name("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(ms.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_milestone_by_name_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_milestone_by_name("owner", "repo", "v1.0")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(ResponseTemplate::new(201).set_body_json(milestone_json(1, "v1.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateMilestoneOption {
            title: "v1.0".to_string(),
            description: String::new(),
            state: crate::StateType::Open,
            deadline: None,
        };
        let (ms, resp) = client
            .issues()
            .create_milestone("owner", "repo", opt)
            .await
            .unwrap();
        assert_eq!(ms.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = CreateMilestoneOption {
            title: "v1.0".to_string(),
            description: String::new(),
            state: crate::StateType::Open,
            deadline: None,
        };
        let result = client.issues().create_milestone("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_milestone_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = CreateMilestoneOption {
            title: String::new(),
            description: String::new(),
            state: crate::StateType::Open,
            deadline: None,
        };
        let result = client.issues().create_milestone("owner", "repo", opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v2.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let (ms, resp) = client
            .issues()
            .edit_milestone("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(ms.title, "v2.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let result = client
            .issues()
            .edit_milestone("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_edit_milestone_by_name_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(200).set_body_json(milestone_json(1, "v2.0")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let (ms, resp) = client
            .issues()
            .edit_milestone_by_name("owner", "repo", "v1.0", opt)
            .await
            .unwrap();
        assert_eq!(ms.title, "v2.0");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_edit_milestone_by_name_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditMilestoneOption {
            title: Some("v2.0".to_string()),
            ..Default::default()
        };
        let result = client
            .issues()
            .edit_milestone_by_name("owner", "repo", "v1.0", opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_milestone_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_milestone("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_milestone_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/milestones/\d+"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().delete_milestone("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_milestone_by_name_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_milestone_by_name("owner", "repo", "v1.0")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_milestone_by_name_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/api/v1/repos/owner/repo/milestones/v1%2E0"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_milestone_by_name("owner", "repo", "v1.0")
            .await;
        assert!(result.is_err());
    }

    // ── issue_reaction.go ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_reactions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([reaction_json(":+1:")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reactions, resp) = client
            .issues()
            .list_issue_reactions("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(reactions.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_reactions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_reactions("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_issue_comment_reactions_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([reaction_json(":+1:")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reactions, resp) = client
            .issues()
            .get_issue_comment_reactions("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(reactions.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_comment_reactions_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue_comment_reactions("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_post_issue_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(reaction_json(":+1:")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reaction, resp) = client
            .issues()
            .post_issue_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(reaction.reaction, ":+1:");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_post_issue_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .post_issue_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_post_issue_comment_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(reaction_json(":+1:")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (reaction, resp) = client
            .issues()
            .post_issue_comment_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(reaction.reaction, ":+1:");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_post_issue_comment_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .post_issue_comment_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_comment_reaction_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_comment_reaction("owner", "repo", 1, "+1")
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_comment_reaction_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/comments/\d+/reactions",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_comment_reaction("owner", "repo", 1, "+1")
            .await;
        assert!(result.is_err());
    }

    // ── issue_subscription.go ─────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_subscribers_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([user_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (users, resp) = client
            .issues()
            .list_issue_subscribers("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(users.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_subscribers_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_subscribers("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_issue_subscription_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .add_issue_subscription("owner", "repo", 1, "testuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_add_issue_subscription_already_subscribed() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .add_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_issue_subscription_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .add_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_subscription_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_subscription("owner", "repo", 1, "testuser")
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_delete_issue_subscription_already_unsubscribed() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(200))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_subscription_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_subscription("owner", "repo", 1, "testuser")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_check_issue_subscription_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/check",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(watch_info_json()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (info, resp) = client
            .issues()
            .check_issue_subscription("owner", "repo", 1)
            .await
            .unwrap();
        assert!(info.subscribed);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_check_issue_subscription_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/check",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .check_issue_subscription("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_issue_subscribe_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
            .mount(&server)
            .await;
        Mock::given(method("PUT"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .issue_subscribe("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_issue_subscribe_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().issue_subscribe("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_issue_unsubscribe_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(user_json()))
            .mount(&server)
            .await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/subscriptions/[^/]+",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .issue_unsubscribe("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_issue_unsubscribe_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/user"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().issue_unsubscribe("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    // ── issue_stopwatch.go ────────────────────────────────────────

    #[tokio::test]
    async fn test_list_my_stopwatches_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/stopwatches"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([stopwatch_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (stopwatches, resp) = client
            .issues()
            .list_my_stopwatches(Default::default())
            .await
            .unwrap();
        assert_eq!(stopwatches.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_stopwatches_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/stopwatches"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_my_stopwatches(Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_issue_stopwatch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/delete",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_issue_stopwatch("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_issue_stopwatch_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/delete",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .delete_issue_stopwatch("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_start_issue_stopwatch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/start",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .start_issue_stopwatch("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_start_issue_stopwatch_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/start",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .start_issue_stopwatch("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_stop_issue_stopwatch_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/stop",
            ))
            .respond_with(ResponseTemplate::new(201))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .stop_issue_stopwatch("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_stop_issue_stopwatch_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/stopwatch/stop",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .stop_issue_stopwatch("owner", "repo", 1)
            .await;
        assert!(result.is_err());
    }

    // ── issue_tracked_time.go ─────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_tracked_times_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/times"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([tracked_time_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (times, resp) = client
            .issues()
            .list_repo_tracked_times("owner", "repo", Default::default())
            .await
            .unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_tracked_times_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_tracked_times("owner", "repo", Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_my_tracked_times_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/times"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([tracked_time_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (times, resp) = client
            .issues()
            .list_my_tracked_times(Default::default())
            .await
            .unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_my_tracked_times_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/user/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_my_tracked_times(Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_time_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(200).set_body_json(tracked_time_json(1)))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = AddTimeOption {
            time: 3600,
            created: None,
            user: String::new(),
        };
        let (tt, resp) = client
            .issues()
            .add_time("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(tt.id, 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_add_time_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = AddTimeOption {
            time: 3600,
            created: None,
            user: String::new(),
        };
        let result = client.issues().add_time("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_add_time_validation() {
        let server = MockServer::start().await;
        let client = create_test_client(&server);
        let opt = AddTimeOption {
            time: 0,
            created: None,
            user: String::new(),
        };
        let result = client.issues().add_time("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_issue_tracked_times_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([tracked_time_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (times, resp) = client
            .issues()
            .list_issue_tracked_times("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(times.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_tracked_times_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_tracked_times("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_reset_issue_time_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .reset_issue_time("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_reset_issue_time_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times"))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().reset_issue_time("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_time_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times/\d+",
            ))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .delete_time("owner", "repo", 1, 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_delete_time_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/times/\d+",
            ))
            .respond_with(ResponseTemplate::new(404).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().delete_time("owner", "repo", 1, 1).await;
        assert!(result.is_err());
    }

    // ── issue_timeline.go ─────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_timeline_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/timeline"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([timeline_comment_json(1)])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (comments, resp) = client
            .issues()
            .list_issue_timeline("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_timeline_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/timeline"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_timeline("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    // ── issue_pin.go ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_repo_pinned_issues_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/pinned"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(1, 1, "Pinned")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_repo_pinned_issues("owner", "repo")
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_pinned_issues_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/pinned"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_repo_pinned_issues("owner", "repo")
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_pin_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/pin"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client.issues().pin_issue("owner", "repo", 1).await.unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_pin_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/pin"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().pin_issue("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unpin_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/pin"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .unpin_issue("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unpin_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/pin"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().unpin_issue("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_move_issue_pin_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/pin/\d+"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .move_issue_pin("owner", "repo", 1, 2)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_move_issue_pin_error() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/pin/\d+"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().move_issue_pin("owner", "repo", 1, 2).await;
        assert!(result.is_err());
    }

    // ── issue_ext.go ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_list_issue_blocks_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_json(2, 2, "Blocker")])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_issue_blocks("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_blocks_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_blocks("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_blocking_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(2, 2, "Blocker")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let (issue, resp) = client
            .issues()
            .create_issue_blocking("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 2);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_blocking_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let result = client
            .issues()
            .create_issue_blocking("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_issue_blocking_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(2, 2, "Blocker")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let (issue, resp) = client
            .issues()
            .remove_issue_blocking("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 2);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_remove_issue_blocking_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/blocks"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 2 };
        let result = client
            .issues()
            .remove_issue_blocking("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_issue_dependencies_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!([issue_json(
                    3,
                    3,
                    "Dependency"
                )])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_issue_dependencies("owner", "repo", 1, Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_issue_dependencies_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client
            .issues()
            .list_issue_dependencies("owner", "repo", 1, Default::default())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_issue_dependency_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(3, 3, "Dep")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let (issue, resp) = client
            .issues()
            .create_issue_dependency("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 3);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_create_issue_dependency_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let result = client
            .issues()
            .create_issue_dependency("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_issue_dependency_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(3, 3, "Dep")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let (issue, resp) = client
            .issues()
            .remove_issue_dependency("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 3);
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_remove_issue_dependency_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(
                r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/dependencies",
            ))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = IssueMeta { index: 3 };
        let result = client
            .issues()
            .remove_issue_dependency("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_lock_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = LockIssueOption {
            lock_reason: String::new(),
        };
        let resp = client
            .issues()
            .lock_issue("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_lock_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = LockIssueOption {
            lock_reason: String::new(),
        };
        let result = client.issues().lock_issue("owner", "repo", 1, opt).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unlock_issue_happy() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(204))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let resp = client
            .issues()
            .unlock_issue("owner", "repo", 1)
            .await
            .unwrap();
        assert_eq!(resp.status, 204);
    }

    #[tokio::test]
    async fn test_unlock_issue_error() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/lock"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().unlock_issue("owner", "repo", 1).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update_issue_deadline_happy() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/deadline"))
            .respond_with(ResponseTemplate::new(201).set_body_json(issue_json(1, 1, "Bug")))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditDeadlineOption {
            deadline: Some(time::OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2025, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            )),
        };
        let (issue, resp) = client
            .issues()
            .update_issue_deadline("owner", "repo", 1, opt)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_update_issue_deadline_error() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issues/\d+/deadline"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let opt = EditDeadlineOption { deadline: None };
        let result = client
            .issues()
            .update_issue_deadline("owner", "repo", 1, opt)
            .await;
        assert!(result.is_err());
    }

    // ── issue_template.go ─────────────────────────────────────────

    #[tokio::test]
    async fn test_get_issue_templates_happy() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issue_templates"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!([issue_template_json()])),
            )
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let (templates, resp) = client
            .issues()
            .get_issue_templates("owner", "repo")
            .await
            .unwrap();
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].name, "Bug Report");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_get_issue_templates_error() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path_regex(r"/api/v1/repos/[^/]+/[^/]+/issue_templates"))
            .respond_with(ResponseTemplate::new(500).set_body_json(error_body()))
            .mount(&server)
            .await;
        let client = create_test_client(&server);
        let result = client.issues().get_issue_templates("owner", "repo").await;
        assert!(result.is_err());
    }
}
