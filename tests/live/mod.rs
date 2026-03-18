// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#![allow(unused_imports)]

//! Shared helpers for live integration tests against a real Gitea instance.

pub mod cleanup;
pub mod client;
pub mod env;
pub mod fixtures;
pub mod keys;
pub mod naming;

pub use cleanup::CleanupRegistry;
pub use client::{
    build_basic_auth_client, build_live_client, build_next_user_basic_auth_client, live_client,
    next_user_client,
};
pub use env::load_live_env;
pub use fixtures::{
    create_generic_package_fixture, create_issue_fixture, create_label_fixture, create_org_fixture,
    create_org_repo_fixture, create_release_fixture, create_repo_fixture,
    enable_issue_dependencies, prepare_deploy_key_fixture,
};
pub use keys::{
    generate_fresh_public_key, load_public_key_from_env, load_public_key_from_repo,
    load_unused_or_generate_public_key,
};
pub use naming::unique_name;
