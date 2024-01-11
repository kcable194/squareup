//! Model for BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType type.

use serde::{Deserialize, Serialize};

/// Indicates whether the daily appointment limit applies to team members or to business locations.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsMaxAppointmentsPerDayLimitType {
    /// The maximum number of daily appointments is set on a per team member basis.
    PerTeamMember,
    /// The maximum number of daily appointments is set on a per location basis.
    PerLocation,
}
