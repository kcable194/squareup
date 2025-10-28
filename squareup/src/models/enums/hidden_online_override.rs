//! Model for HiddenOnlineOverride enum.

use serde::{Deserialize, Serialize};

/// If `YES`, this setting overrides the hidden_online setting on the `CatalogModifier` object,
/// and the modifier is always hidden from online sales channels. If `NO`, the modifier is not
/// hidden. It is always visible in online sales channels for this catalog item. `NOT_SET` means
/// the hidden_online setting on the `CatalogModifier` object is obeyed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HiddenOnlineOverride {
    Yes,
    No,
    NotSet,
}
