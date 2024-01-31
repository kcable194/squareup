//! Response body struct for the TeamMemberUpdatedEventObject type

use crate::models::TeamMember;
use serde::{Deserialize, Serialize};

/// This is a model struct for TeamMemberUpdatedEventObject type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct TeamMemberUpdatedEventObject {
    /// The updated team member.
    pub team_member: TeamMember,
}
