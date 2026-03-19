// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::types::enums::GitServiceType;
use crate::{Deserialize, Serialize};

// ── repo_migrate.go ─────────────────────────────────────────────

/// MigrateRepoOption options for migrating a repository from an external service
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Migrate Repo Option.
pub struct MigrateRepoOption {
    #[serde(rename = "repo_name")]
    pub repo_name: String,
    #[serde(rename = "repo_owner")]
    pub repo_owner: String,
    /// deprecated use RepoOwner
    pub uid: i64,
    #[serde(rename = "clone_addr")]
    pub clone_addr: String,
    pub service: GitServiceType,
    #[serde(rename = "auth_username")]
    pub auth_username: String,
    #[serde(rename = "auth_password")]
    pub auth_password: String,
    #[serde(rename = "auth_token")]
    pub auth_token: String,
    pub mirror: bool,
    pub private: bool,
    pub description: String,
    pub wiki: bool,
    pub milestones: bool,
    pub labels: bool,
    pub issues: bool,
    #[serde(rename = "pull_requests")]
    pub pull_requests: bool,
    pub releases: bool,
    #[serde(rename = "mirror_interval")]
    pub mirror_interval: String,
    pub lfs: bool,
    #[serde(rename = "lfs_endpoint")]
    pub lfs_endpoint: String,
}

impl MigrateRepoOption {
    /// Validate this `MigrateRepoOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.clone_addr.is_empty() {
            return Err(crate::Error::Validation("clone addr required".to_string()));
        }
        if self.repo_name.is_empty() {
            return Err(crate::Error::Validation("repo name required".to_string()));
        } else if self.repo_name.len() > 100 {
            return Err(crate::Error::Validation("repo name too long".to_string()));
        }
        if self.description.len() > 2048 {
            return Err(crate::Error::Validation("description too long".to_string()));
        }
        match self.service {
            GitServiceType::Github => {
                if self.auth_token.is_empty() {
                    return Err(crate::Error::Validation(
                        "github requires token authentication".to_string(),
                    ));
                }
            }
            GitServiceType::Gitlab | GitServiceType::Gitea => {
                if self.auth_token.is_empty() {
                    return Err(crate::Error::Validation(format!(
                        "{} requires token authentication",
                        self.service
                    )));
                }
            }
            GitServiceType::Gogs => {
                if self.auth_token.is_empty() {
                    return Err(crate::Error::Validation(
                        "gogs requires token authentication".to_string(),
                    ));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

// ── repo_transfer.go ────────────────────────────────────────────

/// TransferRepoOption options when transfer a repository's ownership
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Transfer Repo Option.
pub struct TransferRepoOption {
    #[serde(rename = "new_owner")]
    pub new_owner: String,
    #[serde(rename = "team_ids")]
    pub team_ids: Option<Vec<i64>>,
}

// ── repo_template.go ────────────────────────────────────────────

/// CreateRepoFromTemplateOption options when creating repository using a template
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Create Repo From Template Option.
pub struct CreateRepoFromTemplateOption {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub private: bool,
    #[serde(rename = "git_content")]
    pub git_content: bool,
    pub topics: bool,
    #[serde(rename = "git_hooks")]
    pub git_hooks: bool,
    pub webhooks: bool,
    pub avatar: bool,
    pub labels: bool,
}

impl CreateRepoFromTemplateOption {
    /// Validate this `CreateRepoFromTemplateOption` payload.
    pub fn validate(&self) -> crate::Result<()> {
        if self.owner.is_empty() {
            return Err(crate::Error::Validation(
                "field Owner is required".to_string(),
            ));
        }
        if self.name.is_empty() {
            return Err(crate::Error::Validation(
                "field Name is required".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_repo_option_validate_success_git() {
        let opt = MigrateRepoOption {
            clone_addr: "https://example.com/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_migrate_repo_option_validate_empty_clone_addr() {
        let opt = MigrateRepoOption {
            clone_addr: String::new(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_empty_repo_name() {
        let opt = MigrateRepoOption {
            clone_addr: "https://example.com/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: String::new(),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_name_too_long() {
        let opt = MigrateRepoOption {
            clone_addr: "https://example.com/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "a".repeat(101),
            service: GitServiceType::Git,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_github_no_token() {
        let opt = MigrateRepoOption {
            clone_addr: "https://github.com/user/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Github,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: String::new(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_migrate_repo_option_validate_github_with_token() {
        let opt = MigrateRepoOption {
            clone_addr: "https://github.com/user/repo.git".to_string(),
            repo_owner: "myuser".to_string(),
            repo_name: "my-repo".to_string(),
            service: GitServiceType::Github,
            uid: 0,
            auth_username: String::new(),
            auth_password: String::new(),
            auth_token: "token123".to_string(),
            mirror: false,
            private: false,
            description: String::new(),
            wiki: false,
            milestones: false,
            labels: false,
            issues: false,
            pull_requests: false,
            releases: false,
            mirror_interval: String::new(),
            lfs: false,
            lfs_endpoint: String::new(),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_repo_from_template_option_validate_success() {
        let opt = CreateRepoFromTemplateOption {
            owner: "myorg".to_string(),
            name: "my-repo".to_string(),
            description: String::new(),
            private: false,
            git_content: false,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_create_repo_from_template_option_validate_empty_owner() {
        let opt = CreateRepoFromTemplateOption {
            owner: String::new(),
            name: "my-repo".to_string(),
            description: String::new(),
            private: false,
            git_content: false,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        assert!(opt.validate().is_err());
    }

    #[test]
    fn test_create_repo_from_template_option_validate_empty_name() {
        let opt = CreateRepoFromTemplateOption {
            owner: "myorg".to_string(),
            name: String::new(),
            description: String::new(),
            private: false,
            git_content: false,
            topics: false,
            git_hooks: false,
            webhooks: false,
            avatar: false,
            labels: false,
        };
        assert!(opt.validate().is_err());
    }
}
