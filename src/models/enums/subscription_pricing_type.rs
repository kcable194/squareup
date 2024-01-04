//! Model for SubscriptionPricingType enum.

use serde::{Deserialize, Serialize};

/// Supported types of a subscription pricing.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SubscriptionPricingType {
    /// Static pricing
    Static,
    /// Relative pricing
    Relative,
}