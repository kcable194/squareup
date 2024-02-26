//! Response body struct for the TeamMemberEventObject type

use crate::models::TeamMember;
use serde::{Deserialize, Serialize};

/// This is a model struct for TeamMemberEventObject type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct TeamMemberEventObject {
    /// The associated team member.
    pub team_member: TeamMember,
}
