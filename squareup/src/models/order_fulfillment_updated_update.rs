//! Response body struct for the OrderFulfillmentUpdatedUpdate type

use crate::models::enums::OrderFulfillmentState;
use serde::{Deserialize, Serialize};

/// This is a model struct for OrderFulfillmentUpdatedUpdate type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OrderFulfillmentUpdatedUpdate {
    /// A unique ID that identifies the fulfillment only within this order.
    pub fulfillment_uid: String,
    /// The state of the fulfillment before the change. The state is not populated if the
    /// fulfillment is created with this new Order version.
    pub old_state: Option<OrderFulfillmentState>,
    /// The state of the fulfillment after the change. The state might be equal to old_state if a
    /// non-state field was changed on the fulfillment (such as the tracking number).
    pub new_state: OrderFulfillmentState,
}
