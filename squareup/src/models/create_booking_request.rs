//! Request struct for the Create Booking API

use crate::models::Booking;
use serde::Serialize;

/// This is a model class for CreateBookingRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CreateBookingRequest {
    /// A unique key to make this request an idempotent operation.
    /// Max Length 255
    pub idempotency_key: Option<String>,
    /// The details of the booking to be created.
    pub booking: Booking,
}
