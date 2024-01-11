//! Model for BookingCreatorDetailsCreatorType type.

use serde::{Deserialize, Serialize};

/// The type of [Booking] creator
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingCreatorDetailsCreatorType {
    /// The creator is of the seller type.
    TeamMember,
    /// The creator is of the buyer type.
    Customer,
}
