//! Request struct for the Update Location Settings API

use serde::Serialize;

use super::CheckoutLocationSettings;

/// This is a model struct for UpdateLocationSettingsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateLocationSettingsRequest {
    /// Describe your updates using the location_settings object. Make sure it contains only the
    /// fields that have changed.
    pub location_settings: CheckoutLocationSettings,
}
