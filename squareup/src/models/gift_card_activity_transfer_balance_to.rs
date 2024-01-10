//! Model struct for GiftCardActivityTransferBalanceTo type

use crate::models::Money;
use serde::{Deserialize, Serialize};

/// Represents details about a TRANSFER_BALANCE_TO gift card activity type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GiftCardActivityTransferBalanceTo {
    /// The ID of the gift card from which the specified amount was transferred.
    pub transfer_from_gift_card_id: String,
    /// The amount added to the gift card balance for the transfer. This value is a positive integer.
    pub amount_money: Money,
}
