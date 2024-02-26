//! Enum for OnlineCheckoutLocationSettingsEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of online checkout location settings event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OnlineCheckoutLocationSettingsEventType {
    #[serde(rename = "online_checkout.location_settings.updated")]
    Updated,
}

impl Display for OnlineCheckoutLocationSettingsEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OnlineCheckoutLocationSettingsEventType::Updated => {
                write!(f, "online_checkout.location_settings.updated")
            }
        }
    }
}
