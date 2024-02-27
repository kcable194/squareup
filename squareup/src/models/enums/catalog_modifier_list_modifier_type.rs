//! Model for CatalogModifierListModifierType enum.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogModifierListModifierType {
    /// The `CatalogModifierList` instance is a non-empty list of non text-based modifiers.
    List,
    /// The `CatalogModifierList` instance is a single text-based modifier.
    Text,
}
