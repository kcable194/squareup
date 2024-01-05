//! Model for DestinationType enum

use serde::{Deserialize, Serialize};

/// The destination type for this refund.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DestinationType {
    Card,
    BankAccount,
    Wallet,
    BuyNowPayLater,
    Cash,
    External,
}