// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#![allow(dead_code)]

use gitea_sdk::Client;

use super::env::{LiveEnv, load_live_env};

pub fn base_url(env: &LiveEnv) -> String {
    env.base_url()
}

pub fn build_live_client(env: &LiveEnv) -> Client {
    Client::builder(&base_url(env))
        .token(env.token_value.clone())
        .build()
        .expect("build live client")
}

pub fn build_basic_auth_client(env: &LiveEnv) -> Client {
    Client::builder(&base_url(env))
        .basic_auth(env.user_name.clone(), env.user_pass.clone())
        .build()
        .expect("build live basic-auth client")
}

pub fn build_next_user_basic_auth_client(env: &LiveEnv) -> Client {
    Client::builder(&base_url(env))
        .basic_auth(
            env.next_user_name
                .clone()
                .expect("missing GITEA_NEXT_USER_NAME in live env"),
            env.next_user_pass
                .clone()
                .expect("missing GITEA_NEXT_USER_PASS in live env"),
        )
        .build()
        .expect("build next-user basic-auth client")
}

pub fn live_client() -> Client {
    build_live_client(load_live_env())
}

pub fn next_user_client() -> Client {
    build_next_user_basic_auth_client(load_live_env())
}
