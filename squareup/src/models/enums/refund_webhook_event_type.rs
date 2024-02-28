//! Enum for RefundWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of refund event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RefundWebhookEventType {
    #[serde(rename = "refund.created")]
    RefundCreated,
    #[serde(rename = "refund.updated")]
    RefundUpdated,
}

impl Display for RefundWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RefundWebhookEventType::RefundCreated => {
                write!(f, "refund.created")
            }
            RefundWebhookEventType::RefundUpdated => {
                write!(f, "refund.updated")
            }
        }
    }
}
