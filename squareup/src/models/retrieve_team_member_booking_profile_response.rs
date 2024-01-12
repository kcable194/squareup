//! Response body struct for the Retrieve Team Member Booking Profiles API

use crate::models::TeamMemberBookingProfile;
use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for the RetrieveTeamMemberBookingProfileResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveTeamMemberBookingProfileResponse {
    /// The returned team member booking profile.
    pub team_member_booking_profile: Option<TeamMemberBookingProfile>,
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
