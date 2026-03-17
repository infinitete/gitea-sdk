// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use std::sync::LazyLock;

use semver::Version;

use crate::Client;

// Using `LazyLock` because `semver::Version::parse` is not `const`.

macro_rules! version_const {
    ($name:ident, $major:expr, $minor:expr) => {
        #[allow(dead_code)]
        /// Lazily parsed [`semver::Version`] constant.
        pub(crate) static $name: LazyLock<Version> =
            LazyLock::new(|| Version::new($major, $minor, 0));
    };
}

version_const!(VERSION_1_11, 1, 11);
version_const!(VERSION_1_12, 1, 12);
version_const!(VERSION_1_13, 1, 13);
version_const!(VERSION_1_14, 1, 14);
version_const!(VERSION_1_15, 1, 15);
version_const!(VERSION_1_16, 1, 16);
version_const!(VERSION_1_17, 1, 17);
version_const!(VERSION_1_18, 1, 18);
version_const!(VERSION_1_19, 1, 19);
version_const!(VERSION_1_20, 1, 20);
version_const!(VERSION_1_21, 1, 21);
version_const!(VERSION_1_22, 1, 22);

// ── Server version response ───────────────────────────────────────────

/// Response body from `GET /api/v1/version`.
#[derive(Debug, serde::Deserialize)]
struct ServerVersionResponse {
    version: String,
}

// ── Client impl (version methods) ─────────────────────────────────────

impl Client {
    /// Load the server version lazily via [`OnceLock`], falling back to
    /// `1.11.0` on parse failure.
    ///
    /// Resolution order:
    /// 1. If `ignore_version` is set → returns [`Error::Version`].
    /// 2. If a preset version was configured via [`ClientBuilder::gitea_version`]
    ///    → uses that without making an HTTP request.
    /// 3. If the [`OnceLock`] is already initialised → returns the cached value.
    /// 4. Otherwise → `GET /version`, parse the response, and cache it.
    ///
    /// On parse failure the lock is set to `1.11.0` (safety net) and
    /// [`Error::UnknownVersion`] is returned.
    pub(crate) async fn load_server_version(&self) -> crate::Result<Version> {
        // 1. Version checks disabled.
        if self.ignore_version() {
            return Err(crate::Error::Version("version checks disabled".into()));
        }

        // 2. Pre-set version from builder.
        if let Some(v) = self.preset_version() {
            return Ok(v.clone());
        }

        // 3. Already cached via OnceLock.
        if let Some(v) = self.server_version_lock().get() {
            return Ok(v.clone());
        }

        // 4. Fetch from server.
        let (data, _resp) = self
            .get_response(reqwest::Method::GET, "/version", None, None::<String>)
            .await?;

        let svr: ServerVersionResponse = serde_json::from_slice(&data)?;
        let ver_str = svr.version.trim().trim_start_matches('v');

        match Version::parse(ver_str) {
            Ok(v) => {
                let _ = self.server_version_lock().set(v.clone());
                Ok(v)
            }
            Err(_) => {
                // Fallback to 1.11.0 on parse failure.
                let fallback = VERSION_1_11.clone();
                let _ = self.server_version_lock().set(fallback.clone());
                Err(crate::Error::UnknownVersion(ver_str.to_string()))
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
        assert_eq!(VERSION_1_11.major, 1);
        assert_eq!(VERSION_1_11.minor, 11);
        assert_eq!(VERSION_1_11.patch, 0);
        assert_eq!(VERSION_1_22.major, 1);
        assert_eq!(VERSION_1_22.minor, 22);
        assert_eq!(VERSION_1_12.minor, 12);
        assert_eq!(VERSION_1_13.minor, 13);
        assert_eq!(VERSION_1_14.minor, 14);
        assert_eq!(VERSION_1_15.minor, 15);
        assert_eq!(VERSION_1_16.minor, 16);
        assert_eq!(VERSION_1_17.minor, 17);
        assert_eq!(VERSION_1_18.minor, 18);
        assert_eq!(VERSION_1_19.minor, 19);
        assert_eq!(VERSION_1_20.minor, 20);
        assert_eq!(VERSION_1_21.minor, 21);
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
        let a = VERSION_1_11.clone();
        let b = VERSION_1_11.clone();
        assert_eq!(a, b);
        assert_eq!(a.to_string(), "1.11.0");
    }

    #[test]
    fn test_version_constants_all_twelve() {
        // Ensure all 12 constants are distinct and ordered.
        let versions: Vec<&Version> = vec![
            &VERSION_1_11,
            &VERSION_1_12,
            &VERSION_1_13,
            &VERSION_1_14,
            &VERSION_1_15,
            &VERSION_1_16,
            &VERSION_1_17,
            &VERSION_1_18,
            &VERSION_1_19,
            &VERSION_1_20,
            &VERSION_1_21,
            &VERSION_1_22,
        ];
        assert_eq!(versions.len(), 12);
        for window in versions.windows(2) {
            assert!(window[0] < window[1]);
        }
    }
}
