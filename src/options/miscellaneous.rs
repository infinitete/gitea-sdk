// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownOption {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "Wiki", skip_serializing_if = "is_false")]
    pub wiki: bool,
}

fn is_false(b: &bool) -> bool {
    !b
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkupOption {
    #[serde(rename = "Text")]
    pub text: String,
    #[serde(rename = "Mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "Context", skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "FilePath", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "Wiki", skip_serializing_if = "is_false")]
    pub wiki: bool,
}
