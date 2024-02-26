//! Response body struct for the TeamMemberEventData type

use crate::models::TeamMemberEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for TeamMemberEventData type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct TeamMemberEventData {
    /// The type of the event data object. The value is "team_member". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the team member associated with the event.
    pub object: TeamMemberEventObject,
}
