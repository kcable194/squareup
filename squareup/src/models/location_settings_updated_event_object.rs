//! Response body struct for the LocationSettingsUpdatedEventObject type

use crate::models::CheckoutLocationSettings;
use serde::{Deserialize, Serialize};

/// This is a model struct for LocationSettingsUpdatedEventObject type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocationSettingsUpdatedEventObject {
    /// The updated location settings.
    pub location_settings: CheckoutLocationSettings,
}
