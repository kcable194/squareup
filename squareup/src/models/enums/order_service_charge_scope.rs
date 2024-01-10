//! Model for OrderServiceChargeScope enum

use serde::{Deserialize, Serialize};

/// Model for OrderServiceChargeScope enum
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderServiceChargeScope {
    /// Used for reporting only. The original transaction service charge scope is currently
    /// not supported by the API.
    OtherServiceChargeScope,
    /// The service charge should be applied to only line items specified by
    /// OrderLineItemAppliedServiceCharge reference records.
    LineItem,
    /// The service charge should be applied to the entire order.
    Order,
}
