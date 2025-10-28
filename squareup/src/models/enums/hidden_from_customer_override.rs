//! Model for HiddenFromCustomerOverride enum.

use serde::{Deserialize, Serialize};

/// Controls whether all modifiers for an item are hidden from customer receipts.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HiddenFromCustomerOverride {
    Yes,
    No,
    NotSet,
}
