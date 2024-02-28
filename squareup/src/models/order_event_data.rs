//! Response body struct for the OrderEventData type

use crate::models::enums::OrderEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for OrderEventData type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OrderEventData {
    /// The type of the event data object.
    pub r#type: String,
    /// The ID of the affected order.
    pub id: String,
    /// An object containing information about the associated Order.
    pub object: OrderEventObject,
}
