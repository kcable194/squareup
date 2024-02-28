//! Enum for OrderWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of order event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OrderWebhookEventType {
    #[serde(rename = "order.created")]
    OrderCreated,
    #[serde(rename = "order.fulfillment.updated")]
    OrderFulfillmentUpdated,
    #[serde(rename = "order.updated")]
    OrderUpdated,
}

impl Display for OrderWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderWebhookEventType::OrderCreated => {
                write!(f, "order.created")
            }
            OrderWebhookEventType::OrderFulfillmentUpdated => {
                write!(f, "order.fulfillment.updated")
            }
            OrderWebhookEventType::OrderUpdated => {
                write!(f, "order.updated")
            }
        }
    }
}
