//! Response body struct for the BookingEventObject type

use crate::models::Booking;
use serde::{Deserialize, Serialize};

/// This is a model struct for BookingEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct BookingEventObject {
    /// The booking associated with the event.
    pub booking: Booking,
}
