//! Model for BookingCreatorDetailsCreatorType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of [Booking] creator
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BookingCreatorDetailsCreatorType {
    /// The creator is of the seller type.
    TeamMember,
    /// The creator is of the buyer type.
    Customer,
}

impl Display for BookingCreatorDetailsCreatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingCreatorDetailsCreatorType::TeamMember => {
                write!(f, "TEAM_MEMBER")
            }
            BookingCreatorDetailsCreatorType::Customer => {
                write!(f, "CUSTOMER")
            }
        }
    }
}
