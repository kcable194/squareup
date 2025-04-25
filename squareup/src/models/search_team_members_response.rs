//! Response body struct for the Search Team Members API

use serde::{Deserialize, Serialize};

use super::{TeamMember, errors::Error};

/// This is a model struct for SearchTeamMembersResponse type.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub struct SearchTeamMembersResponse {
    /// The filtered list of `TeamMember` objects.
    pub team_members: Option<Vec<TeamMember>>,
    /// The opaque cursor for fetching the next page. For more information, see
    /// [pagination](https://developer.squareup.com/docs/working-with-apis/pagination).
    pub cursor: Option<String>,
    /// The errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
