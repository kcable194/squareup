//! Model struct for CatalogItemModifierListInfo type.

use serde::{Deserialize, Serialize};
use crate::models::enums::{AllowQuantities, HiddenFromCustomerOverride, IsConversational};
use super::CatalogModifierOverride;

/// Options to control the properties of a `CatalogModifierList` applied to a `CatalogItem`
/// instance.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemModifierListInfo {
    /// The ID of the `CatalogModifierList` controlled by this `CatalogModifierListInfo`.
    /// Min Length 1
    pub modifier_list_id: String,
    /// A set of `CatalogModifierOverride` objects that override default modifier settings for this
    /// item.
    pub modifier_overrides: Option<Vec<CatalogModifierOverride>>,
    /// The minimum number of modifiers that must be selected from this modifier list. Values:
    ///     0: No selection is required.
    ///     -1: Default value, the attribute was not set by the client. When max_selected_modifiers is also -1, use the minimum and maximum selection values set on the CatalogItemModifierList.
    ///     >0: The required minimum modifier selections. This can be larger than the total CatalogModifiers when allow_quantities is enabled.
    ///     < -1: Invalid. Treated as no selection required.
    pub min_selected_modifiers: Option<i32>,
    /// The maximum number of modifiers that can be selected. Values:
    ///     0: No maximum limit.
    ///     -1: Default value, the attribute was not set by the client. When min_selected_modifiers is also -1, use the minimum and maximum selection values set on the CatalogItemModifierList.
    ///     >0: The maximum total modifier selections. This can be larger than the total CatalogModifiers when allow_quantities is enabled.
    ///     < -1: Invalid. Treated as no maximum limit.
    pub max_selected_modifiers: Option<i32>,
    /// If `true`, enable this `CatalogModifierList`. The default value is `true`.
    pub enabled: Option<bool>,
    /// The position of this `CatalogItemModifierListInfo` object within the modifier_list_info
    /// list applied to a `CatalogItem` instance.
    pub ordinal: Option<i32>,
    /// Controls whether multiple quantities of the same modifier can be selected for this item.
    ///     `YES` means that every modifier in the `CatalogModifierList` can have multiple quantities selected for this item.
    ///     `NO` means that each modifier in the `CatalogModifierList` can be selected only once for this item.
    ///     `NOT_SET` means that the allow_quantities setting on the `CatalogModifierList` is obeyed.
    pub allow_quantities: Option<AllowQuantities>,
    /// Controls whether conversational mode is enabled for modifiers on this item.
    ///     `YES` means conversational mode is enabled for every modifier in the `CatalogModifierList`.
    ///     `NO` means that conversational mode is not enabled for any modifier in the `CatalogModifierList`.
    ///     `NOT_SET` means that conversational mode is not enabled for any modifier in the `CatalogModifierList`.
    pub is_conversational: Option<IsConversational>,
    /// Controls whether all modifiers for this item are hidden from customer receipts.
    ///     `YES` means that all modifiers in the `CatalogModifierList` are hidden from customer receipts for this item.
    ///     `NO` means that all modifiers in the `CatalogModifierList` are visible on customer receipts for this item.
    ///     `NOT_SET` means that the hidden_from_customer setting on the `CatalogModifierList` is obeyed.
    pub hidden_from_customer_override: Option<HiddenFromCustomerOverride>,
}
