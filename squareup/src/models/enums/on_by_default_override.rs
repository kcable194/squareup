//! Model for OnByDefaultOverride enum.

use serde::{Deserialize, Serialize};

/// If `YES`, this setting overrides the on_by_default setting on the `CatalogModifier` object,
/// and the modifier is always selected by default for the catalog item.
///
/// If `NO`, the modifier is not selected by default for this catalog item. `NOT_SET` means the
/// on_by_default setting on the `CatalogModifier` object is obeyed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OnByDefaultOverride {
    Yes,
    No,
    NotSet,
}
