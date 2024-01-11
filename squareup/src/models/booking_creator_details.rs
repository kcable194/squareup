//! Model struct for BookingCreatorDetails type.

use crate::models::enums::BookingCreatorDetailsCreatorType;
use serde::{Deserialize, Serialize};

/// Information about a booking creator.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct BookingCreatorDetails {
    /// **Read only** The seller-accessible type of the creator of the booking.
    pub creator_type: Option<BookingCreatorDetailsCreatorType>,
    /// **Read only** The ID of the team member who created the booking, when the booking creator
    /// is of the [TEAM_MEMBER] type. Access to this field requires seller-level permissions.
    /// Max Length 32
    pub team_member_id: Option<String>,
    /// **Read only** The ID of the customer who created the booking, when the booking creator is
    /// of the [CUSTOMER] type. Access to this field requires seller-level permissions.
    /// Max Length 192
    pub customer_id: Option<String>,
}
