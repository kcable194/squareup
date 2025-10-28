//! Model struct for CatalogModifierOverride type.

use serde::{Deserialize, Serialize};
use crate::models::enums::{HiddenOnlineOverride, OnByDefaultOverride};

/// Options to control how to override the default behavior of the specified modifier.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogModifierOverride {
    /// The ID of the `CatalogModifier` whose default behavior is being overridden.
    pub modifier_id: String,
    /// If `true`, this `CatalogModifier` should be selected by default for this `CatalogItem`.
    #[deprecated(note = "Use `on_by_default_override` instead.")]
    pub on_by_default: Option<bool>,
    /// If `YES`, this setting overrides the hidden_online setting on the `CatalogModifier` object,
    /// and the modifier is always hidden from online sales channels. If `NO`, the modifier is not
    /// hidden. It is always visible in online sales channels for this catalog item. `NOT_SET` means
    /// the hidden_online setting on the `CatalogModifier` object is obeyed.
    pub hidden_online_override: Option<HiddenOnlineOverride>,
    /// If `YES`, this setting overrides the on_by_default setting on the `CatalogModifier` object,
    /// and the modifier is always selected by default for the catalog item.
    ///
    /// If `NO`, the modifier is not selected by default for this catalog item. `NOT_SET` means the
    /// on_by_default setting on the `CatalogModifier` object is obeyed.
    pub on_by_default_override: Option<OnByDefaultOverride>,
}
