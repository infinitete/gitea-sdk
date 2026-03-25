// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Entity types returned by the Gitea API, organized by resource.

#[doc(hidden)]
mod reexports {
    pub use super::action::{
        ActionTask, ActionTaskResponse, ActionWorkflowJob, ActionWorkflowJobsResponse,
        ActionWorkflowRun, ActionWorkflowRunsResponse, ActionWorkflowStep,
    };
    pub use super::activity::Activity;
    pub use super::badge::Badge;
    pub use super::comment::Comment;
    pub use super::cron_task::CronTask;
    pub use super::hook::Hook;
    pub use super::issue::{
        Issue, IssueBlockedBy, IssueFormElement, IssueFormElementAttributes,
        IssueFormElementValidations, IssueMeta, IssueTemplate, PullRequestMeta, RepositoryMeta,
        StopWatch, TimelineComment, TrackedTime, WatchInfo,
    };
    pub use super::label::Label;
    pub use super::license::{LicenseTemplateInfo, LicensesTemplateListEntry};
    pub use super::milestone::Milestone;
    pub use super::node_info::{
        GitignoreTemplateInfo, LabelTemplate, NodeInfo, NodeInfoServices, NodeInfoSoftware,
        NodeInfoUsage, NodeInfoUsageUsers,
    };
    pub use super::notification::{NotificationThread, NotifySubject};
    pub use super::oauth2::Oauth2;
    pub use super::organization::{OrgPermissions, Organization};
    pub use super::package::{Package, PackageFile};
    pub use super::pull_request::{
        ChangedFile, PRBranchInfo, PRBranchInfoRepo, PullRequest, PullReview, PullReviewComment,
    };
    pub use super::reaction::Reaction;
    pub use super::release::{Attachment, Release};
    pub use super::repository::{
        AnnotatedTag, AnnotatedTagObject, Branch, BranchProtection, CollaboratorPermissionResult,
        Commit, CommitAffectedFiles, CommitDateOptions, CommitMeta, CommitStats, CommitUser,
        Compare, ContentsExtResponse, ContentsResponse, DeployKey, ExternalTracker, ExternalWiki,
        FileCommitResponse, FileLinksResponse, FileResponse, GitBlobResponse, GitEntry, GitHook,
        GitObject, GitTreeResponse, Identity, InternalTracker, Note, PayloadCommit,
        PayloadCommitVerification, PayloadUser, Permission, PushMirrorResponse, Reference,
        RepoActionVariable, RepoTransfer, Repository, Tag, TagProtection, WikiCommit,
        WikiCommitList, WikiPage, WikiPageMetaData,
    };
    pub use super::secret::Secret;
    pub use super::settings::{
        GlobalAPISettings, GlobalAttachmentSettings, GlobalRepoSettings, GlobalUISettings,
    };
    pub use super::status::{CombinedStatus, Status};
    pub use super::team::Team;
    pub use super::user::{
        AccessToken, Email, GPGKey, GPGKeyEmail, PublicKey, User, UserHeatmapData,
    };
    pub use super::user_settings::UserSettings;
}

#[doc(hidden)]
pub use reexports::*;

pub mod enums;
pub mod serde_helpers;

pub mod action;
pub mod activity;
pub mod badge;
pub mod comment;
pub mod cron_task;
pub mod hook;
pub mod issue;
pub mod label;
pub mod license;
pub mod milestone;
pub mod node_info;
pub mod notification;
pub mod oauth2;
pub mod organization;
pub mod package;
pub mod pull_request;
pub mod reaction;
pub mod release;
pub mod repository;
pub mod secret;
pub mod settings;
pub mod status;
pub mod team;
pub mod user;
pub mod user_settings;
