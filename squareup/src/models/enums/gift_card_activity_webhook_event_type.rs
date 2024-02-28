//! Enum for GiftCardActivityWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of gift card activity event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GiftCardActivityWebhookEventType {
    #[serde(rename = "gift_card.activity.created")]
    GiftCardActivityCreated,
    #[serde(rename = "gift_card.activity.update")]
    GiftCardActivityUpdate,
}

impl Display for GiftCardActivityWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GiftCardActivityWebhookEventType::GiftCardActivityCreated => {
                write!(f, "gift_card.activity.created")
            }
            GiftCardActivityWebhookEventType::GiftCardActivityUpdate => {
                write!(f, "gift_card.activity.update")
            }
        }
    }
}
