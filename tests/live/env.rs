// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Configuration for the live Gitea instance.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LiveEnv {
    pub host: String,
    pub http_port: String,
    pub ssh_port: String,
    pub user_name: String,
    pub user_pass: String,
    pub next_user_name: Option<String>,
    pub next_user_pass: Option<String>,
    pub token_name: Option<String>,
    pub token_value: String,
    pub group_name: Option<String>,
    pub repository: Option<String>,
    pub rsa_public_key: Option<String>,
    pub ed25519_public_key: Option<String>,
}

impl LiveEnv {
    pub fn base_url(&self) -> String {
        format!("http://{}:{}", self.host, self.http_port)
    }

    #[allow(dead_code)]
    pub fn ssh_host(&self) -> String {
        format!("{}:{}", self.host, self.ssh_port)
    }

    #[allow(dead_code)]
    pub fn has_next_user(&self) -> bool {
        self.next_user_name.is_some() && self.next_user_pass.is_some()
    }

    #[allow(dead_code)]
    pub fn next_user_credentials(&self) -> (&str, &str) {
        (
            self.next_user_name
                .as_deref()
                .expect("missing GITEA_NEXT_USER_NAME"),
            self.next_user_pass
                .as_deref()
                .expect("missing GITEA_NEXT_USER_PASS"),
        )
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rust-sdk should live under repository root")
        .to_path_buf()
}

fn parse_env(contents: &str) -> HashMap<String, String> {
    contents
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                return None;
            }

            trimmed.split_once('=').map(|(key, value)| {
                (
                    key.trim().to_string(),
                    value.trim().trim_matches('"').to_string(),
                )
            })
        })
        .collect()
}

fn required_value(values: &HashMap<String, String>, key: &str, path: &Path) -> String {
    values
        .get(key)
        .cloned()
        .unwrap_or_else(|| panic!("missing {key} in {}", path.display()))
}

fn optional_value(values: &HashMap<String, String>, key: &str) -> Option<String> {
    values
        .get(key)
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
}

pub fn load_live_env() -> &'static LiveEnv {
    static ENV: OnceLock<LiveEnv> = OnceLock::new();
    ENV.get_or_init(|| {
        let env_path = repo_root().join(".env");
        let contents = fs::read_to_string(&env_path)
            .unwrap_or_else(|err| panic!("failed to read {}: {}", env_path.display(), err));
        let values = parse_env(&contents);

        LiveEnv {
            host: required_value(&values, "GITEA_HOST", &env_path),
            http_port: required_value(&values, "GITEA_HTTP_PORT", &env_path),
            ssh_port: required_value(&values, "GITEA_SSH_PORT", &env_path),
            user_name: required_value(&values, "GITEA_USER_NAME", &env_path),
            user_pass: required_value(&values, "GITEA_USER_PASS", &env_path),
            next_user_name: optional_value(&values, "GITEA_NEXT_USER_NAME"),
            next_user_pass: optional_value(&values, "GITEA_NEXT_USER_PASS"),
            token_name: optional_value(&values, "GITEA_TOKEN_NAME"),
            token_value: required_value(&values, "GITEA_TOKEN_VALUE", &env_path),
            group_name: optional_value(&values, "GITEA_GROUP_NAME"),
            repository: optional_value(&values, "GITEA_REPOSITORY"),
            rsa_public_key: optional_value(&values, "RSA_PUBLIC_KEY"),
            ed25519_public_key: optional_value(&values, "ED25519_PUBLIC_KEY"),
        }
    })
}
