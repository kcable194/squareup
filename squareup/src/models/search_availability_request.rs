//! Request body struct for the Bookings Search Availability API

use crate::models::SearchAvailabilityQuery;
use serde::Serialize;

/// This is a model struct for SearchAvailabilityRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchAvailabilityRequest {
    /// Query conditions used to filter buyer-accessible booking availabilities.
    pub query: SearchAvailabilityQuery,
}
