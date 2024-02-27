//! Response body struct for the CustomerEventData type

use serde::{Deserialize, Serialize};
use crate::models::CustomerEventObject;

/// This is a model struct for CustomerEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CustomerEventData {
    /// The type of the event data object. The value is "customer". Max Length 50
    pub r#type: String,
    /// The ID of the customer associated with the event. Max Length 192
    pub id: String,
    /// An object containing the customer associated with the event.
    pub object: CustomerEventObject,
}
