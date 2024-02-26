//! Model for OrderFulfillmentType enum

use serde::{Deserialize, Serialize};

/// The type of fulfillment.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderFulfillmentType {
    /// A fulfillment to be picked up from a physical [Location] by a recipient.
    Pickup,
    /// A fulfillment to be shipped by a shipping carrier.
    Shipment,
    /// A courier to deliver the fulfillment.
    Delivery,
    /// A digital order of intangible product
    /// Warning: Square's documentation is lacking this variant in many places!
    /// Only found (as of 2/26/24) in example response here:
    /// https://developer.squareup.com/reference/square/checkout-api/create-payment-link
    Digital,
}
