//! Response body struct for the TeamMemberCreatedEventObject type

use crate::models::TeamMember;
use serde::{Deserialize, Serialize};

/// This is a model struct for TeamMemberCreatedEventObject type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct TeamMemberCreatedEventObject {
    /// The created team member.
    pub team_member: TeamMember,
}
