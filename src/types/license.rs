// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for license templates.

use crate::{Deserialize, Serialize};

/// LicensesTemplateListEntry represents a license template in the list
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Licenses Template List Entry payload type.
pub struct LicensesTemplateListEntry {
    pub key: String,
    pub name: String,
    pub url: String,
}

/// LicenseTemplateInfo represents the full content for a license template
#[derive(Debug, Clone, Serialize, Deserialize)]
/// License Template Info payload type.
pub struct LicenseTemplateInfo {
    pub key: String,
    pub name: String,
    pub url: String,
    pub body: String,
    pub implementation: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_licenses_template_list_entry_round_trip() {
        let original = LicensesTemplateListEntry {
            key: "mit".to_string(),
            name: "MIT".to_string(),
            url: "https://example.com/licenses/mit".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: LicensesTemplateListEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.key, original.key);
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.url, original.url);
    }

    #[test]
    fn test_license_template_info_round_trip() {
        let original = LicenseTemplateInfo {
            key: "apache-2.0".to_string(),
            name: "Apache License 2.0".to_string(),
            url: "https://example.com/licenses/apache-2.0".to_string(),
            body: "Apache License text".to_string(),
            implementation: "Hashicorp".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: LicenseTemplateInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.key, original.key);
        assert_eq!(restored.body, original.body);
        assert_eq!(restored.implementation, original.implementation);
    }
}
