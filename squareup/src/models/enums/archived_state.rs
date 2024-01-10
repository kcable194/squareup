//! Model for ArchivedState enum.

use serde::{Deserialize, Serialize};

/// An enum type of catalog item archived state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArchivedState {
    /// Requested items are not archived with the is_archived attribute set to false.
    ArchivedStateNotArchived,
    /// Requested items are archived with the is_archived attribute set to true.
    ArchivedStateArchived,
    /// Requested items can be archived or not archived.
    ArchivedStateAll,
}
