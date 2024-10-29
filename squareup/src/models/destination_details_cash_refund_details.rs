//! Model struct for DestinationDetailsCashRefundDetails type

use crate::models::Money;
use serde::{Deserialize, Serialize};

/// Stores details about a cash refund. Contains only non-confidential information.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DestinationDetailsCashRefundDetails {
    /// The amount and currency of the money supplied by the seller.
    pub seller_supplied_money: Money,
    /// The amount of change due back to the seller. This read-only field is calculated from the
    /// amount_money and seller_supplied_money fields.
    pub change_back_money: Option<Money>,
}
