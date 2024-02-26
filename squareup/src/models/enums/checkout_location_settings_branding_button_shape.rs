//! Model for CheckoutLocationSettingsBrandingButtonShape type.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CheckoutLocationSettingsBrandingButtonShape {
    Squared,
    Rounded,
    Pill,
}
