//! Response body struct for the OrderCreatedObject type

use crate::models::OrderCreated;
use serde::{Deserialize, Serialize};

/// This is a model struct for OrderCreatedObject type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OrderCreatedObject {
    /// Information about the created order.
    pub order_created: OrderCreated,
}
