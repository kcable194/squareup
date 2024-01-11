//! Model for BusinessAppointmentSettingsCancellationPolicy type.

use serde::{Deserialize, Serialize};

/// The cancellation policy adopted by the seller.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAppointmentSettingsCancellationPolicy {
    /// Cancellations are treated as no shows and may incur a fee as specified by
    /// cancellation_fee_money.
    CancellationTreatedAsNoShow,
    /// Cancellations follow the seller-specified policy that is described in free-form text and
    /// not enforced automatically by Square.
    CustomPolicy,
}
