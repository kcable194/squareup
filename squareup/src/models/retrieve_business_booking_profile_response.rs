//! Response struct for the Retrieve Business Booking Profile API

use serde::Deserialize;

use super::{errors::Error, BusinessBookingProfile};

/// This is a model struct for RetrieveBusinessBookingProfileResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveBusinessBookingProfileResponse {
    /// The seller's booking profile.
    pub business_booking_profile: Option<BusinessBookingProfile>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
