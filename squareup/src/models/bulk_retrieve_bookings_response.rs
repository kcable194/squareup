//! Response body struct for the Bulk Retrieve Bookings API

use crate::models::RetrieveBookingResponse;
use serde::Deserialize;
use std::collections::HashMap;

use super::errors::Error;

/// This is a model struct for the BulkRetrieveBookingsResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkRetrieveBookingsResponse {
    /// Requested bookings returned as a map containing booking_id as the key and
    /// [RetrieveBookingResponse] as the value.
    pub bookings: Option<HashMap<String, RetrieveBookingResponse>>,
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
