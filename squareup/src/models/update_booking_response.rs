//! Response struct for the Update Booking API

use serde::Deserialize;

use super::{Booking, errors::Error};

/// This is a model struct for UpdateBookingResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct UpdateBookingResponse {
    /// The [Booking] that was updated.
    pub booking: Option<Booking>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
