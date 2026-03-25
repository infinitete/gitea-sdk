// Copyright 2026 infinitete. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Types for cron tasks.

use crate::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;

/// `CronTask` represents a Cron task
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Cron Task payload type.
pub struct CronTask {
    pub name: String,
    pub schedule: String,
    #[serde(with = "rfc3339")]
    pub next: OffsetDateTime,
    #[serde(with = "rfc3339")]
    pub prev: OffsetDateTime,
    #[serde(rename = "exec_times")]
    pub exec_times: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cron_task_round_trip() {
        let original = CronTask {
            name: "cleanup".to_string(),
            schedule: "@daily".to_string(),
            next: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::February, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            prev: OffsetDateTime::new_utc(
                time::Date::from_calendar_date(2024, time::Month::January, 31).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            ),
            exec_times: 10,
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: CronTask = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.name, original.name);
        assert_eq!(restored.schedule, original.schedule);
        assert_eq!(restored.exec_times, original.exec_times);
    }
}
