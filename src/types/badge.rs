// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for user badges.

use crate::{Deserialize, Serialize};

/// Badge represents a user badge
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Badge payload type.
pub struct Badge {
    pub id: i64,
    pub slug: String,
    pub description: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_badge_round_trip() {
        let original = Badge {
            id: 1,
            slug: "contributor".to_string(),
            description: "Contributed to 10 repos".to_string(),
            image_url: "https://example.com/badges/contributor.png".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Badge = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.id, original.id);
        assert_eq!(restored.slug, original.slug);
        assert_eq!(restored.image_url, original.image_url);
    }
}
