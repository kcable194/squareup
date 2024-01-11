//! Model struct for LocationBookingProfile type

use serde::{Deserialize, Serialize};

/// The booking profile of a seller's location, including the location's ID and whether the
/// location is enabled for online booking.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LocationBookingProfile {
    /// The ID of the location
    pub location_id: Option<String>,
    /// Url for the online booking site for this location.
    pub booking_site_url: Option<String>,
    /// Indicates whether the location is enabled for online booking.
    pub online_booking_enabled: Option<String>,
}
