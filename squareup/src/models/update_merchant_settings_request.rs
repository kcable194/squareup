//! Request struct for the Update Merchant Settings API

use crate::models::checkout_merchant_settings::CheckoutMerchantSettings;
use serde::Serialize;

/// This is a model struct for UpdateMerchantSettingsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateMerchantSettingsRequest {
    /// Describe your updates using the merchant_settings object. Make sure it contains only the
    /// fields that have changed.
    pub merchant_settings: CheckoutMerchantSettings,
}
