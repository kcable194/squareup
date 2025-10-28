//! Model for AllowQuantities enum.

use serde::{Deserialize, Serialize};

/// Controls whether multiple quantities of the same modifier can be selected for an item.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AllowQuantities {
    Yes,
    No,
    NotSet,
}
