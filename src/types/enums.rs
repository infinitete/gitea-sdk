// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Shared enum types used across the Gitea API.

use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display};

/// Issue/pull request state type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// State Type payload type.
pub enum StateType {
    Open,
    Closed,
    #[serde(other)]
    All,
}

/// Issue type — whether an item is an issue, pull request, or both
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Issue Type payload type.
pub enum IssueType {
    #[serde(rename = "")]
    #[strum(serialize = "")]
    All,
    #[serde(rename = "issues")]
    #[strum(serialize = "issues")]
    Issue,
    #[serde(rename = "pulls")]
    #[strum(serialize = "pulls")]
    Pull,
    #[serde(other)]
    Unknown,
}

/// Issue form element type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Issue Form Element Type payload type.
pub enum IssueFormElementType {
    Markdown,
    Textarea,
    Input,
    Dropdown,
    Checkboxes,
    #[serde(other)]
    Unknown,
}

/// Notification status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Notify Status payload type.
pub enum NotifyStatus {
    Unread,
    Read,
    Pinned,
    #[serde(other)]
    Unknown,
}

/// Notification subject type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Notify Subject Type payload type.
pub enum NotifySubjectType {
    #[serde(rename = "Issue")]
    #[strum(serialize = "Issue")]
    Issue,
    #[serde(rename = "Pull")]
    #[strum(serialize = "Pull")]
    Pull,
    #[serde(rename = "Commit")]
    #[strum(serialize = "Commit")]
    Commit,
    #[serde(rename = "Repository")]
    #[strum(serialize = "Repository")]
    Repository,
    #[serde(other)]
    Unknown,
}

/// Notification subject state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Notify Subject State payload type.
pub enum NotifySubjectState {
    Open,
    Closed,
    Merged,
    #[serde(other)]
    Unknown,
}

/// Pull request review state type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Review State Type payload type.
pub enum ReviewStateType {
    #[serde(rename = "")]
    #[strum(serialize = "")]
    Unknown,
    #[serde(rename = "APPROVED")]
    #[strum(serialize = "APPROVED")]
    Approved,
    #[serde(rename = "PENDING")]
    #[strum(serialize = "PENDING")]
    Pending,
    #[serde(rename = "COMMENT")]
    #[strum(serialize = "COMMENT")]
    Comment,
    #[serde(rename = "REQUEST_CHANGES")]
    #[strum(serialize = "REQUEST_CHANGES")]
    RequestChanges,
    #[serde(rename = "REQUEST_REVIEW")]
    #[strum(serialize = "REQUEST_REVIEW")]
    RequestReview,
    #[serde(other)]
    Unrecognized,
}

/// Access mode / permission level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Access Mode payload type.
pub enum AccessMode {
    None,
    Read,
    Write,
    Admin,
    Owner,
    #[serde(other)]
    Unknown,
}

/// Organization/repository visibility type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Visible Type payload type.
pub enum VisibleType {
    Public,
    Limited,
    Private,
    #[serde(other)]
    Unknown,
}

/// Repository unit type (e.g. code, issues, wiki)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Repo Unit Type payload type.
pub enum RepoUnitType {
    #[serde(rename = "repo.code")]
    #[strum(serialize = "repo.code")]
    Code,
    #[serde(rename = "repo.issues")]
    #[strum(serialize = "repo.issues")]
    Issues,
    #[serde(rename = "repo.ext_issues")]
    #[strum(serialize = "repo.ext_issues")]
    ExtIssues,
    #[serde(rename = "repo.wiki")]
    #[strum(serialize = "repo.wiki")]
    Wiki,
    #[serde(rename = "repo.pulls")]
    #[strum(serialize = "repo.pulls")]
    Pulls,
    #[serde(rename = "repo.ext_wiki")]
    #[strum(serialize = "repo.ext_wiki")]
    ExtWiki,
    #[serde(rename = "repo.releases")]
    #[strum(serialize = "repo.releases")]
    Releases,
    #[serde(rename = "repo.projects")]
    #[strum(serialize = "repo.projects")]
    Projects,
    #[serde(rename = "repo.packages")]
    #[strum(serialize = "repo.packages")]
    Packages,
    #[serde(rename = "repo.actions")]
    #[strum(serialize = "repo.actions")]
    Actions,
    #[serde(other)]
    Unknown,
}

/// Repository type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Repo Type payload type.
pub enum RepoType {
    #[serde(rename = "")]
    #[strum(serialize = "")]
    None,
    #[serde(rename = "source")]
    #[strum(serialize = "source")]
    Source,
    #[serde(rename = "fork")]
    #[strum(serialize = "fork")]
    Fork,
    #[serde(rename = "mirror")]
    #[strum(serialize = "mirror")]
    Mirror,
    #[serde(other)]
    Unknown,
}

/// Trust model for git signatures in a repository
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Trust Model payload type.
pub enum TrustModel {
    Default,
    Collaborator,
    Committer,
    CollaboratorCommitter,
    #[serde(other)]
    Unknown,
}

/// Projects mode — which kinds of projects to show
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Projects Mode payload type.
pub enum ProjectsMode {
    Repo,
    Owner,
    All,
    #[serde(other)]
    Unknown,
}

/// Webhook type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Hook Type payload type.
pub enum HookType {
    Gitea,
    Slack,
    Discord,
    Dingtalk,
    Telegram,
    Msteams,
    Feishu,
    Gogs,
    #[serde(other)]
    Unknown,
}

/// Commit status state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Status State payload type.
pub enum StatusState {
    Pending,
    Success,
    Error,
    Failure,
    Warning,
    #[serde(other)]
    Unknown,
}

/// Merge style for pull requests
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Merge Style payload type.
pub enum MergeStyle {
    #[serde(rename = "merge")]
    #[strum(serialize = "merge")]
    Merge,
    #[serde(rename = "rebase")]
    #[strum(serialize = "rebase")]
    Rebase,
    #[serde(rename = "rebase-merge")]
    #[strum(serialize = "rebase-merge")]
    RebaseMerge,
    #[serde(rename = "squash")]
    #[strum(serialize = "squash")]
    Squash,
    #[serde(other)]
    Unknown,
}

/// Access token scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Access Token Scope payload type.
pub enum AccessTokenScope {
    #[serde(rename = "all")]
    #[strum(serialize = "all")]
    All,
    #[serde(rename = "repo")]
    #[strum(serialize = "repo")]
    Repo,
    #[serde(rename = "repo:status")]
    #[strum(serialize = "repo:status")]
    RepoStatus,
    #[serde(rename = "public_repo")]
    #[strum(serialize = "public_repo")]
    PublicRepo,
    #[serde(rename = "admin:org")]
    #[strum(serialize = "admin:org")]
    AdminOrg,
    #[serde(rename = "write:org")]
    #[strum(serialize = "write:org")]
    WriteOrg,
    #[serde(rename = "read:org")]
    #[strum(serialize = "read:org")]
    ReadOrg,
    #[serde(rename = "admin:public_key")]
    #[strum(serialize = "admin:public_key")]
    AdminPublicKey,
    #[serde(rename = "write:public_key")]
    #[strum(serialize = "write:public_key")]
    WritePublicKey,
    #[serde(rename = "read:public_key")]
    #[strum(serialize = "read:public_key")]
    ReadPublicKey,
    #[serde(rename = "admin:repo_hook")]
    #[strum(serialize = "admin:repo_hook")]
    AdminRepoHook,
    #[serde(rename = "write:repo_hook")]
    #[strum(serialize = "write:repo_hook")]
    WriteRepoHook,
    #[serde(rename = "read:repo_hook")]
    #[strum(serialize = "read:repo_hook")]
    ReadRepoHook,
    #[serde(rename = "admin:org_hook")]
    #[strum(serialize = "admin:org_hook")]
    AdminOrgHook,
    #[serde(rename = "admin:user_hook")]
    #[strum(serialize = "admin:user_hook")]
    AdminUserHook,
    #[serde(rename = "notification")]
    #[strum(serialize = "notification")]
    Notification,
    #[serde(rename = "user")]
    #[strum(serialize = "user")]
    User,
    #[serde(rename = "read:user")]
    #[strum(serialize = "read:user")]
    ReadUser,
    #[serde(rename = "user:email")]
    #[strum(serialize = "user:email")]
    UserEmail,
    #[serde(rename = "user:follow")]
    #[strum(serialize = "user:follow")]
    UserFollow,
    #[serde(rename = "delete_repo")]
    #[strum(serialize = "delete_repo")]
    DeleteRepo,
    #[serde(rename = "package")]
    #[strum(serialize = "package")]
    Package,
    #[serde(rename = "write:package")]
    #[strum(serialize = "write:package")]
    WritePackage,
    #[serde(rename = "read:package")]
    #[strum(serialize = "read:package")]
    ReadPackage,
    #[serde(rename = "delete:package")]
    #[strum(serialize = "delete:package")]
    DeletePackage,
    #[serde(rename = "admin:gpg_key")]
    #[strum(serialize = "admin:gpg_key")]
    AdminGpgKey,
    #[serde(rename = "write:gpg_key")]
    #[strum(serialize = "write:gpg_key")]
    WriteGpgKey,
    #[serde(rename = "read:gpg_key")]
    #[strum(serialize = "read:gpg_key")]
    ReadGpgKey,
    #[serde(rename = "admin:application")]
    #[strum(serialize = "admin:application")]
    AdminApplication,
    #[serde(rename = "write:application")]
    #[strum(serialize = "write:application")]
    WriteApplication,
    #[serde(rename = "read:application")]
    #[strum(serialize = "read:application")]
    ReadApplication,
    #[serde(rename = "sudo")]
    #[strum(serialize = "sudo")]
    Sudo,
    #[serde(other)]
    Unknown,
}

/// Archive download format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
/// Archive Type payload type.
pub enum ArchiveType {
    #[serde(rename = ".zip")]
    #[strum(serialize = ".zip")]
    Zip,
    #[serde(rename = ".tar.gz")]
    #[strum(serialize = ".tar.gz")]
    TarGz,
    #[serde(other)]
    Unknown,
}

/// Git service type for repository migration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
/// Git Service Type payload type.
pub enum GitServiceType {
    Git,
    Github,
    Gitea,
    Gitlab,
    Gogs,
    #[serde(other)]
    Unknown,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_state_type() {
        assert_eq!(serde_json::to_string(&StateType::Open).unwrap(), "\"open\"");
        assert_eq!(
            serde_json::to_string(&StateType::Closed).unwrap(),
            "\"closed\""
        );
        assert_eq!(serde_json::to_string(&StateType::All).unwrap(), "\"all\"");

        assert_eq!(
            serde_json::from_str::<StateType>("\"open\"").unwrap(),
            StateType::Open
        );
        assert_eq!(
            serde_json::from_str::<StateType>("\"closed\"").unwrap(),
            StateType::Closed
        );
        assert_eq!(
            serde_json::from_str::<StateType>("\"all\"").unwrap(),
            StateType::All
        );
        assert_eq!(
            serde_json::from_str::<StateType>("\"unknown\"").unwrap(),
            StateType::All
        );
    }

    #[test]
    fn test_issue_type() {
        assert_eq!(serde_json::to_string(&IssueType::All).unwrap(), "\"\"");
        assert_eq!(
            serde_json::to_string(&IssueType::Issue).unwrap(),
            "\"issues\""
        );
        assert_eq!(
            serde_json::to_string(&IssueType::Pull).unwrap(),
            "\"pulls\""
        );

        assert_eq!(
            serde_json::from_str::<IssueType>("\"\"").unwrap(),
            IssueType::All
        );
        assert_eq!(
            serde_json::from_str::<IssueType>("\"issues\"").unwrap(),
            IssueType::Issue
        );
        assert_eq!(
            serde_json::from_str::<IssueType>("\"pulls\"").unwrap(),
            IssueType::Pull
        );
        assert_eq!(
            serde_json::from_str::<IssueType>("\"unknown\"").unwrap(),
            IssueType::Unknown
        );
    }

    #[test]
    fn test_issue_form_element_type() {
        assert_eq!(
            serde_json::to_string(&IssueFormElementType::Markdown).unwrap(),
            "\"markdown\""
        );
        assert_eq!(
            serde_json::to_string(&IssueFormElementType::Textarea).unwrap(),
            "\"textarea\""
        );
        assert_eq!(
            serde_json::to_string(&IssueFormElementType::Input).unwrap(),
            "\"input\""
        );
        assert_eq!(
            serde_json::to_string(&IssueFormElementType::Dropdown).unwrap(),
            "\"dropdown\""
        );
        assert_eq!(
            serde_json::to_string(&IssueFormElementType::Checkboxes).unwrap(),
            "\"checkboxes\""
        );

        assert_eq!(
            serde_json::from_str::<IssueFormElementType>("\"markdown\"").unwrap(),
            IssueFormElementType::Markdown
        );
        assert_eq!(
            serde_json::from_str::<IssueFormElementType>("\"unknown\"").unwrap(),
            IssueFormElementType::Unknown
        );
    }

    #[test]
    fn test_notify_status() {
        assert_eq!(
            serde_json::to_string(&NotifyStatus::Unread).unwrap(),
            "\"unread\""
        );
        assert_eq!(
            serde_json::to_string(&NotifyStatus::Read).unwrap(),
            "\"read\""
        );
        assert_eq!(
            serde_json::to_string(&NotifyStatus::Pinned).unwrap(),
            "\"pinned\""
        );

        assert_eq!(
            serde_json::from_str::<NotifyStatus>("\"unread\"").unwrap(),
            NotifyStatus::Unread
        );
        assert_eq!(
            serde_json::from_str::<NotifyStatus>("\"read\"").unwrap(),
            NotifyStatus::Read
        );
        assert_eq!(
            serde_json::from_str::<NotifyStatus>("\"pinned\"").unwrap(),
            NotifyStatus::Pinned
        );
        assert_eq!(
            serde_json::from_str::<NotifyStatus>("\"unknown\"").unwrap(),
            NotifyStatus::Unknown
        );
    }

    #[test]
    fn test_notify_subject_type() {
        assert_eq!(
            serde_json::to_string(&NotifySubjectType::Issue).unwrap(),
            "\"Issue\""
        );
        assert_eq!(
            serde_json::to_string(&NotifySubjectType::Pull).unwrap(),
            "\"Pull\""
        );
        assert_eq!(
            serde_json::to_string(&NotifySubjectType::Commit).unwrap(),
            "\"Commit\""
        );
        assert_eq!(
            serde_json::to_string(&NotifySubjectType::Repository).unwrap(),
            "\"Repository\""
        );

        assert_eq!(
            serde_json::from_str::<NotifySubjectType>("\"Issue\"").unwrap(),
            NotifySubjectType::Issue
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectType>("\"Pull\"").unwrap(),
            NotifySubjectType::Pull
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectType>("\"Commit\"").unwrap(),
            NotifySubjectType::Commit
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectType>("\"Repository\"").unwrap(),
            NotifySubjectType::Repository
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectType>("\"unknown\"").unwrap(),
            NotifySubjectType::Unknown
        );
    }

    #[test]
    fn test_notify_subject_state() {
        assert_eq!(
            serde_json::to_string(&NotifySubjectState::Open).unwrap(),
            "\"open\""
        );
        assert_eq!(
            serde_json::to_string(&NotifySubjectState::Closed).unwrap(),
            "\"closed\""
        );
        assert_eq!(
            serde_json::to_string(&NotifySubjectState::Merged).unwrap(),
            "\"merged\""
        );

        assert_eq!(
            serde_json::from_str::<NotifySubjectState>("\"open\"").unwrap(),
            NotifySubjectState::Open
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectState>("\"closed\"").unwrap(),
            NotifySubjectState::Closed
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectState>("\"merged\"").unwrap(),
            NotifySubjectState::Merged
        );
        assert_eq!(
            serde_json::from_str::<NotifySubjectState>("\"unknown\"").unwrap(),
            NotifySubjectState::Unknown
        );
    }

    #[test]
    fn test_review_state_type() {
        assert_eq!(
            serde_json::to_string(&ReviewStateType::Unknown).unwrap(),
            "\"\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewStateType::Approved).unwrap(),
            "\"APPROVED\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewStateType::Pending).unwrap(),
            "\"PENDING\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewStateType::Comment).unwrap(),
            "\"COMMENT\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewStateType::RequestChanges).unwrap(),
            "\"REQUEST_CHANGES\""
        );
        assert_eq!(
            serde_json::to_string(&ReviewStateType::RequestReview).unwrap(),
            "\"REQUEST_REVIEW\""
        );

        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"\"").unwrap(),
            ReviewStateType::Unknown
        );
        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"APPROVED\"").unwrap(),
            ReviewStateType::Approved
        );
        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"PENDING\"").unwrap(),
            ReviewStateType::Pending
        );
        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"COMMENT\"").unwrap(),
            ReviewStateType::Comment
        );
        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"REQUEST_CHANGES\"").unwrap(),
            ReviewStateType::RequestChanges
        );
        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"REQUEST_REVIEW\"").unwrap(),
            ReviewStateType::RequestReview
        );
        assert_eq!(
            serde_json::from_str::<ReviewStateType>("\"unknown\"").unwrap(),
            ReviewStateType::Unrecognized
        );
    }

    #[test]
    fn test_access_mode() {
        assert_eq!(
            serde_json::to_string(&AccessMode::None).unwrap(),
            "\"none\""
        );
        assert_eq!(
            serde_json::to_string(&AccessMode::Read).unwrap(),
            "\"read\""
        );
        assert_eq!(
            serde_json::to_string(&AccessMode::Write).unwrap(),
            "\"write\""
        );
        assert_eq!(
            serde_json::to_string(&AccessMode::Admin).unwrap(),
            "\"admin\""
        );
        assert_eq!(
            serde_json::to_string(&AccessMode::Owner).unwrap(),
            "\"owner\""
        );

        assert_eq!(
            serde_json::from_str::<AccessMode>("\"none\"").unwrap(),
            AccessMode::None
        );
        assert_eq!(
            serde_json::from_str::<AccessMode>("\"read\"").unwrap(),
            AccessMode::Read
        );
        assert_eq!(
            serde_json::from_str::<AccessMode>("\"write\"").unwrap(),
            AccessMode::Write
        );
        assert_eq!(
            serde_json::from_str::<AccessMode>("\"admin\"").unwrap(),
            AccessMode::Admin
        );
        assert_eq!(
            serde_json::from_str::<AccessMode>("\"owner\"").unwrap(),
            AccessMode::Owner
        );
        assert_eq!(
            serde_json::from_str::<AccessMode>("\"unknown\"").unwrap(),
            AccessMode::Unknown
        );
    }

    #[test]
    fn test_visible_type() {
        assert_eq!(
            serde_json::to_string(&VisibleType::Public).unwrap(),
            "\"public\""
        );
        assert_eq!(
            serde_json::to_string(&VisibleType::Limited).unwrap(),
            "\"limited\""
        );
        assert_eq!(
            serde_json::to_string(&VisibleType::Private).unwrap(),
            "\"private\""
        );

        assert_eq!(
            serde_json::from_str::<VisibleType>("\"public\"").unwrap(),
            VisibleType::Public
        );
        assert_eq!(
            serde_json::from_str::<VisibleType>("\"limited\"").unwrap(),
            VisibleType::Limited
        );
        assert_eq!(
            serde_json::from_str::<VisibleType>("\"private\"").unwrap(),
            VisibleType::Private
        );
        assert_eq!(
            serde_json::from_str::<VisibleType>("\"unknown\"").unwrap(),
            VisibleType::Unknown
        );
    }

    #[test]
    fn test_repo_unit_type() {
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Code).unwrap(),
            "\"repo.code\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Issues).unwrap(),
            "\"repo.issues\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::ExtIssues).unwrap(),
            "\"repo.ext_issues\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Wiki).unwrap(),
            "\"repo.wiki\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Pulls).unwrap(),
            "\"repo.pulls\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::ExtWiki).unwrap(),
            "\"repo.ext_wiki\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Releases).unwrap(),
            "\"repo.releases\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Projects).unwrap(),
            "\"repo.projects\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Packages).unwrap(),
            "\"repo.packages\""
        );
        assert_eq!(
            serde_json::to_string(&RepoUnitType::Actions).unwrap(),
            "\"repo.actions\""
        );

        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.code\"").unwrap(),
            RepoUnitType::Code
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.issues\"").unwrap(),
            RepoUnitType::Issues
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.pulls\"").unwrap(),
            RepoUnitType::Pulls
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.ext_issues\"").unwrap(),
            RepoUnitType::ExtIssues
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.wiki\"").unwrap(),
            RepoUnitType::Wiki
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.ext_wiki\"").unwrap(),
            RepoUnitType::ExtWiki
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.releases\"").unwrap(),
            RepoUnitType::Releases
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.projects\"").unwrap(),
            RepoUnitType::Projects
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.packages\"").unwrap(),
            RepoUnitType::Packages
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"repo.actions\"").unwrap(),
            RepoUnitType::Actions
        );
        assert_eq!(
            serde_json::from_str::<RepoUnitType>("\"unknown\"").unwrap(),
            RepoUnitType::Unknown
        );
    }

    #[test]
    fn test_repo_type() {
        assert_eq!(serde_json::to_string(&RepoType::None).unwrap(), "\"\"");
        assert_eq!(
            serde_json::to_string(&RepoType::Source).unwrap(),
            "\"source\""
        );
        assert_eq!(serde_json::to_string(&RepoType::Fork).unwrap(), "\"fork\"");
        assert_eq!(
            serde_json::to_string(&RepoType::Mirror).unwrap(),
            "\"mirror\""
        );

        assert_eq!(
            serde_json::from_str::<RepoType>("\"\"").unwrap(),
            RepoType::None
        );
        assert_eq!(
            serde_json::from_str::<RepoType>("\"source\"").unwrap(),
            RepoType::Source
        );
        assert_eq!(
            serde_json::from_str::<RepoType>("\"fork\"").unwrap(),
            RepoType::Fork
        );
        assert_eq!(
            serde_json::from_str::<RepoType>("\"mirror\"").unwrap(),
            RepoType::Mirror
        );
        assert_eq!(
            serde_json::from_str::<RepoType>("\"unknown\"").unwrap(),
            RepoType::Unknown
        );
    }

    #[test]
    fn test_trust_model() {
        assert_eq!(
            serde_json::to_string(&TrustModel::Default).unwrap(),
            "\"default\""
        );
        assert_eq!(
            serde_json::to_string(&TrustModel::Collaborator).unwrap(),
            "\"collaborator\""
        );
        assert_eq!(
            serde_json::to_string(&TrustModel::Committer).unwrap(),
            "\"committer\""
        );
        assert_eq!(
            serde_json::to_string(&TrustModel::CollaboratorCommitter).unwrap(),
            "\"collaboratorcommitter\""
        );

        assert_eq!(
            serde_json::from_str::<TrustModel>("\"default\"").unwrap(),
            TrustModel::Default
        );
        assert_eq!(
            serde_json::from_str::<TrustModel>("\"collaborator\"").unwrap(),
            TrustModel::Collaborator
        );
        assert_eq!(
            serde_json::from_str::<TrustModel>("\"committer\"").unwrap(),
            TrustModel::Committer
        );
        assert_eq!(
            serde_json::from_str::<TrustModel>("\"collaboratorcommitter\"").unwrap(),
            TrustModel::CollaboratorCommitter
        );
        assert_eq!(
            serde_json::from_str::<TrustModel>("\"unknown\"").unwrap(),
            TrustModel::Unknown
        );
    }

    #[test]
    fn test_projects_mode() {
        assert_eq!(
            serde_json::to_string(&ProjectsMode::Repo).unwrap(),
            "\"repo\""
        );
        assert_eq!(
            serde_json::to_string(&ProjectsMode::Owner).unwrap(),
            "\"owner\""
        );
        assert_eq!(
            serde_json::to_string(&ProjectsMode::All).unwrap(),
            "\"all\""
        );

        assert_eq!(
            serde_json::from_str::<ProjectsMode>("\"repo\"").unwrap(),
            ProjectsMode::Repo
        );
        assert_eq!(
            serde_json::from_str::<ProjectsMode>("\"owner\"").unwrap(),
            ProjectsMode::Owner
        );
        assert_eq!(
            serde_json::from_str::<ProjectsMode>("\"all\"").unwrap(),
            ProjectsMode::All
        );
        assert_eq!(
            serde_json::from_str::<ProjectsMode>("\"unknown\"").unwrap(),
            ProjectsMode::Unknown
        );
    }

    #[test]
    fn test_hook_type() {
        assert_eq!(
            serde_json::to_string(&HookType::Gitea).unwrap(),
            "\"gitea\""
        );
        assert_eq!(
            serde_json::to_string(&HookType::Slack).unwrap(),
            "\"slack\""
        );
        assert_eq!(
            serde_json::to_string(&HookType::Discord).unwrap(),
            "\"discord\""
        );
        assert_eq!(
            serde_json::to_string(&HookType::Dingtalk).unwrap(),
            "\"dingtalk\""
        );
        assert_eq!(
            serde_json::to_string(&HookType::Telegram).unwrap(),
            "\"telegram\""
        );
        assert_eq!(
            serde_json::to_string(&HookType::Msteams).unwrap(),
            "\"msteams\""
        );
        assert_eq!(
            serde_json::to_string(&HookType::Feishu).unwrap(),
            "\"feishu\""
        );
        assert_eq!(serde_json::to_string(&HookType::Gogs).unwrap(), "\"gogs\"");

        assert_eq!(
            serde_json::from_str::<HookType>("\"gitea\"").unwrap(),
            HookType::Gitea
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"slack\"").unwrap(),
            HookType::Slack
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"discord\"").unwrap(),
            HookType::Discord
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"dingtalk\"").unwrap(),
            HookType::Dingtalk
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"telegram\"").unwrap(),
            HookType::Telegram
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"msteams\"").unwrap(),
            HookType::Msteams
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"feishu\"").unwrap(),
            HookType::Feishu
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"gogs\"").unwrap(),
            HookType::Gogs
        );
        assert_eq!(
            serde_json::from_str::<HookType>("\"unknown\"").unwrap(),
            HookType::Unknown
        );
    }

    #[test]
    fn test_status_state() {
        assert_eq!(
            serde_json::to_string(&StatusState::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&StatusState::Success).unwrap(),
            "\"success\""
        );
        assert_eq!(
            serde_json::to_string(&StatusState::Error).unwrap(),
            "\"error\""
        );
        assert_eq!(
            serde_json::to_string(&StatusState::Failure).unwrap(),
            "\"failure\""
        );
        assert_eq!(
            serde_json::to_string(&StatusState::Warning).unwrap(),
            "\"warning\""
        );

        assert_eq!(
            serde_json::from_str::<StatusState>("\"pending\"").unwrap(),
            StatusState::Pending
        );
        assert_eq!(
            serde_json::from_str::<StatusState>("\"success\"").unwrap(),
            StatusState::Success
        );
        assert_eq!(
            serde_json::from_str::<StatusState>("\"error\"").unwrap(),
            StatusState::Error
        );
        assert_eq!(
            serde_json::from_str::<StatusState>("\"failure\"").unwrap(),
            StatusState::Failure
        );
        assert_eq!(
            serde_json::from_str::<StatusState>("\"warning\"").unwrap(),
            StatusState::Warning
        );
        assert_eq!(
            serde_json::from_str::<StatusState>("\"unknown\"").unwrap(),
            StatusState::Unknown
        );
    }

    #[test]
    fn test_merge_style() {
        assert_eq!(
            serde_json::to_string(&MergeStyle::Merge).unwrap(),
            "\"merge\""
        );
        assert_eq!(
            serde_json::to_string(&MergeStyle::Rebase).unwrap(),
            "\"rebase\""
        );
        assert_eq!(
            serde_json::to_string(&MergeStyle::RebaseMerge).unwrap(),
            "\"rebase-merge\""
        );
        assert_eq!(
            serde_json::to_string(&MergeStyle::Squash).unwrap(),
            "\"squash\""
        );

        assert_eq!(
            serde_json::from_str::<MergeStyle>("\"merge\"").unwrap(),
            MergeStyle::Merge
        );
        assert_eq!(
            serde_json::from_str::<MergeStyle>("\"rebase\"").unwrap(),
            MergeStyle::Rebase
        );
        assert_eq!(
            serde_json::from_str::<MergeStyle>("\"rebase-merge\"").unwrap(),
            MergeStyle::RebaseMerge
        );
        assert_eq!(
            serde_json::from_str::<MergeStyle>("\"squash\"").unwrap(),
            MergeStyle::Squash
        );
        assert_eq!(
            serde_json::from_str::<MergeStyle>("\"unknown\"").unwrap(),
            MergeStyle::Unknown
        );
    }

    #[test]
    fn test_access_token_scope() {
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::All).unwrap(),
            "\"all\""
        );
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::Repo).unwrap(),
            "\"repo\""
        );
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::RepoStatus).unwrap(),
            "\"repo:status\""
        );
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::AdminOrg).unwrap(),
            "\"admin:org\""
        );
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::WriteOrg).unwrap(),
            "\"write:org\""
        );
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::ReadOrg).unwrap(),
            "\"read:org\""
        );
        assert_eq!(
            serde_json::to_string(&AccessTokenScope::Sudo).unwrap(),
            "\"sudo\""
        );

        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"all\"").unwrap(),
            AccessTokenScope::All
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"repo\"").unwrap(),
            AccessTokenScope::Repo
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"repo:status\"").unwrap(),
            AccessTokenScope::RepoStatus
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"admin:org\"").unwrap(),
            AccessTokenScope::AdminOrg
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"write:org\"").unwrap(),
            AccessTokenScope::WriteOrg
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"read:org\"").unwrap(),
            AccessTokenScope::ReadOrg
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"sudo\"").unwrap(),
            AccessTokenScope::Sudo
        );
        assert_eq!(
            serde_json::from_str::<AccessTokenScope>("\"unknown\"").unwrap(),
            AccessTokenScope::Unknown
        );
    }

    #[test]
    fn test_archive_type() {
        assert_eq!(
            serde_json::to_string(&ArchiveType::Zip).unwrap(),
            "\".zip\""
        );
        assert_eq!(
            serde_json::to_string(&ArchiveType::TarGz).unwrap(),
            "\".tar.gz\""
        );

        assert_eq!(
            serde_json::from_str::<ArchiveType>("\".zip\"").unwrap(),
            ArchiveType::Zip
        );
        assert_eq!(
            serde_json::from_str::<ArchiveType>("\".tar.gz\"").unwrap(),
            ArchiveType::TarGz
        );
        assert_eq!(
            serde_json::from_str::<ArchiveType>("\"unknown\"").unwrap(),
            ArchiveType::Unknown
        );
    }

    #[test]
    fn test_git_service_type() {
        assert_eq!(
            serde_json::to_string(&GitServiceType::Git).unwrap(),
            "\"git\""
        );
        assert_eq!(
            serde_json::to_string(&GitServiceType::Github).unwrap(),
            "\"github\""
        );
        assert_eq!(
            serde_json::to_string(&GitServiceType::Gitea).unwrap(),
            "\"gitea\""
        );
        assert_eq!(
            serde_json::to_string(&GitServiceType::Gitlab).unwrap(),
            "\"gitlab\""
        );
        assert_eq!(
            serde_json::to_string(&GitServiceType::Gogs).unwrap(),
            "\"gogs\""
        );

        assert_eq!(
            serde_json::from_str::<GitServiceType>("\"git\"").unwrap(),
            GitServiceType::Git
        );
        assert_eq!(
            serde_json::from_str::<GitServiceType>("\"github\"").unwrap(),
            GitServiceType::Github
        );
        assert_eq!(
            serde_json::from_str::<GitServiceType>("\"gitea\"").unwrap(),
            GitServiceType::Gitea
        );
        assert_eq!(
            serde_json::from_str::<GitServiceType>("\"gitlab\"").unwrap(),
            GitServiceType::Gitlab
        );
        assert_eq!(
            serde_json::from_str::<GitServiceType>("\"gogs\"").unwrap(),
            GitServiceType::Gogs
        );
        assert_eq!(
            serde_json::from_str::<GitServiceType>("\"unknown\"").unwrap(),
            GitServiceType::Unknown
        );
    }

    #[test]
    fn test_strum_display() {
        assert_eq!(StateType::Open.to_string(), "open");
        assert_eq!(StateType::Closed.to_string(), "closed");
        assert_eq!(IssueType::Issue.to_string(), "issues");
        assert_eq!(ReviewStateType::Approved.to_string(), "APPROVED");
        assert_eq!(RepoUnitType::Code.to_string(), "repo.code");
        assert_eq!(AccessTokenScope::AdminOrg.to_string(), "admin:org");
        assert_eq!(ArchiveType::Zip.to_string(), ".zip");
    }

    #[test]
    fn test_strum_as_ref_str() {
        assert_eq!(StateType::Open.as_ref(), "open");
        assert_eq!(IssueType::Issue.as_ref(), "issues");
        assert_eq!(ReviewStateType::Approved.as_ref(), "APPROVED");
        assert_eq!(RepoUnitType::Code.as_ref(), "repo.code");
    }

    #[test]
    fn test_copy() {
        let a = StateType::Open;
        let b = a;
        assert_eq!(a, b);
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(StateType::Open, StateType::Open);
        assert_ne!(StateType::Open, StateType::Closed);
    }
}
