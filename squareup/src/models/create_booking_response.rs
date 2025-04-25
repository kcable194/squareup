//! Response struct for the Create Booking API

use serde::Deserialize;

use super::{Booking, errors::Error};

/// This is a model struct for CreateBookingResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CreateBookingResponse {
    /// The [Booking] that was created.
    pub booking: Option<Booking>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
