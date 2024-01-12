//! Response struct for the Cancel Booking API

use serde::Deserialize;

use super::{errors::Error, Booking};

/// This is a model struct for CancelBookingResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct CancelBookingResponse {
    /// The [Booking] that was canceled.
    pub booking: Option<Booking>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
