//! Model struct for CheckoutLocationSettingsBranding type

use crate::models::enums::{
    CheckoutLocationSettingsBrandingButtonShape, CheckoutLocationSettingsBrandingHeaderType,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutLocationSettingsBranding {
    /// Show the location logo on the checkout page.
    pub header_type: Option<CheckoutLocationSettingsBrandingHeaderType>,
    /// The HTML-supported hex color for the button on the checkout page (for example, "#FFFFFF").
    /// Min Length 7, Max Length 7
    pub button_color: Option<String>,
    /// The shape of the button on the checkout page.
    pub button_shape: Option<CheckoutLocationSettingsBrandingButtonShape>,
}
