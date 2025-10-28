//! Model struct for CatalogModifier type.

use serde::{Deserialize, Serialize};

use super::{ModifierLocationOverrides, Money};

/// A modifier applicable to items at the time of sale.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogModifier {
    /// The modifier name. This is a searchable attribute for use in applicable query filters, and
    /// its value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// The modifier price.
    pub price_money: Option<Money>,
    /// When true, this modifier is selected by default when displaying the modifier list. This
    /// setting can be overridden at the item level using CatalogModifierListInfo.modifier_overrides.
    pub on_by_default: Option<bool>,
    /// Determines where this `CatalogModifier` appears in the `CatalogModifierList`.
    pub ordinal: Option<i32>,
    /// The ID of the `CatalogModifierList` associated with this modifier.
    pub modifier_list_id: Option<String>,
    /// Location-specific price overrides.
    pub location_overrides: Option<ModifierLocationOverrides>,
    /// The ID of the image associated with this `CatalogModifier` instance. Currently, this image is
    /// not displayed by Square, but is free to be displayed in 3rd party applications.
    pub image_id: Option<String>,
    /// When true, this modifier is hidden from online ordering channels. This setting can be
    /// overridden at the item level using CatalogModifierListInfo.modifier_overrides.
    pub hidden_online: Option<bool>,
}
