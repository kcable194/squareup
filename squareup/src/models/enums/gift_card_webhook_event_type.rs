//! Enum for GiftCardWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of gift card event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GiftCardWebhookEventType {
    #[serde(rename = "gift_card.created")]
    GiftCardCreated,
    #[serde(rename = "gift_card.customer_linked")]
    GiftCardCustomerLinked,
    #[serde(rename = "gift_card.customer_unlinked")]
    GiftCardCustomerUnlinked,
    #[serde(rename = "gift_card.update")]
    GiftCardUpdate,
}

impl Display for GiftCardWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GiftCardWebhookEventType::GiftCardCreated => {
                write!(f, "gift_card.created")
            }
            GiftCardWebhookEventType::GiftCardCustomerLinked => {
                write!(f, "gift_card.customer_linked")
            }
            GiftCardWebhookEventType::GiftCardCustomerUnlinked => {
                write!(f, "gift_card.customer_unlinked")
            }
            GiftCardWebhookEventType::GiftCardUpdate => {
                write!(f, "gift_card.update")
            }
        }
    }
}
