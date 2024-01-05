//! Model for DigitalWalletBrand enum

use serde::{Deserialize, Serialize};

/// The brand used for the `WALLET` payment.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DigitalWalletBrand {
    CashApp,
    /// I'm not sure if this a typo in Square's docs
    Paypay,
    Unknown
}