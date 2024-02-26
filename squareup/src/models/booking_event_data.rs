//! Response body struct for the BookingEventData type

use crate::models::booking_event_object::BookingEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for BookingEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct BookingEventData {
    /// The type of the event data object. The value is "booking". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the booking associated with the event.
    pub object: BookingEventObject,
}
