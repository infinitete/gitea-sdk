// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Integration tests for SSH agent connectivity.
//!
//! These tests verify graceful failure when no SSH agent is available.
//! All tests unset `SSH_AUTH_SOCK` to ensure a clean state.

use std::path::Path;

struct SockGuard {
    original: Option<String>,
}

impl SockGuard {
    fn remove() -> Self {
        let original = std::env::var("SSH_AUTH_SOCK").ok();
        unsafe { std::env::remove_var("SSH_AUTH_SOCK") };
        Self { original }
    }
}

impl Drop for SockGuard {
    fn drop(&mut self) {
        if let Some(ref val) = self.original {
            unsafe { std::env::set_var("SSH_AUTH_SOCK", val) };
        }
    }
}

#[test]
fn test_ssh_agent_connect_no_agent() {
    let _guard = SockGuard::remove();

    let sock_path = std::env::var("SSH_AUTH_SOCK");
    assert!(
        sock_path.is_err(),
        "SSH_AUTH_SOCK should not be set after guard removes it"
    );

    let path = Path::new("/nonexistent/agent.sock");
    let result = ssh_agent_client_rs::Client::connect(path);
    assert!(
        result.is_err(),
        "connecting to nonexistent agent socket should fail gracefully"
    );
}

#[test]
fn test_ssh_agent_list_identities_no_agent() {
    let _guard = SockGuard::remove();

    let path = Path::new("/nonexistent/agent.sock");
    let result = ssh_agent_client_rs::Client::connect(path);
    match result {
        Err(_) => {}
        Ok(mut client) => {
            let identities = client.list_identities();
            assert!(
                identities.is_err(),
                "list_identities on broken connection should fail"
            );
        }
    }
}
