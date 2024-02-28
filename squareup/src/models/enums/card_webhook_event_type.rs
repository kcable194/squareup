//! Enum for CardWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of card event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CardWebhookEventType {
    #[serde(rename = "card.automatically_updated")]
    CardAutomaticallyUpdated,
    #[serde(rename = "card.created")]
    CardCreated,
    #[serde(rename = "card.disabled")]
    CardDisabled,
    #[serde(rename = "card.forgotten")]
    CardForgotten,
    #[serde(rename = "card.updated")]
    CardUpdated,
}

impl Display for CardWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CardWebhookEventType::CardAutomaticallyUpdated => {
                write!(f, "card.automatically_updated")
            }
            CardWebhookEventType::CardCreated => {
                write!(f, "card.created")
            }
            CardWebhookEventType::CardDisabled => {
                write!(f, "card.disabled")
            }
            CardWebhookEventType::CardForgotten => {
                write!(f, "card.forgotten")
            }
            CardWebhookEventType::CardUpdated => {
                write!(f, "card.updated")
            }
        }
    }
}
