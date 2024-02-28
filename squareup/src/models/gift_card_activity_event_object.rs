//! Response body struct for the GiftCardActivityEventObject type

use crate::models::GiftCardActivity;
use serde::{Deserialize, Serialize};

/// This is a model struct for GiftCardActivityEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct GiftCardActivityEventObject {
    /// The gift card activity associated with the event.
    pub gift_card_activity: GiftCardActivity,
}
