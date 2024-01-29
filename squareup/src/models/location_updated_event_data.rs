//! Response body struct for the LocationUpdatedEventData type

use serde::{Deserialize, Serialize};

/// This is a model struct for LocationUpdatedEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocationUpdatedEventData {
    /// The type of the event data object. The value is "location". Max Length 50
    pub r#type: String,
    /// The ID of the updated Location
    pub id: String,
}
