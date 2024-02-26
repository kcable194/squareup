//! Response body struct for the LocationEventData type

use serde::{Deserialize, Serialize};

/// This is a model struct for LocationEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct LocationEventData {
    /// The type of the event data object. The value is "location". Max Length 50
    pub r#type: String,
    /// The ID of the associated Location
    pub id: String,
}
