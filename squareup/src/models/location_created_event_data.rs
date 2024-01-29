//! Response body struct for the LocationCreatedEventData type

use serde::{Deserialize, Serialize};

/// This is a model struct for LocationCreatedEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocationCreatedEventData {
    /// The type of the event data object. The value is "location". Max Length 50
    pub r#type: String,
    /// The ID of the created Location
    pub id: String,
}
