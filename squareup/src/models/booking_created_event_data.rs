//! Response body struct for the BookingCreatedEventData type

use crate::models::BookingCreatedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for BookingCreatedEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct BookingCreatedEventData {
    /// The type of the event data object. The value is "booking". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the created booking.
    pub object: BookingCreatedEventObject,
}
