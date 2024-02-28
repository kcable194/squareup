//! Response body struct for the GiftCardEventObject type

use crate::models::GiftCard;
use serde::{Deserialize, Serialize};

/// This is a model struct for GiftCardEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct GiftCardEventObject {
    /// The gift card with the associated customer_ids field.
    pub gift_card: GiftCard,
    /// The ID of the linked customer
    pub linked_customer_id: String,
}
