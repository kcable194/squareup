//! Response body struct for the GiftCardEventData type

use crate::models::GiftCardEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for GiftCardEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct GiftCardEventData {
    /// The type of the event data object. The value is "gift_card". Max Length 50
    pub r#type: String,
    /// The ID of the associated gift card.
    pub id: String,
    /// An object that contains the associated gift card and the ID of the linked customer.
    pub object: GiftCardEventObject,
}
