//! Response struct for the Retrieve Location Settings API

use serde::Deserialize;

use super::{CheckoutLocationSettings, errors::Error};

/// This is a model struct for RetrieveLocationSettingsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveLocationSettingsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The CheckoutLocationSettings returned.
    pub location_settings: Option<CheckoutLocationSettings>,
}
