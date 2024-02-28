//! Enum for OrderEventData type.

use crate::models::{OrderCreatedObject, OrderFulfillmentUpdatedObject, OrderUpdatedObject};
use serde::{Deserialize, Serialize};

/// The type of order event data coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type", content = "object")]
#[serde(rename_all = "snake_case")]
pub enum OrderEventObject {
    OrderCreated(OrderCreatedObject),
    OrderFulfillmentUpdated(OrderFulfillmentUpdatedObject),
    OrderUpdated(OrderUpdatedObject),
}
