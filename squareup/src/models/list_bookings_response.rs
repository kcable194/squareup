//! Response struct for the List Bookings API

use serde::Deserialize;

use super::{errors::Error, Booking};

/// This is a model struct for ListBookingsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListBookingsResponse {
    /// The list of targeted bookings.
    pub bookings: Option<Vec<Booking>>,
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
