//! Enum for OnlineCheckoutMerchantSettingsEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of online checkout merchant settings event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OnlineCheckoutMerchantSettingsEventType {
    #[serde(rename = "online_checkout.merchant_settings.updated")]
    OnlineCheckoutMerchantSettingsUpdated,
}

impl Display for OnlineCheckoutMerchantSettingsEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnlineCheckoutMerchantSettingsEventType::OnlineCheckoutMerchantSettingsUpdated => {
                write!(f, "online_checkout.merchant_settings.updated")
            }
        }
    }
}
