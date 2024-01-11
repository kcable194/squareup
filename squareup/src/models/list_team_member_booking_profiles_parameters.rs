//! Query parameters for the List Team Member Booking Profiles API

use std::fmt::Display;

/// This is a model struct for ListTeamMemberBookingProfilesParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListTeamMemberBookingProfilesParameters {
    /// Indicates whether to include only bookable team members in the returned result (true) or
    /// not (false).
    pub bookable_only: Option<bool>,
    /// The maximum number of results to return in a paged response.
    pub limit: Option<i32>,
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// The page size is currently set to be 100.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
    /// Indicates whether to include only team members enabled at the given location in the
    /// returned result.
    pub location_id: Option<String>,
}

impl ListTeamMemberBookingProfilesParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListTeamMemberBookingProfilesParameters> for String {
    fn from(
        list_team_member_booking_profiles_parameters: ListTeamMemberBookingProfilesParameters,
    ) -> Self {
        list_team_member_booking_profiles_parameters.to_string()
    }
}

impl Display for ListTeamMemberBookingProfilesParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(bookable_only) = &self.bookable_only {
            params.push(format!("bookable_only={}", bookable_only));
        }

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(location_id) = &self.location_id {
            params.push(format!("location_id={}", location_id));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
