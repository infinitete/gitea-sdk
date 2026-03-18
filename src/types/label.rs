// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};

/// Label represents a label for an issue or a pull request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Label payload type.
pub struct Label {
    pub id: i64,
    pub name: String,
    /// example: 00aabb
    pub color: String,
    pub description: String,
    pub exclusive: bool,
    #[serde(rename = "is_archived")]
    pub is_archived: bool,
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_round_trip() {
        let original = Label {
            id: 1,
            name: "bug".to_string(),
            color: "ff0000".to_string(),
            description: "Something is broken".to_string(),
            exclusive: false,
            is_archived: false,
            url: "https://gitea.example.com/api/v1/repos/test/labels/1".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Label = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.color, original.color);
    }
}
