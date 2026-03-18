// Copyright 2026 The Gitea Authors. All rights reserved.
// Use of this source code is governed by a MIT-style
// license that can be found in the LICENSE file.

//! Custom serde helpers for handling Gitea API quirks.
//!
//! Gitea's Go backend uses `time.Time` whose zero value serializes as
//! `"0001-01-01T00:00:00Z"`. In Rust we map this to `None` for
//! `Option<OffsetDateTime>` fields.

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serializer};
use std::fmt;
use time::OffsetDateTime;

/// Checks whether the given `OffsetDateTime` is Go's zero time
/// (`0001-01-01T00:00:00Z`).
fn is_zero(dt: OffsetDateTime) -> bool {
    dt.year() == 1
        && dt.month() == time::Month::January
        && dt.day() == 1
        && dt.hour() == 0
        && dt.minute() == 0
        && dt.second() == 0
        && dt.nanosecond() == 0
        && dt.offset() == time::UtcOffset::UTC
}

/// Serde module for `Option<OffsetDateTime>` that treats Go's zero time as `None`.
///
/// # Serialization
/// - `Some(dt)` → RFC 3339 string (e.g. `"2024-01-15T10:30:00Z"`)
/// - `None` → `null`
///
/// # Deserialization
/// - `"0001-01-01T00:00:00Z"` → `None`
/// - `null` → `None`
/// - Valid RFC 3339 timestamp → `Some(dt)`
///
/// # Usage
///
/// ```no_run
/// use gitea_sdk::types::serde_helpers::nullable_rfc3339;
/// use serde::{Deserialize, Serialize};
/// use time::OffsetDateTime;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyEntity {
///     #[serde(with = "nullable_rfc3339", skip_serializing_if = "Option::is_none")]
///     created_at: Option<OffsetDateTime>,
/// }
///
/// let _entity = MyEntity {
///     created_at: Some(OffsetDateTime::UNIX_EPOCH),
/// };
/// ```
pub mod nullable_rfc3339 {
    use super::*;

    struct NullableRfc3339Visitor;

    impl<'de> Visitor<'de> for NullableRfc3339Visitor {
        type Value = Option<OffsetDateTime>;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "an RFC 3339 datetime string or null")
        }

        fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
            Ok(None)
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
            let dt = OffsetDateTime::parse(v, &time::format_description::well_known::Rfc3339)
                .map_err(de::Error::custom)?;
            Ok(if is_zero(dt) { None } else { Some(dt) })
        }
    }

    /// Deserialize an optional RFC 3339 timestamp for serde.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(NullableRfc3339Visitor)
    }

    /// Serialize an optional RFC 3339 timestamp for serde.
    pub fn serialize<S>(opt: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(dt) => {
                let formatted = dt
                    .format(&time::format_description::well_known::Rfc3339)
                    .map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&formatted)
            }
            None => serializer.serialize_none(),
        }
    }
}

/// Deserialize `null` as `T::default()`.
pub fn null_to_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(deserializer)?.unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Deserialize;
    use serde::Serialize;
    use serde_json;

    fn dt(s: &str) -> OffsetDateTime {
        OffsetDateTime::parse(s, &time::format_description::well_known::Rfc3339).unwrap()
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Fixture {
        #[serde(with = "nullable_rfc3339", skip_serializing_if = "Option::is_none")]
        created_at: Option<OffsetDateTime>,
    }

    #[test]
    fn deserialize_zero_time_returns_none() {
        let json = r#"{"created_at":"0001-01-01T00:00:00Z"}"#;
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        assert!(fixture.created_at.is_none());
    }

    #[test]
    fn deserialize_null_returns_none() {
        let json = r#"{"created_at":null}"#;
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        assert!(fixture.created_at.is_none());
    }

    #[test]
    fn deserialize_normal_timestamp_returns_some() {
        let json = r#"{"created_at":"2024-01-15T10:30:00Z"}"#;
        let fixture: Fixture = serde_json::from_str(json).unwrap();
        assert_eq!(fixture.created_at, Some(dt("2024-01-15T10:30:00Z")));
    }

    #[test]
    fn serialize_some_outputs_rfc3339() {
        let fixture = Fixture {
            created_at: Some(dt("2024-01-15T10:30:00Z")),
        };
        let json = serde_json::to_value(&fixture).unwrap();
        assert_eq!(json["created_at"], "2024-01-15T10:30:00Z");
    }

    #[test]
    fn serialize_none_outputs_null() {
        let fixture = Fixture { created_at: None };
        let json = serde_json::to_value(&fixture).unwrap();
        assert!(json.get("created_at").is_none());
    }

    #[test]
    fn serialize_none_without_skip_serializing_if() {
        #[derive(Serialize)]
        struct NoSkip {
            #[serde(with = "nullable_rfc3339")]
            created_at: Option<OffsetDateTime>,
        }
        let fixture = NoSkip { created_at: None };
        let json = serde_json::to_value(&fixture).unwrap();
        assert!(json["created_at"].is_null());
    }

    #[test]
    fn round_trip_normal_timestamp() {
        let original = Fixture {
            created_at: Some(dt("2024-06-10T08:00:00Z")),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Fixture = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn round_trip_zero_time_becomes_none() {
        let original = Fixture {
            created_at: Some(OffsetDateTime::new_utc(
                time::Date::from_calendar_date(1, time::Month::January, 1).unwrap(),
                time::Time::from_hms(0, 0, 0).unwrap(),
            )),
        };
        let json = serde_json::to_string(&original).unwrap();
        let restored: Fixture = serde_json::from_str(&json).unwrap();
        assert!(restored.created_at.is_none());
    }
}
