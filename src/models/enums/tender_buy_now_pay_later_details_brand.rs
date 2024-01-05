//! Model for TenderBuyNowPayLaterDetailsBrand enum.

use serde::{Deserialize, Serialize};

/// The Buy Now Pay Later brand.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderBuyNowPayLaterDetailsBrand {
    OtherBrand,
    Afterpay,
}