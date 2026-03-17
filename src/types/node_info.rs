// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use std::collections::HashMap;

/// GitignoreTemplateInfo represents a gitignore template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitignoreTemplateInfo {
    pub name: String,
    pub source: String,
}

/// LabelTemplate represents a label template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabelTemplate {
    pub name: String,
    pub color: String,
    pub description: String,
    pub exclusive: bool,
}

/// NodeInfoSoftware represents software information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoSoftware {
    pub name: String,
    pub version: String,
    pub repository: String,
    pub homepage: String,
}

/// NodeInfoServices represents third party services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoServices {
    #[serde(default)]
    pub inbound: Vec<String>,
    #[serde(default)]
    pub outbound: Vec<String>,
}

/// NodeInfoUsageUsers represents user statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoUsageUsers {
    pub total: i64,
    #[serde(rename = "activeHalfyear")]
    pub active_halfyear: i64,
    #[serde(rename = "activeMonth")]
    pub active_month: i64,
}

/// NodeInfoUsage represents usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfoUsage {
    pub users: NodeInfoUsageUsers,
    #[serde(rename = "localPosts")]
    pub local_posts: i64,
    #[serde(rename = "localComments")]
    pub local_comments: i64,
}

/// NodeInfo represents nodeinfo about the server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    pub version: String,
    pub software: NodeInfoSoftware,
    #[serde(default)]
    pub protocols: Vec<String>,
    pub services: NodeInfoServices,
    #[serde(rename = "openRegistrations")]
    pub open_registrations: bool,
    pub usage: NodeInfoUsage,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gitignore_template_info_round_trip() {
        let original = GitignoreTemplateInfo {
            name: "Rust".to_string(),
            source: "/target/\n**/*.rs.bk\n".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: GitignoreTemplateInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.source, original.source);
    }

    #[test]
    fn test_label_template_round_trip() {
        let original = LabelTemplate {
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: "Something is broken".to_string(),
            exclusive: false,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: LabelTemplate = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.color, original.color);
    }

    #[test]
    fn test_node_info_round_trip() {
        let original = NodeInfo {
            version: "2.1".to_string(),
            software: NodeInfoSoftware {
                name: "gitea".to_string(),
                version: "1.20.0".to_string(),
                repository: "https://gitea.com/gitea/gitea".to_string(),
                homepage: "https://gitea.io".to_string(),
            },
            protocols: vec!["activitypub".to_string()],
            services: NodeInfoServices {
                inbound: vec![],
                outbound: vec![],
            },
            open_registrations: true,
            usage: NodeInfoUsage {
                users: NodeInfoUsageUsers {
                    total: 100,
                    active_halfyear: 50,
                    active_month: 20,
                },
                local_posts: 500,
                local_comments: 1000,
            },
            metadata: HashMap::new(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: NodeInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.version, original.version);
        assert_eq!(restored.software.name, "gitea");
        assert_eq!(restored.usage.users.total, 100);
    }
}
