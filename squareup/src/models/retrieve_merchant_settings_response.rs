//! Response struct for the Retrieve Location Settings API

use crate::models::checkout_merchant_settings::CheckoutMerchantSettings;
use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for RetrieveMerchantSettingsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveMerchantSettingsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The merchant settings.
    pub location_settings: Option<CheckoutMerchantSettings>,
}
