//! Response struct for the Retrieve Booking API

use serde::Deserialize;

use super::{Booking, errors::Error};

/// This is a model struct for RetrieveBookingResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveBookingResponse {
    /// The [Booking] that was requested.
    pub booking: Option<Booking>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
