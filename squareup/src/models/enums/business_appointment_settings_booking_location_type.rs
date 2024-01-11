//! Model for BusinessAppointmentSettingsBookingLocationType type.

use serde::{Deserialize, Serialize};

/// The location type of a business, specific to appointments
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsBookingLocationType {
    /// The service is provided at a seller location.
    BusinessLocation,
    /// The service is provided at a customer location.
    CustomerLocation,
    /// The service is provided over the phone.
    Phone,
}
