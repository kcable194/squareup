//! Response body struct for the TeamMemberCreatedEventData type

use crate::models::TeamMemberCreatedEventObject;
use serde::Deserialize;

/// This is a model struct for TeamMemberCreatedEventData type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct TeamMemberCreatedEventData {
    /// The type of the event data object. The value is "team_member". Max Length 50
    pub r#type: String,
    /// The ID of the event data object. Max Length 192
    pub id: String,
    /// An object containing the created team member.
    pub object: TeamMemberCreatedEventObject,
}
