//! Model struct for GiftCardActivityTransferBalanceFrom type

use crate::models::Money;
use serde::{Deserialize, Serialize};

/// Represents details about a TRANSFER_BALANCE_FROM gift card activity type
// .
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityTransferBalanceFrom {
    /// The ID of the gift card to which the specified amount was transferred.
    pub transfer_to_gift_card_id: String,
    /// The amount deducted from the gift card for the transfer. This value is a positive integer.
    pub amount_money: Money,
}
