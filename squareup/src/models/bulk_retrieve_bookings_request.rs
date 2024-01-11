//! Request body struct for the Bulk Retrieve Bookings API

use serde::Serialize;

/// This is a model struct for the BulkRetrieveBookingsRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkRetrieveBookingsRequest {
    /// A non-empty list of Booking IDs specifying bookings to retrieve.
    /// Min Length 1, Max Length 10
    pub booking_ids: Vec<String>,
}
