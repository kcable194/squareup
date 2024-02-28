//! Response body struct for the OrderUpdatedObject type

use crate::models::OrderUpdated;
use serde::{Deserialize, Serialize};

/// This is a model struct for OrderUpdatedObject type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OrderUpdatedObject {
    /// Information about the updated order.
    pub order_updated: OrderUpdated,
}
