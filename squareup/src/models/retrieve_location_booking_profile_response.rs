//! Response struct for the Retrieve Location Booking Profile API

use serde::Deserialize;

use super::{LocationBookingProfile, errors::Error};

/// This is a model struct for RetrieveLocationBookingProfileResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveLocationBookingProfileResponse {
    /// The requested location booking profile.
    pub location_booking_profile: Option<LocationBookingProfile>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
