//! Response body struct for the BookingUpdatedEventData type

use crate::models::BookingUpdatedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for BookingUpdatedEventData type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct BookingUpdatedEventData {
    /// The type of the event data object. The value is "booking". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the updated booking.
    pub object: BookingUpdatedEventObject,
}
