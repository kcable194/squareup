//! Request struct for the Cancel Booking API

use serde::Serialize;

/// This is a model class for CancelBookingRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct CancelBookingRequest {
    /// A unique key to make this request an idempotent operation.
    /// Max Length 255
    pub idempotency_key: Option<String>,
    /// The revision number for the booking used for optimistic concurrency.
    pub booking_version: Option<i32>,
}
