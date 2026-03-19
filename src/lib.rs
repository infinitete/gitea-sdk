// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Gitea API client for Rust.
//!
//! This crate provides an asynchronous client for the Gitea API.
//!
//! # Quick Start
//!
//! ```no_run
//! use gitea_sdk_rs::Client;
//!
//! # fn main() -> Result<(), gitea_sdk_rs::Error> {
//! let client = Client::builder("https://gitea.example.com")
//!     .token("your-token")
//!     .build()?;
//! # let _ = client;
//! # Ok(())
//! # }
//! ```

// Re-export serde macros for convenience.
pub use serde::{Deserialize, Serialize};

mod client;
mod error;
mod internal;
mod pagination;
mod response;
mod version;

pub mod auth;

pub mod api;
pub mod options;
pub mod types;

// Public API re-exports.
pub use client::{Client, ClientBuilder};
pub use error::{Error, Result};
pub use pagination::{ListOptions, QueryEncode};
pub use response::{PageLinks, Response};

// Re-export entity types for convenience.
pub use types::{
    AccessToken, ActionTask, ActionTaskResponse, ActionWorkflowJob, ActionWorkflowJobsResponse,
    ActionWorkflowRun, ActionWorkflowRunsResponse, ActionWorkflowStep, Activity, AnnotatedTag,
    AnnotatedTagObject, Attachment, Badge, Branch, BranchProtection, ChangedFile,
    CollaboratorPermissionResult, CombinedStatus, Comment, Commit, CommitAffectedFiles,
    CommitDateOptions, CommitMeta, CommitStats, CommitUser, Compare, ContentsExtResponse,
    ContentsResponse, CronTask, DeployKey, Email, ExternalTracker, ExternalWiki,
    FileCommitResponse, FileLinksResponse, FileResponse, GPGKey, GPGKeyEmail, GitBlobResponse,
    GitEntry, GitHook, GitObject, GitTreeResponse, GitignoreTemplateInfo, GlobalAPISettings,
    GlobalAttachmentSettings, GlobalRepoSettings, GlobalUISettings, Hook, Identity,
    InternalTracker, Issue, IssueBlockedBy, IssueFormElement, IssueFormElementAttributes,
    IssueFormElementValidations, IssueMeta, IssueTemplate, Label, LabelTemplate,
    LicenseTemplateInfo, LicensesTemplateListEntry, Milestone, NodeInfo, NodeInfoServices,
    NodeInfoSoftware, NodeInfoUsage, NodeInfoUsageUsers, Note, NotificationThread, NotifySubject,
    Oauth2, OrgPermissions, Organization, PRBranchInfo, PRBranchInfoRepo, Package, PackageFile,
    PayloadCommit, PayloadCommitVerification, PayloadUser, Permission, PublicKey, PullRequest,
    PullRequestMeta, PullReview, PullReviewComment, PushMirrorResponse, Reaction, Reference,
    Release, RepoActionVariable, RepoTransfer, Repository, RepositoryMeta, Secret, Status,
    StopWatch, Tag, TagProtection, Team, TimelineComment, TrackedTime, User, UserHeatmapData,
    UserSettings, WatchInfo, WikiCommit, WikiCommitList, WikiPage, WikiPageMetaData,
};

// Re-export all enums for convenience.
pub use types::enums::*;

// Re-export API sub-structs for convenience.
pub use api::{
    ActionsApi, ActivityPubApi, AdminApi, HooksApi, IssuesApi, MiscApi, NotificationsApi,
    Oauth2Api, OrgsApi, PackagesApi, PullsApi, ReleasesApi, ReposApi, SettingsApi, StatusApi,
    UsersApi,
};

#[cfg(test)]
mod tests {
    #[test]
    fn test_public_api_accessible() {
        let _: fn() -> crate::Client = || unimplemented!();
        let _: fn() -> crate::ClientBuilder<'static> = || unimplemented!();
        let _: fn() -> crate::Error = || unimplemented!();
        let _: fn() -> crate::Result<String> = || unimplemented!();
        let _: fn() -> crate::Response = || unimplemented!();
        let _: fn() -> crate::PageLinks = || unimplemented!();
        let _: fn() -> crate::ListOptions = || unimplemented!();
    }

    #[test]
    fn test_public_types_accessible() {
        let _: fn() -> crate::User = || unimplemented!();
        let _: fn() -> crate::Label = || unimplemented!();
        let _: fn() -> crate::Milestone = || unimplemented!();
        let _: fn() -> crate::Organization = || unimplemented!();
        let _: fn() -> crate::Team = || unimplemented!();
        let _: fn() -> crate::Release = || unimplemented!();
        let _: fn() -> crate::Comment = || unimplemented!();
        let _: fn() -> crate::Status = || unimplemented!();
        let _: fn() -> crate::StateType = || unimplemented!();
        let _: fn() -> crate::ReposApi<'static> = || unimplemented!();
        let _: fn() -> crate::IssuesApi<'static> = || unimplemented!();
    }
}
