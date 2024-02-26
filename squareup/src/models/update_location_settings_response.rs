//! Response struct for the Update Location Settings API

use serde::Deserialize;

use super::{errors::Error, CheckoutLocationSettings};

/// This is a model struct for UpdateLocationSettingsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateLocationSettingsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated location settings.
    pub location_settings: Option<CheckoutLocationSettings>,
}
