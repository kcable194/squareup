//! Response struct for the Booking Search Availability API

use serde::Deserialize;

use super::{errors::Error, Availability};

/// This is a model struct for SearchAvailabilityResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct SearchAvailabilityResponse {
    /// Returned items matching the specified query expressions.
    pub availabilities: Option<Vec<Availability>>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
