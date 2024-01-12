//! Response body struct for the Bulk Retrieve Team Member Booking Profiles API

use crate::models::RetrieveTeamMemberBookingProfileResponse;
use serde::Deserialize;
use std::collections::HashMap;

use super::errors::Error;

/// This is a model struct for the BulkRetrieveTeamMemberBookingProfilesResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkRetrieveTeamMemberBookingProfilesResponse {
    /// The returned team members' booking profiles, as a map with team_member_id as the key and
    /// [TeamMemberBookingProfile] the value.
    pub team_member_booking_profiles:
        Option<HashMap<String, RetrieveTeamMemberBookingProfileResponse>>,
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
