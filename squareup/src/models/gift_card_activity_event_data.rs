//! Response body struct for the GiftCardActivityEventData type

use crate::models::GiftCardActivityEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for GiftCardActivityEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct GiftCardActivityEventData {
    /// The type of the event data object. The value is "gift_card_activity". Max Length 50
    pub r#type: String,
    /// The ID of the associated gift card activity.
    pub id: String,
    /// An object containing the gift card activity associated with the event.
    pub object: GiftCardActivityEventObject,
}
