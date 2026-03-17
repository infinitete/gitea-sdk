//! Gitea API client for Rust.
//!
//! This crate provides an asynchronous client for the Gitea API.
//!
//! # Quick Start
//!
//! ```ignore
//! use gitea_sdk::Client;
//!
//! let client = Client::builder("https://gitea.example.com")
//!     .token("your-token")
//!     .build()?;
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
    AccessToken, Activity, Attachment, Badge, CombinedStatus, Comment, CronTask, Email, GPGKey,
    GPGKeyEmail, GitignoreTemplateInfo, Label, LabelTemplate, Milestone, NodeInfo,
    NodeInfoServices, NodeInfoSoftware, NodeInfoUsage, NodeInfoUsageUsers, NotificationThread,
    NotifySubject, Oauth2, OrgPermissions, Organization, Package, PackageFile, PublicKey, Reaction,
    Release, Secret, Status, Team, User, UserSettings,
};

// Re-export all enums for convenience.
pub use types::enums::*;

// Re-export API sub-structs for convenience.
pub use api::{
    ActionsApi, ActivityPubApi, AdminApi, HooksApi, IssuesApi, MiscApi, NotificationsApi,
    Oauth2Api, OrgsApi, PullsApi, ReleasesApi, ReposApi, SettingsApi, StatusApi, UsersApi,
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
