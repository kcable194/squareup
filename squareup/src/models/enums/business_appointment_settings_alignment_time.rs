//! Model for BusinessAppointmentSettingsAlignmentTime type.

use serde::{Deserialize, Serialize};

/// The time unit of the service duration for bookings.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsAlignmentTime {
    /// The service duration unit is one visit of a fixed time interval specified by the seller.
    ServiceDuration,
    /// The service duration unit is a 15-minute interval. Bookings can be scheduled every quarter
    /// hour.
    QuarterHourly,
    /// The service duration unit is a 30-minute interval. Bookings can be scheduled every half
    /// hour.
    HalfHourly,
    /// The service duration unit is a 60-minute interval. Bookings can be scheduled every hour.
    Hourly,
}
