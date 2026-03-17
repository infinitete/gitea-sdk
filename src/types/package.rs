// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::user::User;

/// Package represents a package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// the package's id
    pub id: i64,
    /// the package's owner
    pub owner: User,
    /// the repo this package belongs to (if any)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<serde_json::Value>,
    /// the package's creator
    pub creator: User,
    /// the type of package
    #[serde(rename = "type")]
    pub package_type: String,
    /// the name of the package
    pub name: String,
    /// the version of the package
    pub version: String,
    /// the date the package was uploaded
    #[serde(rename = "created_at", with = "rfc3339")]
    pub created_at: OffsetDateTime,
}

/// PackageFile represents a file from a package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageFile {
    /// the file's ID
    pub id: i64,
    /// the size of the file in bytes
    pub size: i64,
    /// the name of the file
    pub name: String,
    /// the md5 hash of the file
    #[serde(rename = "md5")]
    pub md5: String,
    /// the sha1 hash of the file
    #[serde(rename = "sha1")]
    pub sha1: String,
    /// the sha256 hash of the file
    #[serde(rename = "sha256")]
    pub sha256: String,
    /// the sha512 hash of the file
    #[serde(rename = "sha512")]
    pub sha512: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_package_file_round_trip() {
        let original = PackageFile {
            id: 1,
            size: 1024,
            name: "package.tar.gz".to_string(),
            md5: "abc".to_string(),
            sha1: "def".to_string(),
            sha256: "ghi".to_string(),
            sha512: "jkl".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: PackageFile = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.sha256, original.sha256);
    }

    #[test]
    fn test_package_deserialize() {
        let json = r#"{
            "id": 1,
            "owner": {"id": 2, "login": "testuser", "login_name": "", "source_id": 0, "full_name": "", "email": "", "avatar_url": "", "html_url": "", "language": "", "is_admin": false, "last_login": "0001-01-01T00:00:00Z", "created": "0001-01-01T00:00:00Z", "restricted": false, "active": true, "prohibit_login": false, "location": "", "website": "", "description": "", "visibility": "public", "followers_count": 0, "following_count": 0, "starred_repos_count": 0},
            "repository": null,
            "creator": {"id": 2, "login": "testuser", "login_name": "", "source_id": 0, "full_name": "", "email": "", "avatar_url": "", "html_url": "", "language": "", "is_admin": false, "last_login": "0001-01-01T00:00:00Z", "created": "0001-01-01T00:00:00Z", "restricted": false, "active": true, "prohibit_login": false, "location": "", "website": "", "description": "", "visibility": "public", "followers_count": 0, "following_count": 0, "starred_repos_count": 0},
            "type": "debian",
            "name": "my-package",
            "version": "1.0.0",
            "created_at": "2024-01-15T10:00:00Z"
        }"#;
        let package: Package = serde_json::from_str(json).unwrap();
        assert_eq!(package.id, 1);
        assert_eq!(package.package_type, "debian");
        assert_eq!(package.name, "my-package");
        assert!(package.repository.is_none());
    }
}
