// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};

/// Badge represents a user badge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Badge {
    pub id: i64,
    pub slug: String,
    pub description: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
}
