//! Model for IsConversational enum.

use serde::{Deserialize, Serialize};

/// Controls whether conversational mode is enabled for modifiers on an item.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IsConversational {
    Yes,
    No,
    NotSet,
}
