//! Response body struct for the OrderCreated type

use crate::models::enums::OrderState;
use crate::models::DateTime;
use serde::{Deserialize, Serialize};

/// This is a model struct for OrderCreated type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct OrderCreated {
    /// The order's unique ID.
    pub order_id: String,
    /// The version number, which is incremented each time an update is committed to the order.
    /// Orders that were not created through the API do not include a version number and therefore
    /// cannot be updated.
    pub version: Option<i32>,
    /// The ID of the seller location that this order is associated with.
    pub location_id: String,
    /// The state of the order.
    pub state: OrderState,
    /// **Read only** The timestamp of when the event was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: DateTime,
}
