//! Model struct for AppointmentSegment type.

use serde::{Deserialize, Serialize};

/// Defines an appointment segment of a booking.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct AppointmentSegment {
    /// The time span in minutes of an appointment segment.
    /// Max 1500
    pub duration_minutes: Option<i32>,
    /// The ID of the CatalogItemVariation object representing the service booked in this segment.
    /// Max Length 36
    pub service_variation_id: Option<String>,
    /// The ID of the TeamMember object representing the team member booked in this segment.
    /// Min Length 1, Max Length 32
    pub team_member_id: String,
    /// The current version of the item variation representing the service booked in this segment.
    pub service_variation_version: Option<i64>,
    /// **Read only** Time between the end of this segment and the beginning of the subsequent
    /// segment.
    pub intermission_minutes: Option<i32>,
    /// **Read only** Whether the customer accepts any team member, instead of a specific one, to
    /// serve this segment.
    pub any_team_member: Option<bool>,
    /// **Read only** The IDs of the seller-accessible resources used for this appointment segment.
    pub resource_ids: Option<Vec<String>>,
}
