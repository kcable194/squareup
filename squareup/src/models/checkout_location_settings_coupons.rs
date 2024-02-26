//! Model struct for CheckoutLocationSettingsCoupons type

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutLocationSettingsCoupons {
    /// Indicates whether coupons are enabled for this location.
    pub enabled: Option<bool>,
}
