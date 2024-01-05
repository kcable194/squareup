//! Model for DelayAction enum

use serde::{Deserialize, Serialize};

/// The action to be applied to the payment when the delay_duration has elapsed.
/// The action must be CANCEL or COMPLETE.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DelayAction {
    Cancel,
    Complete,
}