//! Query parameters for the List Bookings API

use crate::models::DateTime;
use std::fmt::Display;

/// This is a model struct for ListBookingsParameters (query parameters)
#[derive(Clone, Debug, Default)]
pub struct ListBookingsParameters {
    /// The maximum number of results per page to return in a paged response.
    pub limit: Option<i32>,
    /// The pagination cursor returned in the previous response. Leave unset for an initial request.
    /// The page size is currently set to be 100.
    /// See [Pagination](https://developer.squareup.com/docs/basics/api101/pagination) for
    /// more information.
    pub cursor: Option<String>,
    /// The customer for whom to retrieve bookings. If this is not set, bookings for all
    /// customers are retrieved.
    pub customer_id: Option<String>,
    /// The team member for whom to retrieve bookings. If this is not set, bookings of all
    /// members are retrieved.
    pub team_member_id: Option<String>,
    /// The location for which to retrieve bookings. If this is not set, all locations' bookings
    /// are retrieved.
    pub location_id: Option<String>,
    /// The RFC 3339 timestamp specifying the earliest of the start time. If this is not set,
    /// the current time is used.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time: UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub start_at_min: Option<DateTime>,
    /// The RFC 3339 timestamp specifying the latest of the start time. If this is not set,
    /// the time of 31 days after start_at_min is used.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time: UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub start_at_max: Option<DateTime>,
}

impl ListBookingsParameters {
    pub fn to_query_string(&self) -> String {
        self.to_string()
    }
}

impl From<ListBookingsParameters> for String {
    fn from(list_bookings_parameters: ListBookingsParameters) -> Self {
        list_bookings_parameters.to_string()
    }
}

impl Display for ListBookingsParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut params = Vec::new();

        if let Some(limit) = &self.limit {
            params.push(format!("limit={}", limit));
        }

        if let Some(cursor) = &self.cursor {
            params.push(format!("cursor={}", cursor));
        }

        if let Some(customer_id) = &self.customer_id {
            params.push(format!("customer_id={}", customer_id));
        }

        if let Some(team_member_id) = &self.team_member_id {
            params.push(format!("team_member_id={}", team_member_id));
        }

        if let Some(location_id) = &self.location_id {
            params.push(format!("location_id={}", location_id));
        }

        if let Some(start_at_min) = &self.start_at_min {
            params.push(format!("start_at_min={:?}", start_at_min));
        }

        if let Some(start_at_max) = &self.start_at_max {
            params.push(format!("start_at_max={:?}", start_at_max));
        }

        let str = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.join("&"))
        };
        write!(f, "{}", str)
    }
}
