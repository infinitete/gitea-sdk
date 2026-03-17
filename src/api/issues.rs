// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::Client;
use crate::Response;
use crate::options::issue::*;
use crate::pagination::QueryEncode;
use crate::types::{
    Attachment, Comment, Issue, IssueMeta, IssueTemplate, Milestone, StopWatch, TimelineComment,
    TrackedTime, WatchInfo,
};

pub struct IssuesApi<'a> {
    client: &'a Client,
}

fn json_body<T: serde::Serialize>(val: &T) -> crate::Result<String> {
    Ok(serde_json::to_string(val)?)
}

fn json_header() -> reqwest::header::HeaderMap {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::CONTENT_TYPE,
        reqwest::header::HeaderValue::from_static("application/json"),
    );
    headers
}

impl<'a> IssuesApi<'a> {
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
        self.client()
            .get_parsed_response(
                reqwest::Method::POST,
                &path,
                Some(&json_header()),
                Some(body),
            )
            .await
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
    use wiremock::matchers::{method, path};
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

    #[tokio::test]
    async fn test_create_issue() {
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
        assert_eq!(issue.title, "Bug fix");
        assert_eq!(issue.index, 1);
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_get_issue() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(
                1,
                1,
                "Feature request",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (issue, resp) = client
            .issues()
            .get_issue("testowner", "testrepo", 1)
            .await
            .unwrap();
        assert_eq!(issue.id, 1);
        assert_eq!(issue.title, "Feature request");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_list_repo_issues() {
        let server = MockServer::start().await;
        let body = serde_json::json!([issue_json(1, 1, "Issue 1"), issue_json(2, 2, "Issue 2"),]);

        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&body))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let (issues, resp) = client
            .issues()
            .list_repo_issues("testowner", "testrepo", Default::default())
            .await
            .unwrap();
        assert_eq!(issues.len(), 2);
        assert_eq!(issues[0].title, "Issue 1");
        assert_eq!(issues[1].title, "Issue 2");
        assert_eq!(resp.status, 200);
    }

    #[tokio::test]
    async fn test_create_issue_comment() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1/comments"))
            .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
                "id": 1,
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
                "body": "Nice issue",
                "created": "2024-01-15T10:00:00Z",
                "updated": "2024-01-15T10:00:00Z"
            })))
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
        assert_eq!(comment.body, "Nice issue");
        assert_eq!(resp.status, 201);
    }

    #[tokio::test]
    async fn test_error_case() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/999"))
            .respond_with(
                ResponseTemplate::new(404)
                    .set_body_json(serde_json::json!({"message": "Issue not found"})),
            )
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let result = client
            .issues()
            .get_issue("testowner", "testrepo", 999)
            .await;
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::Error::Api {
                status, message, ..
            } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Issue not found");
            }
            other => panic!("expected Error::Api, got: {other}"),
        }
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
    async fn test_delete_issue() {
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
    async fn test_edit_issue() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/api/v1/repos/testowner/testrepo/issues/1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(issue_json(
                1,
                1,
                "Updated title",
            )))
            .mount(&server)
            .await;

        let client = create_test_client(&server);
        let opt = EditIssueOption {
            title: Some("Updated title".to_string()),
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
        assert_eq!(issue.title, "Updated title");
        assert_eq!(resp.status, 200);
    }
}
