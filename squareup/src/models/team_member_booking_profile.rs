//! Model struct for TeamMemberBookingProfile type

use serde::{Deserialize, Serialize};

/// The booking profile of a seller's team member, including the team member's ID, display name,
/// description and whether the team member can be booked as a service provider.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TeamMemberBookingProfile {
    /// **Read only** The ID of the TeamMember object for the team member associated with the
    /// booking profile.
    ///  Max Length 32
    pub team_member_id: Option<String>,
    /// **Read only** The description of the team member.
    ///  Max Length 65536
    pub description: Option<String>,
    /// **Read only** The display name of the team member.
    ///  Max Length 512
    pub display_name: Option<String>,
    /// Indicates whether the team member can be booked through the Bookings API or the seller's
    /// online booking channel or site (true) or not (false).
    pub is_bookable: Option<bool>,
    /// **Read only** The URL of the team member's image for the bookings profile.
    /// Max Length 2048
    pub profile_image_url: Option<String>,
}
