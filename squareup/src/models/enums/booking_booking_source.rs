//! Model for BookingBookingSource type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of a booking booking
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingBookingSource {
    /// The booking was created by a seller from a Square Appointments application, such as the
    /// Square Appointments Dashboard or a Square Appointments mobile app.
    FirstPartyMerchant,
    /// The booking was created by a buyer from a Square Appointments application, such as
    /// Square Online Booking Site.
    FirstPartyBuyer,
    /// The booking was created by a buyer created from a third-party application.
    ThirdPartyBuyer,
    /// The booking was created by a seller or a buyer from the Square Bookings API.
    Api,
}

impl Display for BookingBookingSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingBookingSource::FirstPartyMerchant => {
                write!(f, "FIRST_PARTY_MERCHANT")
            }
            BookingBookingSource::FirstPartyBuyer => {
                write!(f, "FIRST_PARTY_BUYER")
            }
            BookingBookingSource::ThirdPartyBuyer => {
                write!(f, "THIRD_PARTY_BUYER")
            }
            BookingBookingSource::Api => {
                write!(f, "API")
            }
        }
    }
}
