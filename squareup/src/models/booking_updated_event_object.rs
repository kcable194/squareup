//! Response body struct for the BookingUpdatedEventObject type

use crate::models::Booking;
use serde::{Deserialize, Serialize};

/// This is a model struct for BookingUpdatedEventObject type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct BookingUpdatedEventObject {
    /// The updated booking.
    pub booking: Booking,
}
