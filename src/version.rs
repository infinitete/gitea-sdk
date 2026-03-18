// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::sync::LazyLock;

use semver::Version;

use crate::Client;
use crate::client::CachedServerVersion;

// Using `LazyLock` because `semver::Version::parse` is not `const`.

macro_rules! version_const {
    ($name:ident, $major:expr, $minor:expr, $patch:expr) => {
        #[allow(dead_code)]
        /// Lazily parsed [`semver::Version`] constant.
        pub(crate) static $name: LazyLock<Version> =
            LazyLock::new(|| Version::new($major, $minor, $patch));
    };
}

version_const!(VERSION_1_11_0, 1, 11, 0);
version_const!(VERSION_1_11_5, 1, 11, 5);
version_const!(VERSION_1_12_0, 1, 12, 0);
version_const!(VERSION_1_12_3, 1, 12, 3);
version_const!(VERSION_1_13_0, 1, 13, 0);
version_const!(VERSION_1_14_0, 1, 14, 0);
version_const!(VERSION_1_15_0, 1, 15, 0);
version_const!(VERSION_1_16_0, 1, 16, 0);
version_const!(VERSION_1_17_0, 1, 17, 0);
version_const!(VERSION_1_22_0, 1, 22, 0);
version_const!(VERSION_1_23_0, 1, 23, 0);
version_const!(VERSION_1_25_0, 1, 25, 0);

// ── Server version response ───────────────────────────────────────────

/// Response body from `GET /api/v1/version`.
#[derive(Debug, serde::Deserialize)]
struct ServerVersionResponse {
    version: String,
}

// ── Client impl (version methods) ─────────────────────────────────────

impl Client {
    /// Load the server version lazily via [`OnceLock`].
    ///
    /// Resolution order:
    /// 1. If `ignore_version` is set → returns [`Error::Version`].
    /// 2. If a preset version was configured via [`ClientBuilder::gitea_version`]
    ///    → uses that without making an HTTP request.
    /// 3. If the [`OnceLock`] is already initialised → returns the cached value.
    /// 4. Otherwise → `GET /version`, parse the response, and cache either the
    ///    parsed version or the unknown-version failure.
    ///
    /// On parse failure, [`Error::UnknownVersion`] is returned and that failure
    /// is cached to avoid repeated network round-trips.
    pub(crate) async fn load_server_version(&self) -> crate::Result<Version> {
        if self.ignore_version() {
            return Err(crate::Error::Version("version checks disabled".into()));
        }

        if let Some(v) = self.preset_version() {
            return Ok(v.clone());
        }

        if let Some(v) = self.server_version_lock().get() {
            return match v {
                CachedServerVersion::Parsed(parsed) => Ok(parsed.clone()),
                CachedServerVersion::Unknown(raw) => Err(crate::Error::UnknownVersion(raw.clone())),
            };
        }

        let _guard = self.version_loading_lock().await;
        if let Some(v) = self.server_version_lock().get() {
            return match v {
                CachedServerVersion::Parsed(parsed) => Ok(parsed.clone()),
                CachedServerVersion::Unknown(raw) => Err(crate::Error::UnknownVersion(raw.clone())),
            };
        }

        let (data, _resp) = self
            .get_response(reqwest::Method::GET, "/version", None, None::<String>)
            .await?;

        let svr: ServerVersionResponse = serde_json::from_slice(&data)?;
        let ver_str = svr.version.trim().trim_start_matches('v');

        match Version::parse(ver_str) {
            Ok(v) => {
                let _ = self
                    .server_version_lock()
                    .set(CachedServerVersion::Parsed(v.clone()));
                Ok(v)
            }
            Err(_) => {
                let raw = ver_str.to_string();
                let _ = self
                    .server_version_lock()
                    .set(CachedServerVersion::Unknown(raw.clone()));
                Err(crate::Error::UnknownVersion(raw))
            }
        }
    }

    /// Return the server version string.
    ///
    /// Equivalent to Go SDK `ServerVersion()`.
    pub async fn server_version(&self) -> crate::Result<String> {
        let v = self.load_server_version().await?;
        Ok(v.to_string())
    }

    /// Trigger lazy version loading and cache the result.
    ///
    /// This lets callers verify server compatibility before issuing other
    /// API requests.
    pub async fn check_version(&self) -> crate::Result<()> {
        let _ = self.load_server_version().await?;
        Ok(())
    }

    /// Check a semver version constraint against the server version.
    ///
    /// Returns `Ok(())` if the constraint is satisfied, or
    /// [`crate::Error::Version`] if it is not.
    ///
    /// Equivalent to Go SDK `CheckServerVersionConstraint`.
    pub async fn check_server_version_constraint(&self, constraint: &str) -> crate::Result<()> {
        let server_ver = self.load_server_version().await?;
        let req = semver::VersionReq::parse(constraint).map_err(|e| {
            crate::Error::Version(format!("invalid constraint '{constraint}': {e}"))
        })?;
        if req.matches(&server_ver) {
            Ok(())
        } else {
            Err(crate::Error::Version(format!(
                "server version {server_ver} does not satisfy constraint '{constraint}'"
            )))
        }
    }

    /// Check that the server version is >= the given version.
    ///
    /// When `ignore_version` is enabled this always returns `Ok(())`.
    ///
    /// Equivalent to Go SDK `checkServerVersionGreaterThanOrEqual`.
    #[allow(dead_code)]
    pub(crate) async fn check_server_version_ge(&self, v: &Version) -> crate::Result<()> {
        if self.ignore_version() {
            return Ok(());
        }
        let server_ver = self.load_server_version().await?;
        if server_ver >= *v {
            Ok(())
        } else {
            Err(crate::Error::Version(format!(
                "server version {server_ver} is older than required {v}"
            )))
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_constants() {
        assert_eq!(VERSION_1_11_0.major, 1);
        assert_eq!(VERSION_1_11_0.minor, 11);
        assert_eq!(VERSION_1_11_0.patch, 0);
        assert_eq!(VERSION_1_22_0.major, 1);
        assert_eq!(VERSION_1_22_0.minor, 22);
        assert_eq!(VERSION_1_12_0.minor, 12);
        assert_eq!(VERSION_1_12_0.patch, 0);
        assert_eq!(VERSION_1_12_3.patch, 3);
        assert_eq!(VERSION_1_13_0.minor, 13);
        assert_eq!(VERSION_1_14_0.minor, 14);
        assert_eq!(VERSION_1_15_0.minor, 15);
        assert_eq!(VERSION_1_16_0.minor, 16);
        assert_eq!(VERSION_1_17_0.minor, 17);
        assert_eq!(VERSION_1_22_0.patch, 0);
        assert_eq!(VERSION_1_23_0.minor, 23);
        assert_eq!(VERSION_1_25_0.minor, 25);
    }

    #[test]
    fn test_version_constraint_passes() {
        let v: Version = "1.22.0".parse().unwrap();
        let req = semver::VersionReq::parse(">=1.11.0").unwrap();
        assert!(req.matches(&v));
    }

    #[test]
    fn test_version_constraint_fails() {
        let v: Version = "1.19.0".parse().unwrap();
        let req = semver::VersionReq::parse(">=1.20.0").unwrap();
        assert!(!req.matches(&v));
    }

    #[test]
    fn test_version_parse_with_v_prefix() {
        let v: Version = "v1.22.0".trim_start_matches('v').parse().unwrap();
        assert_eq!(v.to_string(), "1.22.0");
    }

    #[test]
    fn test_version_constants_lazy_parse() {
        // Verify LazyLock produces consistent, clonable values.
        let a = VERSION_1_11_0.clone();
        let b = VERSION_1_11_0.clone();
        assert_eq!(a, b);
        assert_eq!(a.to_string(), "1.11.0");
    }

    #[test]
    fn test_version_constants_all_twelve() {
        // Ensure all 12 constants are distinct and ordered.
        let versions: Vec<&Version> = vec![
            &VERSION_1_11_0,
            &VERSION_1_11_5,
            &VERSION_1_12_0,
            &VERSION_1_12_3,
            &VERSION_1_13_0,
            &VERSION_1_14_0,
            &VERSION_1_15_0,
            &VERSION_1_16_0,
            &VERSION_1_17_0,
            &VERSION_1_22_0,
            &VERSION_1_23_0,
            &VERSION_1_25_0,
        ];
        assert_eq!(versions.len(), 12);
        for window in versions.windows(2) {
            assert!(window[0] < window[1]);
        }
    }
}
