//! Model struct for CatalogAvailabilityPeriod type.

use crate::models::enums::DayOfWeek;
use serde::{Deserialize, Serialize};

/// Represents a time period of availability.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogAvailabilityPeriod {
    /// The start time of an availability period, specified in local time using partial-time
    /// RFC 3339 format. For example, 8:30:00 for a period starting at 8:30 in the morning.
    /// Note that the seconds value is always :00, but it is appended for conformance to the RFC.
    pub start_local_time: Option<String>,
    /// The end time of an availability period, specified in local time using partial-time
    /// RFC 3339 format. For example, 21:00:00 for a period ending at 9:00 in the evening.
    /// Note that the seconds value is always :00, but it is appended for conformance to the RFC.
    pub end_local_time: Option<String>,
    /// The day of the week for this availability period.
    pub day_of_week: Option<DayOfWeek>,
}
