//! Response body struct for the BookingUpdatedEventObject type

use crate::models::Booking;
use serde::Deserialize;

/// This is a model struct for BookingUpdatedEventObject type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BookingUpdatedEventObject {
    /// The updated booking.
    pub booking: Booking,
}
