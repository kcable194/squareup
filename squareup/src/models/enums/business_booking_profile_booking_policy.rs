//! Model for BusinessBookingProfileBookingPolicy type.

use serde::{Deserialize, Serialize};

/// The policy for the seller to automatically accept booking requests (AcceptAll) or not
/// (RequiresAcceptance).
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessBookingProfileBookingPolicy {
    /// The seller accepts all booking requests automatically.
    AcceptAll,
    /// The seller must accept requests to complete bookings.
    RequiresAcceptance,
}
