//! Model for BusinessBookingProfileCustomerTimezoneChoice type.

use serde::{Deserialize, Serialize};

/// The choice of customer's time zone information of a booking
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessBookingProfileCustomerTimezoneChoice {
    /// Use the time zone of the business location for bookings.
    BusinessLocationTimezone,
    /// Use the customer-chosen time zone for bookings.
    CustomerChoice,
}
