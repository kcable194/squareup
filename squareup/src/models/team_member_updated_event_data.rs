//! Response body struct for the TeamMemberUpdatedEventData type

use crate::models::TeamMemberUpdatedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for TeamMemberCreatedEventData type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct TeamMemberUpdatedEventData {
    /// The type of the event data object. The value is "team_member". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the created team member.
    pub object: TeamMemberUpdatedEventObject,
}
