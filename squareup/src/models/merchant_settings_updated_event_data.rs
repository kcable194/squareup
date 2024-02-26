//! Response body struct for the MerchantSettingsUpdatedEventData type

use crate::models::MerchantSettingsUpdatedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for MerchantSettingsUpdatedEventData type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MerchantSettingsUpdatedEventData {
    /// Name of the updated object’s type, "online_checkout.merchant_settings".
    pub r#type: String,
    /// ID of the updated merchant settings.
    pub id: String,
    /// An object containing the updated merchant settings.
    pub object: MerchantSettingsUpdatedEventObject,
}
