// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Client construction, configuration, and authentication.

use std::sync::{Arc, OnceLock};

use parking_lot::RwLock;

use crate::Error;
use crate::api::{
    ActionsApi, ActivityPubApi, AdminApi, HooksApi, IssuesApi, MiscApi, NotificationsApi,
    Oauth2Api, OrgsApi, PackagesApi, PullsApi, ReleasesApi, ReposApi, SettingsApi, StatusApi,
    UsersApi,
};

/// Configuration fields that can be mutated at runtime via setters on [`Client`].
#[derive(Clone)]
pub(crate) struct ClientConfig {
    /// Base URL of the Gitea server (trailing slash stripped).
    pub(crate) base_url: String,
    /// Bearer token for API authentication.
    pub(crate) access_token: String,
    /// Username for basic authentication.
    pub(crate) username: String,
    /// Password for basic authentication.
    pub(crate) password: String,
    /// One-time password for 2FA.
    pub(crate) otp: String,
    /// Username to impersonate via the Sudo header.
    pub(crate) sudo: String,
    /// User-Agent header sent with every request.
    pub(crate) user_agent: String,
    /// Whether debug logging is enabled.
    pub(crate) debug: bool,
    /// When `true`, skip all server-version compatibility checks.
    pub(crate) ignore_version: bool,
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("base_url", &self.base_url)
            .field(
                "access_token",
                &if self.access_token.is_empty() {
                    &"" as &dyn std::fmt::Debug
                } else {
                    &"***" as &dyn std::fmt::Debug
                },
            )
            .field("username", &self.username)
            .field("password", &"***")
            .field("otp", &"***")
            .field("sudo", &self.sudo)
            .field("user_agent", &self.user_agent)
            .field("debug", &self.debug)
            .field("ignore_version", &self.ignore_version)
            .finish()
    }
}

struct ClientInner {
    http: RwLock<reqwest::Client>,
    /// Mutable configuration protected by a reader-writer lock.
    config: RwLock<ClientConfig>,
    /// Server version discovered lazily (populated by the first version check).
    server_version: OnceLock<semver::Version>,
    /// Pre-set version supplied via [`ClientBuilder::gitea_version`].
    preset_version: Option<semver::Version>,
    /// Guards against concurrent `load_server_version` fetches.
    version_loading: tokio::sync::Mutex<()>,
    /// SSH signer for HTTP Signature authentication, if configured.
    ssh_signer: RwLock<Option<crate::auth::ssh_sign::SshSigner>>,
}

/// A thread-safe Gitea API client.
///
/// The client wraps an [`Arc<ClientInner>`] so it can be freely cloned and
/// shared across threads. Mutable configuration fields (token, credentials,
/// etc.) are protected by a `parking_lot::RwLock` and can be changed at
/// runtime through the setter methods.
///
/// # Examples
///
/// ```no_run
/// use gitea_sdk::Client;
///
/// # fn main() -> Result<(), gitea_sdk::Error> {
/// let client = Client::builder("https://gitea.example.com")
///     .token("my-secret-token")
///     .build()?;
/// # let _ = client;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
/// Client payload type.
pub struct Client {
    inner: Arc<ClientInner>,
}

impl std::fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client")
            .field("base_url", &self.inner.config.read().base_url)
            .finish()
    }
}

// ── Constructors ────────────────────────────────────────────────────

impl Client {
    /// Create a new [`ClientBuilder`] anchored to `base_url`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Url`] if `base_url` cannot be parsed.
    pub fn builder(base_url: &str) -> ClientBuilder<'_> {
        ClientBuilder::new(base_url)
    }
}

// ── Mutable setters (Go-compatible) ─────────────────────────────────

impl Client {
    /// Replace the access token.
    pub fn set_token(&self, token: impl Into<String>) {
        let mut cfg = self.inner.config.write();
        cfg.access_token = token.into();
        // Clear basic auth when switching to token auth.
        cfg.username.clear();
        cfg.password.clear();
    }

    /// Set username and password for basic authentication.
    pub fn set_basic_auth(&self, username: impl Into<String>, password: impl Into<String>) {
        let mut cfg = self.inner.config.write();
        cfg.username = username.into();
        cfg.password = password.into();
        cfg.access_token.clear();
    }

    /// Set the one-time password for 2FA.
    pub fn set_otp(&self, otp: impl Into<String>) {
        self.inner.config.write().otp = otp.into();
    }

    /// Set the username to impersonate via the Sudo header.
    pub fn set_sudo(&self, sudo: impl Into<String>) {
        self.inner.config.write().sudo = sudo.into();
    }

    /// Set the User-Agent header sent with every request.
    pub fn set_user_agent(&self, agent: impl Into<String>) {
        self.inner.config.write().user_agent = agent.into();
    }

    /// Replace the underlying HTTP client used for requests.
    pub fn set_http_client(&self, client: reqwest::Client) {
        *self.inner.http.write() = client;
    }

    /// Return the configured base URL (trailing slash stripped).
    pub fn base_url(&self) -> String {
        self.inner.config.read().base_url.clone()
    }
}

// ── Internal helpers (crate-visible) ────────────────────────────────

impl Client {
    #[allow(private_interfaces)]
    pub(crate) fn read_config(&self) -> parking_lot::RwLockReadGuard<'_, ClientConfig> {
        self.inner.config.read()
    }

    /// Acquire a write lock on the configuration.
    #[allow(dead_code)]
    fn write_config(&self) -> parking_lot::RwLockWriteGuard<'_, ClientConfig> {
        self.inner.config.write()
    }

    pub(crate) fn http_client(&self) -> reqwest::Client {
        self.inner.http.read().clone()
    }

    /// Borrow the [`OnceLock`] that will hold the server version once
    /// discovered.
    pub(crate) fn server_version_lock(&self) -> &OnceLock<semver::Version> {
        &self.inner.server_version
    }

    /// The version pre-set via [`ClientBuilder::gitea_version`], if any.
    pub(crate) fn preset_version(&self) -> &Option<semver::Version> {
        &self.inner.preset_version
    }

    /// Whether version compatibility checks should be skipped entirely.
    pub(crate) fn ignore_version(&self) -> bool {
        self.inner.config.read().ignore_version
    }

    pub(crate) async fn version_loading_lock(&self) -> tokio::sync::MutexGuard<'_, ()> {
        self.inner.version_loading.lock().await
    }

    pub(crate) fn ssh_signer(
        &self,
    ) -> parking_lot::RwLockReadGuard<'_, Option<crate::auth::ssh_sign::SshSigner>> {
        self.inner.ssh_signer.read()
    }

    /// Determine whether to use legacy HTTP Signature format for SSH signing.
    ///
    /// Returns `true` (use legacy) when:
    /// - Version checks are disabled (`ignore_version`), or
    /// - The server version is known to be < 1.23.0.
    ///
    /// Returns `false` (use modern) when:
    /// - The server version is known to be >= 1.23.0, or
    /// - The version is unknown (optimistically assume modern).
    pub(crate) fn should_use_legacy_ssh(&self) -> bool {
        if self.ignore_version() {
            return true;
        }
        if let Some(v) = self.preset_version() {
            return v < &*crate::version::VERSION_1_23_0;
        }
        if let Some(v) = self.server_version_lock().get() {
            return v < &*crate::version::VERSION_1_23_0;
        }
        false
    }
}

// ── API accessor methods ────────────────────────────────────────────

impl Client {
    /// Access repository API methods.
    pub fn repos(&self) -> ReposApi<'_> {
        ReposApi::new(self)
    }

    /// Access issue API methods.
    pub fn issues(&self) -> IssuesApi<'_> {
        IssuesApi::new(self)
    }

    /// Access pull request API methods.
    pub fn pulls(&self) -> PullsApi<'_> {
        PullsApi::new(self)
    }

    /// Access organization API methods.
    pub fn orgs(&self) -> OrgsApi<'_> {
        OrgsApi::new(self)
    }

    /// Access user API methods.
    pub fn users(&self) -> UsersApi<'_> {
        UsersApi::new(self)
    }

    /// Access admin API methods.
    pub fn admin(&self) -> AdminApi<'_> {
        AdminApi::new(self)
    }

    /// Access webhook API methods.
    pub fn hooks(&self) -> HooksApi<'_> {
        HooksApi::new(self)
    }

    /// Access notification API methods.
    pub fn notifications(&self) -> NotificationsApi<'_> {
        NotificationsApi::new(self)
    }

    /// Access actions API methods.
    pub fn actions(&self) -> ActionsApi<'_> {
        ActionsApi::new(self)
    }

    /// Access release API methods.
    pub fn releases(&self) -> ReleasesApi<'_> {
        ReleasesApi::new(self)
    }

    /// Access settings API methods.
    pub fn settings(&self) -> SettingsApi<'_> {
        SettingsApi::new(self)
    }

    /// Access OAuth2 application API methods.
    pub fn oauth2(&self) -> Oauth2Api<'_> {
        Oauth2Api::new(self)
    }

    /// Access package API methods.
    pub fn packages(&self) -> PackagesApi<'_> {
        PackagesApi::new(self)
    }

    /// Access miscellaneous API methods.
    pub fn miscellaneous(&self) -> MiscApi<'_> {
        MiscApi::new(self)
    }

    /// Access ActivityPub API methods.
    pub fn activitypub(&self) -> ActivityPubApi<'_> {
        ActivityPubApi::new(self)
    }

    /// Access commit status API methods.
    pub fn status(&self) -> StatusApi<'_> {
        StatusApi::new(self)
    }
}

// ── ClientBuilder ───────────────────────────────────────────────────

/// Fluent builder for constructing a [`Client`].
///
/// Call [`Client::builder`] to obtain an instance, chain setter methods, and
/// finalize with [`.build()`](ClientBuilder::build).
pub struct ClientBuilder<'a> {
    base_url: &'a str,
    access_token: Option<String>,
    username: Option<String>,
    password: Option<String>,
    otp: Option<String>,
    sudo: Option<String>,
    user_agent: Option<String>,
    debug: bool,
    ignore_version: bool,
    raw_preset_version: Option<String>,
    preset_version: Option<semver::Version>,
    http_client: Option<reqwest::Client>,
    ssh_signer: Option<crate::auth::ssh_sign::SshSigner>,
}

impl std::fmt::Debug for ClientBuilder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientBuilder")
            .field("base_url", &self.base_url)
            .field("access_token", &self.access_token.as_ref().map(|_| "***"))
            .field("username", &self.username)
            .field("password", &"***")
            .field("otp", &"***")
            .field("sudo", &self.sudo)
            .field("user_agent", &self.user_agent)
            .field("debug", &self.debug)
            .field("ignore_version", &self.ignore_version)
            .field("raw_preset_version", &self.raw_preset_version)
            .field("preset_version", &self.preset_version)
            .field("http_client", &self.http_client)
            .finish()
    }
}

impl<'a> ClientBuilder<'a> {
    /// Create a new builder for the given `base_url`.
    ///
    /// The URL is validated eagerly with [`url::Url::parse`].
    pub fn new(base_url: &'a str) -> Self {
        Self {
            base_url,
            access_token: None,
            username: None,
            password: None,
            otp: None,
            sudo: None,
            user_agent: None,
            debug: false,
            ignore_version: false,
            raw_preset_version: None,
            preset_version: None,
            http_client: None,
            ssh_signer: None,
        }
    }

    /// Set the bearer access token.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.access_token = Some(token.into());
        self
    }

    /// Set username and password for basic authentication.
    pub fn basic_auth(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    /// Set the one-time password for 2FA.
    pub fn otp(mut self, otp: impl Into<String>) -> Self {
        self.otp = Some(otp.into());
        self
    }

    /// Set the username to impersonate via the Sudo header.
    pub fn sudo(mut self, sudo: impl Into<String>) -> Self {
        self.sudo = Some(sudo.into());
        self
    }

    /// Set the User-Agent header sent with every request.
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    /// Configure the assumed Gitea server version.
    ///
    /// * **Empty string** — tells the SDK to skip all version compatibility
    ///   checks (equivalent to Go's `SetGiteaVersion("")`).
    /// * **Non-empty string** — parsed as a semantic version and used in
    ///   place of the version discovered from the server.
    pub fn gitea_version(mut self, version: &str) -> Self {
        if version.is_empty() {
            self.ignore_version = true;
            self.raw_preset_version = None;
            self.preset_version = None;
        } else {
            self.ignore_version = false;
            self.raw_preset_version = Some(version.to_string());
            self.preset_version = version.parse::<semver::Version>().ok();
        }
        self
    }

    /// Enable or disable debug logging.
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Provide a custom [`reqwest::Client`].
    ///
    /// If not called, a default client is created by [`build()`](Self::build).
    pub fn http_client(mut self, client: reqwest::Client) -> Self {
        self.http_client = Some(client);
        self
    }

    /// Configure SSH certificate-based authentication.
    ///
    /// Reads the private key from `key_path` and stores a certificate signer
    /// with the given `principal`. Optionally reads the certificate from
    /// `cert_path` (needed for `x-ssh-certificate` header).
    ///
    /// # Errors
    ///
    /// Returns [`Error::SshSign`] if the key file cannot be read, parsed, or decrypted.
    pub fn ssh_cert<P: AsRef<std::path::Path>>(
        mut self,
        principal: impl Into<String>,
        key_path: P,
        passphrase: Option<&str>,
    ) -> crate::Result<Self> {
        let key = crate::auth::ssh_sign::load_private_key(key_path.as_ref(), passphrase)?;
        self.ssh_signer = Some(crate::auth::ssh_sign::SshSigner::Cert {
            principal: principal.into(),
            key,
            certificate_bytes: None,
        });
        Ok(self)
    }

    /// Configure SSH certificate-based authentication with a certificate file.
    ///
    /// Like [`Self::ssh_cert`] but also reads the OpenSSH certificate from
    /// `cert_path` and includes it as the `x-ssh-certificate` header in
    /// signed requests.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SshSign`] if the key or certificate file cannot be read.
    pub fn ssh_cert_with_certificate<P1, P2>(
        mut self,
        principal: impl Into<String>,
        key_path: P1,
        cert_path: P2,
        passphrase: Option<&str>,
    ) -> crate::Result<Self>
    where
        P1: AsRef<std::path::Path>,
        P2: AsRef<std::path::Path>,
    {
        let key = crate::auth::ssh_sign::load_private_key(key_path.as_ref(), passphrase)?;
        let cert_bytes = std::fs::read(cert_path.as_ref()).map_err(|e| {
            crate::Error::SshSign(format!(
                "failed to read {}: {e}",
                cert_path.as_ref().display()
            ))
        })?;
        self.ssh_signer = Some(crate::auth::ssh_sign::SshSigner::Cert {
            principal: principal.into(),
            key,
            certificate_bytes: Some(cert_bytes),
        });
        Ok(self)
    }

    /// Configure SSH public key-based authentication.
    ///
    /// Reads the private key from `key_path` and stores a public-key signer
    /// with the given `fingerprint`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SshSign`] if the key file cannot be read, parsed, or decrypted.
    pub fn ssh_pubkey<P: AsRef<std::path::Path>>(
        mut self,
        fingerprint: impl Into<String>,
        key_path: P,
        passphrase: Option<&str>,
    ) -> crate::Result<Self> {
        let key = crate::auth::ssh_sign::load_private_key(key_path.as_ref(), passphrase)?;
        self.ssh_signer = Some(crate::auth::ssh_sign::SshSigner::Pubkey {
            fingerprint: fingerprint.into(),
            key,
        });
        Ok(self)
    }

    /// Consume the builder and produce a [`Client`].
    ///
    /// # Errors
    ///
    /// * [`Error::Url`] — `base_url` is not a valid URL.
    /// * [`Error::Validation`] — `base_url` does not use `http` or `https`.
    /// * [`Error::Version`] — `gitea_version` was provided but is not valid semver.
    pub fn build(self) -> crate::Result<Client> {
        // Validate URL.
        let parsed = url::Url::parse(self.base_url)?;

        match parsed.scheme() {
            "http" | "https" => {}
            other => {
                return Err(Error::Validation(format!(
                    "base_url must use http or https, got: {other}"
                )));
            }
        }

        if let Some(raw_version) = self.raw_preset_version.as_deref()
            && self.preset_version.is_none()
        {
            return Err(Error::Version(format!(
                "invalid Gitea version '{raw_version}'"
            )));
        }

        // Strip trailing slash to match Go SDK behaviour.
        let base_url = parsed.as_str().trim_end_matches('/').to_string();

        let http = self.http_client.unwrap_or_default();

        let config = ClientConfig {
            base_url,
            access_token: self.access_token.unwrap_or_default(),
            username: self.username.unwrap_or_default(),
            password: self.password.unwrap_or_default(),
            otp: self.otp.unwrap_or_default(),
            sudo: self.sudo.unwrap_or_default(),
            user_agent: self.user_agent.unwrap_or_default(),
            debug: self.debug,
            ignore_version: self.ignore_version,
        };

        Ok(Client {
            inner: Arc::new(ClientInner {
                http: RwLock::new(http),
                config: RwLock::new(config),
                server_version: OnceLock::new(),
                preset_version: self.preset_version,
                version_loading: tokio::sync::Mutex::new(()),
                ssh_signer: RwLock::new(self.ssh_signer),
            }),
        })
    }
}

// ── Tests ───────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<Client>();
    }

    #[test]
    fn test_client_build_token() {
        let client = Client::builder("https://example.com")
            .token("abc123")
            .build()
            .unwrap();
        assert_eq!(client.base_url(), "https://example.com");
        let cfg = client.read_config();
        assert_eq!(cfg.access_token, "abc123");
        assert!(cfg.username.is_empty());
        assert!(cfg.password.is_empty());
    }

    #[test]
    fn test_client_build_basic_auth() {
        let client = Client::builder("https://example.com")
            .basic_auth("user", "pass")
            .build()
            .unwrap();
        let cfg = client.read_config();
        assert_eq!(cfg.username, "user");
        assert_eq!(cfg.password, "pass");
        assert!(cfg.access_token.is_empty());
    }

    #[test]
    fn test_client_build_invalid_url() {
        let result = Client::builder("not-a-url").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_client_build_invalid_scheme() {
        let result = Client::builder("ftp://example.com").build();
        assert!(result.is_err());
    }

    #[test]
    fn test_client_setters() {
        let client = Client::builder("https://example.com")
            .token("initial")
            .build()
            .unwrap();

        client.set_token("new-token");
        assert_eq!(client.read_config().access_token, "new-token");

        client.set_basic_auth("admin", "secret");
        {
            let cfg = client.read_config();
            assert_eq!(cfg.username, "admin");
            assert_eq!(cfg.password, "secret");
            assert!(cfg.access_token.is_empty());
        }

        client.set_otp("123456");
        assert_eq!(client.read_config().otp, "123456");

        client.set_sudo("target-user");
        assert_eq!(client.read_config().sudo, "target-user");

        client.set_user_agent("my-sdk/0.1");
        assert_eq!(client.read_config().user_agent, "my-sdk/0.1");
    }

    #[test]
    fn test_client_gitea_version_ignore() {
        let client = Client::builder("https://example.com")
            .gitea_version("")
            .build()
            .unwrap();
        assert!(client.ignore_version());
        assert!(client.preset_version().is_none());
    }

    #[test]
    fn test_client_gitea_version_preset() {
        let client = Client::builder("https://example.com")
            .gitea_version("1.22.0")
            .build()
            .unwrap();
        assert!(!client.ignore_version());
        assert_eq!(
            client.preset_version().as_ref().map(|v| v.to_string()),
            Some("1.22.0".to_string()),
        );
    }

    #[test]
    fn test_client_builder_url_trailing_slash() {
        let client = Client::builder("https://example.com/").build().unwrap();
        assert_eq!(client.base_url(), "https://example.com");
    }

    #[test]
    fn test_client_builder_url_multiple_trailing_slashes() {
        let client = Client::builder("https://example.com///").build().unwrap();
        assert_eq!(client.base_url(), "https://example.com");
    }

    #[test]
    fn test_client_builder_url_path_preserved() {
        let client = Client::builder("https://example.com/gitea/")
            .build()
            .unwrap();
        assert_eq!(client.base_url(), "https://example.com/gitea");
    }

    #[test]
    fn test_client_debug_flag() {
        let client = Client::builder("https://example.com")
            .debug(true)
            .build()
            .unwrap();
        assert!(client.read_config().debug);

        let client = Client::builder("https://example.com")
            .debug(false)
            .build()
            .unwrap();
        assert!(!client.read_config().debug);
    }

    #[test]
    fn test_client_builder_default() {
        let client = Client::builder("https://example.com").build().unwrap();
        let cfg = client.read_config();
        assert!(cfg.access_token.is_empty());
        assert!(cfg.username.is_empty());
        assert!(cfg.password.is_empty());
        assert!(cfg.otp.is_empty());
        assert!(cfg.sudo.is_empty());
        assert!(cfg.user_agent.is_empty());
        assert!(!cfg.debug);
        assert!(!cfg.ignore_version);
    }

    #[test]
    fn test_client_gitea_version_invalid_string() {
        let err = Client::builder("https://example.com")
            .gitea_version("not-a-version")
            .build()
            .unwrap_err();
        match err {
            Error::Version(message) => assert!(message.contains("not-a-version")),
            other => panic!("expected Error::Version, got: {other}"),
        }
    }

    #[test]
    fn test_client_clone_shares_state() {
        let client = Client::builder("https://example.com")
            .token("shared-token")
            .build()
            .unwrap();
        let cloned = client.clone();

        client.set_token("updated-token");
        assert_eq!(cloned.read_config().access_token, "updated-token");
    }

    #[test]
    fn test_client_builder_ssh_cert() {
        let tmp = std::env::temp_dir().join("gitea_sdk_test_ssh_cert_builder");
        std::fs::write(
            &tmp,
            include_bytes!("../tests/ssh_fixtures/id_ed25519_test"),
        )
        .expect("write temp key");
        let client = Client::builder("https://example.com")
            .ssh_cert("test-principal", &tmp, None::<&str>)
            .expect("ssh_cert should succeed with valid key")
            .build()
            .expect("build with ssh_cert should succeed");
        let signer = client.ssh_signer();
        assert!(
            signer.is_some(),
            "ssh_signer should be present after ssh_cert()"
        );
        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn test_client_builder_ssh_pubkey() {
        let tmp = std::env::temp_dir().join("gitea_sdk_test_ssh_pubkey_builder");
        std::fs::write(
            &tmp,
            include_bytes!("../tests/ssh_fixtures/id_ed25519_test"),
        )
        .expect("write temp key");
        let fp = crate::auth::ssh_sign::fingerprint(
            ssh_key::PrivateKey::from_openssh(include_bytes!(
                "../tests/ssh_fixtures/id_ed25519_test"
            ))
            .expect("parse test key")
            .public_key(),
        );
        let client = Client::builder("https://example.com")
            .ssh_pubkey(&fp, &tmp, None::<&str>)
            .expect("ssh_pubkey should succeed with valid key")
            .build()
            .expect("build with ssh_pubkey should succeed");
        let signer = client.ssh_signer();
        assert!(
            signer.is_some(),
            "ssh_signer should be present after ssh_pubkey()"
        );
        let _ = std::fs::remove_file(&tmp);
    }

    #[test]
    fn test_client_builder_no_ssh() {
        let client = Client::builder("https://example.com").build().unwrap();
        let signer = client.ssh_signer();
        assert!(
            signer.is_none(),
            "ssh_signer should be None when no SSH configured"
        );
    }

    #[test]
    fn test_client_builder_ssh_cert_invalid_path() {
        let result = Client::builder("https://example.com").ssh_cert(
            "principal",
            "/nonexistent/key",
            None::<&str>,
        );
        assert!(
            result.is_err(),
            "ssh_cert with nonexistent path should return Err"
        );
    }

    #[test]
    fn test_client_builder_ssh_pubkey_invalid_path() {
        let result = Client::builder("https://example.com").ssh_pubkey(
            "SHA256:abc",
            "/nonexistent/key",
            None::<&str>,
        );
        assert!(
            result.is_err(),
            "ssh_pubkey with nonexistent path should return Err"
        );
    }
}
