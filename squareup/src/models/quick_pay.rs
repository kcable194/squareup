/// Model struct for QuickPay type
use serde::{Deserialize, Serialize};

use super::Money;

/// This is a model class for QuickPay type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct QuickPay {
    /// The price of the item.
    pub name: String,
    /// The ad hoc item name. In the resulting Order, this name appears as the line item name.
    /// Min Length 1, Max Length 255
    pub location_id: String,
    /// The ID of the business location the checkout is associated with.
    pub price_money: Money,
}
