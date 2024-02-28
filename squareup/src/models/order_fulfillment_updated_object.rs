//! Response body struct for the OrderFulfillmentUpdatedObject type

use crate::models::OrderFulfillmentUpdated;
use serde::{Deserialize, Serialize};

/// This is a model struct for OrderFulfillmentUpdatedObject type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OrderFulfillmentUpdatedObject {
    /// Information about the updated order.
    pub order_fulfillment_updated: OrderFulfillmentUpdated,
}
