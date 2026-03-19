// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::pagination::{ListOptions, QueryEncode};
use crate::types::enums::AccessMode;
use crate::{Deserialize, Serialize};

// ── repo_collaborator.go ────────────────────────────────────────

/// ListCollaboratorsOptions options for listing a repository's collaborators
#[derive(Debug, Clone, Default)]
/// Options for List Collaborators Option.
pub struct ListCollaboratorsOptions {
    pub list_options: ListOptions,
}

impl QueryEncode for ListCollaboratorsOptions {
    fn query_encode(&self) -> String {
        self.list_options.query_encode()
    }
}

/// AddCollaboratorOption options when adding a user as a collaborator
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Options for Add Collaborator Option.
pub struct AddCollaboratorOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<AccessMode>,
}

impl AddCollaboratorOption {
    /// Validate this `AddCollaboratorOption` payload.
    pub fn validate(&mut self) -> crate::Result<()> {
        if let Some(ref perm) = self.permission {
            match perm {
                AccessMode::Owner => {
                    self.permission = Some(AccessMode::Admin);
                    return Ok(());
                }
                AccessMode::None => {
                    self.permission = None;
                    return Ok(());
                }
                AccessMode::Read | AccessMode::Write | AccessMode::Admin => {}
                _ => {
                    return Err(crate::Error::Validation(
                        "permission mode invalid".to_string(),
                    ));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_collaborator_option_validate_read() {
        let mut opt = AddCollaboratorOption {
            permission: Some(AccessMode::Read),
        };
        assert!(opt.validate().is_ok());
    }

    #[test]
    fn test_add_collaborator_option_validate_none() {
        let mut opt = AddCollaboratorOption { permission: None };
        assert!(opt.validate().is_ok());
    }
}
