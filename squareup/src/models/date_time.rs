//! Model struct for DateTime type.

use std::fmt::{Debug, Display};

use chrono::Utc;
use log::error;
use serde::{Deserialize, Serialize};

use super::errors::SquareApiError;

/// Represents a Timestamp or DateTime.
///
/// Handles Serialization and Deserialization and conversion between Square standard
/// RFC3339 String and Unix Timestamp i64.
///
/// Uses chrono library under the hood.
#[derive(Clone, Eq, PartialEq)]
pub struct DateTime {
    inner: chrono::DateTime<Utc>,
}

impl DateTime {
    /// Generates a new `DateTime` representing the present time.
    ///
    /// Alias for `DateTime::now()`
    pub fn new() -> Self {
        Self::now()
    }

    /// Generates a new `DateTime` representing the present time.
    ///
    /// Alias for `DateTime::new()`
    pub fn now() -> Self {
        Self {
            inner: chrono::offset::Utc::now(),
        }
    }
}

impl Debug for DateTime {
    /// Unwraps the inner DateTime for Debug.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

impl Default for DateTime {
    /// Default `DateTime` is the present/current time.
    ///
    /// Alias for `DateTime::now()`
    fn default() -> Self {
        Self::now()
    }
}

impl<'de> Deserialize<'de> for DateTime {
    /// Attempts to generate a `DateTime` from its serialized version.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Self {
            inner: chrono::DateTime::deserialize(deserializer)?,
        })
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
    }
}

impl From<DateTime> for chrono::DateTime<Utc> {
    fn from(val: DateTime) -> Self {
        val.inner
    }
}

impl From<DateTime> for chrono::NaiveDateTime {
    fn from(val: DateTime) -> Self {
        val.inner.naive_utc()
    }
}

impl From<&chrono::DateTime<Utc>> for DateTime {
    /// Converts a `chrono::DateTime<Utc>` reference into a `DateTime`.
    fn from(dt: &chrono::DateTime<Utc>) -> Self {
        Self {
            inner: dt.to_owned(),
        }
    }
}

impl TryFrom<i64> for DateTime {
    type Error = SquareApiError;

    /// Attempts to convert a Unix timestamp into a `DateTime`.
    ///
    /// Returns an API Error if the input i64 cannot be parsed.
    fn try_from(timestamp: i64) -> Result<Self, Self::Error> {
        let inner = chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0);
        match inner {
            Some(datetime) => Ok(Self::from(
                &chrono::DateTime::<Utc>::from_naive_utc_and_offset(datetime, Utc),
            )),
            None => {
                let msg = format!(
                    "Error parsing UTC timestamp to NaiveDateTime: {}",
                    timestamp
                );
                error!("{}", &msg);
                Err(Self::Error::new(&msg))
            }
        }
    }
}

impl Serialize for DateTime {
    /// Unwraps the inner DateTime for Serialization.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl TryFrom<&str> for DateTime {
    type Error = SquareApiError;

    /// Attempts to generate a `DateTime` from RFC3339 formatted String slice.
    ///
    /// Returns an API Error if the input String slice cannot be parsed.
    fn try_from(rfc3339: &str) -> Result<Self, Self::Error> {
        let inner = chrono::DateTime::parse_from_rfc3339(rfc3339).map_err(|e| {
            let msg = format!("Error parsing RFC3339 DateTime string: {}: {}", rfc3339, e);
            error!("{}", &msg);
            Self::Error::new(&msg)
        })?;
        let inner = inner.with_timezone(&Utc);
        Ok(Self { inner })
    }
}

impl TryFrom<&String> for DateTime {
    type Error = SquareApiError;

    /// Attempts to generate a `DateTime` from RFC3339 formatted String slice.
    ///
    /// Returns an API Error if the input String cannot be parsed.
    fn try_from(rfc3339: &String) -> Result<Self, Self::Error> {
        let inner = chrono::DateTime::parse_from_rfc3339(rfc3339.as_str()).map_err(|e| {
            let msg = format!("Error parsing RFC3339 DateTime string: {}: {}", rfc3339, e);
            error!("{}", &msg);
            Self::Error::new(&msg)
        })?;
        let inner = inner.with_timezone(&Utc);
        Ok(Self { inner })
    }
}
