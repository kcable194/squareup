//! Query parameters for the List Location Booking Profiles API

use std::fmt::Display;

/// This is a model struct for ListLocationBookingProfilesParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListLocationBookingProfilesParameters {
    /// The maximum number of results per page to return in a paged response.
    pub limit: Option<i32>,
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// The page size is currently set to be 100.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
}

impl ListLocationBookingProfilesParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListLocationBookingProfilesParameters> for String {
    fn from(
        list_location_booking_profiles_parameters: ListLocationBookingProfilesParameters,
    ) -> Self {
        list_location_booking_profiles_parameters.to_string()
    }
}

impl Display for ListLocationBookingProfilesParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
