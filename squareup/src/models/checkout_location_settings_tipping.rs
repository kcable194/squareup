//! Model struct for CheckoutLocationSettingsTipping type

use crate::models::Money;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutLocationSettingsTipping {
    /// Set three custom percentage amounts that buyers can select at checkout. If Smart Tip is
    /// enabled, this only applies to transactions totaling $10 or more.
    /// Max Length 3
    pub percentages: Option<Vec<String>>,
    /// Enables Smart Tip Amounts. If Smart Tip Amounts is enabled, tipping works as follows: If a
    /// transaction is less than $10, the available tipping options include No Tip, $1, $2, or $3.
    /// If a transaction is $10 or more, the available tipping options include No Tip, 15%, 20%, or
    /// 25%. You can set custom percentage amounts with the percentages field.
    pub smart_tipping_enabled: Option<bool>,
    /// Set the pre-selected percentage amounts that appear at checkout. If Smart Tip is enabled,
    /// this only applies to transactions totaling $10 or more.
    pub default_percent: Option<i32>,
    /// Show the Smart Tip Amounts for this location.
    pub smart_tips: Option<Vec<Money>>,
    /// Set the pre-selected whole amount that appears at checkout when Smart Tip is enabled and
    /// the transaction amount is less than $10.
    pub default_smart_tip: Option<Money>,
}
