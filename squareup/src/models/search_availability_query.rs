//! Model struct for SearchAvailabilityQuery type

use crate::models::SearchAvailabilityFilter;
use serde::Serialize;

/// The query used to search for buyer-accessible availabilities of bookings.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SearchAvailabilityQuery {
    /// The query filter to search for buyer-accessible availabilities of existing bookings.
    pub filter: SearchAvailabilityFilter,
}
