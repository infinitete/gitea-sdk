// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

#![allow(dead_code)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::env::LiveEnv;
use super::naming::unique_name;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("rust-sdk should live under repository root")
        .to_path_buf()
}

pub fn load_public_key_from_repo(relative_path: &str) -> io::Result<String> {
    fs::read_to_string(repo_root().join(relative_path))
}

pub fn load_public_key_from_env(env: &LiveEnv) -> io::Result<String> {
    if let Some(path) = env.ed25519_public_key.as_deref() {
        return load_public_key_from_repo(path);
    }
    if let Some(path) = env.rsa_public_key.as_deref() {
        return load_public_key_from_repo(path);
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "no public key path configured in live env",
    ))
}

pub fn load_unused_or_generate_public_key(
    env: &LiveEnv,
    existing_keys: &[String],
    prefix: &str,
) -> io::Result<String> {
    for path in [
        env.ed25519_public_key.as_deref(),
        env.rsa_public_key.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        if let Ok(key) = load_public_key_from_repo(path)
            && !existing_keys
                .iter()
                .any(|existing| existing.trim() == key.trim())
        {
            return Ok(key);
        }
    }

    generate_fresh_public_key(prefix)
}

pub fn generate_fresh_public_key(prefix: &str) -> io::Result<String> {
    let temp_base = std::env::temp_dir().join(unique_name(prefix));
    fs::create_dir_all(&temp_base)?;
    let key_path = temp_base.join("id_ed25519");
    let comment = unique_name(prefix);

    let output = Command::new("ssh-keygen")
        .arg("-q")
        .arg("-t")
        .arg("ed25519")
        .arg("-N")
        .arg("")
        .arg("-C")
        .arg(&comment)
        .arg("-f")
        .arg(&key_path)
        .output()?;

    if !output.status.success() {
        return Err(io::Error::other(format!(
            "ssh-keygen failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    fs::read_to_string(key_path.with_extension("pub"))
}
