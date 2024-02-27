//! Response body struct for the CardEventData type

use crate::models::CardEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for CardEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CardEventData {
    /// The type of the event data object. The value is "card". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the card associated with the event.
    pub object: CardEventObject,
}
