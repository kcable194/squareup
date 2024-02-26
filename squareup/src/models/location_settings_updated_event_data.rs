//! Response body struct for the LocationSettingsUpdatedEventData type

use crate::models::LocationSettingsUpdatedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for LocationSettingsUpdatedEventData type.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocationSettingsUpdatedEventData {
    /// Name of the updated object’s type, "online_checkout.location_settings".
    pub r#type: String,
    /// ID of the updated location settings.
    pub id: String,
    /// An object containing the updated location settings.
    pub object: LocationSettingsUpdatedEventObject,
}
