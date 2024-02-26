//! Response struct for the Update Merchant Settings API

use serde::Deserialize;

use super::{errors::Error, CheckoutMerchantSettings};

/// This is a model struct for UpdateMerchantSettingsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateMerchantSettingsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated merchant settings.
    pub merchant_settings: Option<CheckoutMerchantSettings>,
}
