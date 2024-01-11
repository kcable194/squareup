//! Response struct for the List Location Booking Profiles API

use serde::Deserialize;

use super::{errors::Error, LocationBookingProfile};

/// This is a model struct for ListLocationBookingProfilesResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListLocationBookingProfilesResponse {
    /// The list of a seller's location booking profiles.
    pub location_booking_profiles: Option<Vec<LocationBookingProfile>>,
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
