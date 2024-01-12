//! Request struct for the Update Booking API

use crate::models::Booking;
use serde::Serialize;

/// This is a model class for UpdateBookingRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateBookingRequest {
    /// A unique key to make this request an idempotent operation.
    /// Max Length 255
    pub idempotency_key: Option<String>,
    /// The booking to be updated. Individual attributes explicitly specified here override the
    /// corresponding values of the existing booking.
    pub booking: Booking,
}
