//! Response struct for the List Team Member Booking Profiles API

use serde::Deserialize;

use super::{errors::Error, TeamMemberBookingProfile};

/// This is a model struct for ListTeamMemberBookingProfilesResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListTeamMemberBookingProfilesResponse {
    /// The list of team member booking profiles. The results are returned in the ascending
    /// order of the time when the team member booking profiles were last updated. Multiple
    /// booking profiles updated at the same time are further sorted in the ascending order
    /// of their IDs.
    pub team_member_booking_profiles: Option<Vec<TeamMemberBookingProfile>>,
    /// The pagination cursor to be used in a subsequent request. If unset, this is the final
    /// response.
    ///  Max Length 65536
    ///
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for more
    /// information.
    pub cursor: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
