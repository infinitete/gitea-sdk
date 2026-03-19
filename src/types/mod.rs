// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Entity types returned by the Gitea API, organized by resource.

#[allow(unused_imports)]
#[doc(hidden)]
pub use action::{
    ActionTask, ActionTaskResponse, ActionWorkflowJob, ActionWorkflowJobsResponse,
    ActionWorkflowRun, ActionWorkflowRunsResponse, ActionWorkflowStep,
};
#[allow(unused_imports)]
#[doc(hidden)]
pub use activity::Activity;
#[allow(unused_imports)]
#[doc(hidden)]
pub use badge::Badge;
#[allow(unused_imports)]
#[doc(hidden)]
pub use comment::Comment;
#[allow(unused_imports)]
#[doc(hidden)]
pub use cron_task::CronTask;
#[allow(unused_imports)]
#[doc(hidden)]
pub use hook::Hook;
#[allow(unused_imports)]
#[doc(hidden)]
pub use issue::{
    Issue, IssueBlockedBy, IssueFormElement, IssueFormElementAttributes,
    IssueFormElementValidations, IssueMeta, IssueTemplate, PullRequestMeta, RepositoryMeta,
    StopWatch, TimelineComment, TrackedTime, WatchInfo,
};
#[allow(unused_imports)]
#[doc(hidden)]
pub use label::Label;
#[allow(unused_imports)]
#[doc(hidden)]
pub use license::{LicenseTemplateInfo, LicensesTemplateListEntry};
#[allow(unused_imports)]
#[doc(hidden)]
pub use milestone::Milestone;
#[allow(unused_imports)]
#[doc(hidden)]
pub use node_info::{
    GitignoreTemplateInfo, LabelTemplate, NodeInfo, NodeInfoServices, NodeInfoSoftware,
    NodeInfoUsage, NodeInfoUsageUsers,
};
#[allow(unused_imports)]
#[doc(hidden)]
pub use notification::{NotificationThread, NotifySubject};
#[allow(unused_imports)]
#[doc(hidden)]
pub use oauth2::Oauth2;
#[allow(unused_imports)]
#[doc(hidden)]
pub use organization::{OrgPermissions, Organization};
#[allow(unused_imports)]
#[doc(hidden)]
pub use package::{Package, PackageFile};
#[allow(unused_imports)]
#[doc(hidden)]
pub use pull_request::{
    ChangedFile, PRBranchInfo, PRBranchInfoRepo, PullRequest, PullReview, PullReviewComment,
};
#[allow(unused_imports)]
#[doc(hidden)]
pub use reaction::Reaction;
#[allow(unused_imports)]
#[doc(hidden)]
pub use release::{Attachment, Release};
#[allow(unused_imports)]
#[doc(hidden)]
pub use repository::{
    AnnotatedTag, AnnotatedTagObject, Branch, BranchProtection, CollaboratorPermissionResult,
    Commit, CommitAffectedFiles, CommitDateOptions, CommitMeta, CommitStats, CommitUser, Compare,
    ContentsExtResponse, ContentsResponse, DeployKey, ExternalTracker, ExternalWiki,
    FileCommitResponse, FileLinksResponse, FileResponse, GitBlobResponse, GitEntry, GitHook,
    GitObject, GitTreeResponse, Identity, InternalTracker, Note, PayloadCommit,
    PayloadCommitVerification, PayloadUser, Permission, PushMirrorResponse, Reference,
    RepoActionVariable, RepoTransfer, Repository, Tag, TagProtection, WikiCommit, WikiCommitList,
    WikiPage, WikiPageMetaData,
};
#[allow(unused_imports)]
#[doc(hidden)]
pub use secret::Secret;
#[allow(unused_imports)]
#[doc(hidden)]
pub use settings::{
    GlobalAPISettings, GlobalAttachmentSettings, GlobalRepoSettings, GlobalUISettings,
};
#[allow(unused_imports)]
#[doc(hidden)]
pub use status::{CombinedStatus, Status};
#[allow(unused_imports)]
#[doc(hidden)]
pub use team::Team;
#[allow(unused_imports)]
#[doc(hidden)]
pub use user::{AccessToken, Email, GPGKey, GPGKeyEmail, PublicKey, User, UserHeatmapData};
#[allow(unused_imports)]
#[doc(hidden)]
pub use user_settings::UserSettings;

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
