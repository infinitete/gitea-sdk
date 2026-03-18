// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#![allow(dead_code)]

pub mod actions;
pub mod activitypub;
pub mod admin;
pub mod hooks;
pub mod issues;
pub mod miscellaneous;
pub mod notifications;
pub mod oauth2;
pub mod orgs;
pub mod packages;
pub mod pulls;
pub mod releases;
pub mod repos;
pub mod settings;
pub mod status;
pub mod users;

pub use actions::ActionsApi;
pub use activitypub::ActivityPubApi;
pub use admin::AdminApi;
pub use hooks::HooksApi;
pub use issues::IssuesApi;
pub use miscellaneous::MiscApi;
pub use notifications::NotificationsApi;
pub use oauth2::Oauth2Api;
pub use orgs::OrgsApi;
pub use packages::PackagesApi;
pub use pulls::PullsApi;
pub use releases::ReleasesApi;
pub use repos::ReposApi;
pub use settings::SettingsApi;
pub use status::StatusApi;
pub use users::UsersApi;
