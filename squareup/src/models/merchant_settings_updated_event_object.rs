//! Response body struct for the MerchantSettingsUpdatedEventObject type

use crate::models::CheckoutMerchantSettings;
use serde::{Deserialize, Serialize};

/// This is a model struct for MerchantSettingsUpdatedEventObject type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MerchantSettingsUpdatedEventObject {
    /// The updated merchant settings.
    pub merchant_settings: CheckoutMerchantSettings,
}
