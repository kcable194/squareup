//! Model for CardCoBrand enum

use serde::{Deserialize, Serialize};

/// Indicates a card's co-brand, such as `AFTERPAY` or `CLEARPAY`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardCoBrand {
    Unknown,
    Afterpay,
    Clearpay,
}
