//! Response body struct for the BookingCreatedEventObject type

use crate::models::Booking;
use serde::Deserialize;

/// This is a model struct for BookingCreatedEventObject type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BookingCreatedEventObject {
    /// The created booking.
    pub booking: Booking,
}
