//! Response body struct for the BookingCreatedEventObject type

use crate::models::Booking;
use serde::{Deserialize, Serialize};

/// This is a model struct for BookingCreatedEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct BookingCreatedEventObject {
    /// The created booking.
    pub booking: Booking,
}
