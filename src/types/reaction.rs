// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

use super::user::User;

/// Reaction contains one reaction
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Reaction payload type.
pub struct Reaction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub reaction: String,
    #[serde(with = "rfc3339")]
    pub created: OffsetDateTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reaction_round_trip() {
        let original = Reaction {
            user: None,
            reaction: ":+1:".to_string(),
            created: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 15).unwrap(),
                time::Time::from_hms(10, 0, 0).unwrap(),
            ),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Reaction = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.reaction, original.reaction);
    }
}
