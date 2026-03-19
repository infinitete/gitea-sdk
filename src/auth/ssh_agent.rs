// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! SSH agent authentication for the Gitea API.

use ssh_key::{Certificate, Fingerprint, HashAlg, PublicKey};

use crate::Error;

#[allow(dead_code)]
/// An identity (public key) held by the SSH agent.
#[derive(Debug, Clone)]
pub(crate) struct SshIdentity {
    /// The SSH public key returned by the agent.
    key: PublicKey,
}

#[allow(dead_code)]
impl SshIdentity {
    /// Returns the underlying SSH public key.
    pub(crate) fn public_key(&self) -> &PublicKey {
        &self.key
    }

    /// Returns the key's comment (if any).
    pub(crate) fn comment(&self) -> &str {
        self.key.comment()
    }

    /// Returns the SHA-256 fingerprint of this identity.
    pub(crate) fn fingerprint(&self) -> Fingerprint {
        self.key.fingerprint(HashAlg::Sha256)
    }

    /// Returns `true` if this identity is an SSH certificate (rather than a plain
    /// public key). Certificate-type keys have algorithm names ending in
    /// `-cert-v01@openssh.com`.
    pub(crate) fn is_certificate(&self) -> bool {
        self.key.algorithm().as_str().contains("-cert-")
    }

    /// Attempts to parse this identity as an SSH certificate.
    ///
    /// Returns `Some(certificate)` if the underlying key is a certificate type,
    /// or `None` if it is a plain public key or parsing fails.
    pub(crate) fn as_certificate(&self) -> Option<Certificate> {
        if !self.is_certificate() {
            return None;
        }
        let bytes = self.key.to_bytes().ok()?;
        Certificate::from_bytes(&bytes).ok()
    }
}

#[allow(dead_code)]
/// Connects to the local SSH agent via the `SSH_AUTH_SOCK` environment variable.
///
/// Returns an error with [`Error::SshSign`] if `SSH_AUTH_SOCK` is not set or the
/// connection fails.
pub(crate) fn connect() -> crate::Result<SshAgent> {
    let sock_path = std::env::var("SSH_AUTH_SOCK")
        .map_err(|_| Error::SshSign("SSH_AUTH_SOCK not set".into()))?;
    let path = std::path::Path::new(&sock_path);
    let client =
        ssh_agent_client_rs::Client::connect(path).map_err(|e| Error::SshSign(e.to_string()))?;
    Ok(SshAgent {
        client: parking_lot::Mutex::new(client),
    })
}

#[allow(dead_code)]
/// An SSH agent connection for listing identities and signing data.
///
/// Wraps the [`ssh_agent_client_rs::Client`] with interior mutability so it can
/// be shared across async contexts.
pub(crate) struct SshAgent {
    client: parking_lot::Mutex<ssh_agent_client_rs::Client>,
}

#[allow(dead_code)]
impl SshAgent {
    /// Lists all identities (public keys) held by the SSH agent.
    pub(crate) fn list_identities(&self) -> crate::Result<Vec<SshIdentity>> {
        let mut client = self.client.lock();
        let keys = client
            .list_identities()
            .map_err(|e| Error::SshSign(format!("failed to list identities: {e}")))?;
        Ok(keys.into_iter().map(|key| SshIdentity { key }).collect())
    }

    /// Signs the given data using the specified identity held by the agent.
    ///
    /// Returns the [`ssh_key::Signature`] produced by the agent.
    pub(crate) fn sign_data(
        &self,
        identity: &SshIdentity,
        data: &[u8],
    ) -> crate::Result<ssh_key::Signature> {
        let mut client = self.client.lock();
        let signature = client
            .sign(identity.public_key(), data)
            .map_err(|e| Error::SshSign(format!("failed to sign data: {e}")))?;
        Ok(signature)
    }

    /// Disconnects from the SSH agent.
    ///
    /// The underlying Unix socket is closed when this `SshAgent` is dropped;
    /// this method provides an explicit way to disconnect.
    pub(crate) fn disconnect(self) -> crate::Result<()> {
        drop(self.client);
        Ok(())
    }

    /// Finds an SSH certificate identity matching the given principal.
    ///
    /// Iterates through all agent identities and returns the first certificate
    /// whose `valid_principals` list contains `principal`.
    pub(crate) fn find_cert_signer(&self, principal: &str) -> crate::Result<Option<SshIdentity>> {
        let identities = self.list_identities()?;
        for identity in &identities {
            if let Some(cert) = identity.as_certificate()
                && cert.valid_principals().iter().any(|p| p == principal)
            {
                return Ok(Some(identity.clone()));
            }
        }
        Ok(None)
    }

    /// Finds an SSH public key identity matching the given SHA-256 fingerprint.
    ///
    /// `fingerprint` should be in the format `SHA256:xxxxx...` (as produced by
    /// `ssh-keygen -l`). The `SHA256:` prefix is optional.
    pub(crate) fn find_pubkey_signer(
        &self,
        fingerprint: &str,
    ) -> crate::Result<Option<SshIdentity>> {
        let identities = self.list_identities()?;
        let normalized = fingerprint.strip_prefix("SHA256:").unwrap_or(fingerprint);
        for identity in identities {
            let key_fp = identity.fingerprint().to_string();
            // Compare both with and without SHA256: prefix
            if key_fp == normalized || key_fp == fingerprint {
                return Ok(Some(identity));
            }
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to safely remove and restore `SSH_AUTH_SOCK` in tests.
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
    fn test_connect_missing_ssh_auth_sock() {
        let _guard = SockGuard::remove();

        let result = connect();
        assert!(result.is_err(), "expected error when SSH_AUTH_SOCK not set");
        let msg = result.err().expect("already asserted err").to_string();
        assert!(
            msg.contains("SSH_AUTH_SOCK not set"),
            "expected 'SSH_AUTH_SOCK not set', got: {msg}"
        );
    }

    #[test]
    fn test_ssh_sign_error_type() {
        let err = Error::SshSign("test error".into());
        assert!(err.to_string().contains("SSH signing error"));
        assert!(err.to_string().contains("test error"));
    }

    #[test]
    fn test_find_cert_signer_no_agent() {
        let _guard = SockGuard::remove();
        let result = connect();
        match result {
            Err(e) => assert!(e.to_string().contains("SSH_AUTH_SOCK not set")),
            Ok(agent) => {
                let _ = agent.find_cert_signer("nonexistent-principal");
            }
        }
    }

    #[test]
    fn test_find_pubkey_signer_no_agent() {
        let _guard = SockGuard::remove();
        let result = connect();
        match result {
            Err(e) => assert!(e.to_string().contains("SSH_AUTH_SOCK not set")),
            Ok(agent) => {
                let _ = agent.find_pubkey_signer("SHA256:nonexistent");
            }
        }
    }

    #[test]
    fn test_ssh_identity_is_not_certificate_for_plain_key() {
        let private_key = ssh_key::PrivateKey::from_openssh(include_str!(
            "../../tests/ssh_fixtures/id_ed25519_test"
        ))
        .expect("test key should parse");
        let identity = SshIdentity {
            key: private_key.public_key().clone(),
        };

        assert!(
            !identity.is_certificate(),
            "plain ed25519 key should not be a certificate"
        );
        assert!(identity.as_certificate().is_none());
        assert!(
            identity.fingerprint().to_string().starts_with("SHA256:"),
            "fingerprint should start with SHA256:"
        );
    }

    #[test]
    fn test_find_pubkey_signer_empty_list() {
        let _guard = SockGuard::remove();
        let _ = connect();
    }
}
