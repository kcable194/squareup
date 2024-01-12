//! Request body struct for the Bulk Retrieve Team Member Booking Profiles API

use serde::Serialize;

/// This is a model struct for the BulkRetrieveTeamMemberBookingProfilesRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkRetrieveTeamMemberBookingProfilesRequest {
    /// A non-empty list of IDs of team members whose booking profiles you want to retrieve.
    /// Min Length 1, Max Length 10
    pub team_member_ids: Vec<String>,
}
