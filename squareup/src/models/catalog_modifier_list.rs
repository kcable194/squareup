//! Model struct for CatalogModifierList type.

use crate::models::enums::CatalogModifierListModifierType;
use serde::{Deserialize, Serialize};

use super::{CatalogObject, enums::CatalogModifierListSelectionType};

///  list of modifiers applicable to items at the time of sale.
///
/// For example, a "Condiments" modifier list applicable to a "Hot Dog" item may contain "Ketchup",
/// "Mustard", and "Relish" modifiers. Use the `selection_type` field to specify whether or not
/// multiple selections from the modifier list are allowed.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogModifierList {
    /// The name for the `CatalogModifierList` instance. This is a searchable attribute for use in
    /// applicable query filters, and its value length is of Unicode code points.
    ///
    /// Max Length 255
    pub name: Option<String>,
    /// Determines where this modifier list appears in a list of `CatalogModifierList` values.
    pub ordinal: Option<i32>,
    /// Indicates whether multiple options from the modifier list can be applied to a single
    /// `CatalogItem`.
    #[deprecated]
    pub selection_type: Option<CatalogModifierListSelectionType>,
    /// A non-empty list of `CatalogModifier` objects to be included in the `CatalogModifierList`,
    /// for non text-based modifiers when the modifier_type attribute is LIST. Each element of this list
    /// is a `CatalogObject` instance of the MODIFIER type, containing the following attributes:
    /// {
    /// "id": "{{catalog_modifier_id}}",
    /// "type": "MODIFIER",
    /// "modifier_data": {{a CatalogModifier instance>}}
    /// }
    pub modifiers: Option<Vec<CatalogObject>>,
    /// The IDs of images associated with this `CatalogModifierList` instance. Currently these
    /// images are not displayed by Square, but are free to be displayed in 3rd party applications.
    pub image_ids: Option<Vec<String>>,
    /// When true, allows multiple quantities of the same modifier to be selected.
    pub allow_quantities: Option<bool>,
    /// True if modifiers belonging to this list can be used conversationally.
    pub is_conversational: Option<bool>,
    /// The type of the modifier.
    ///
    /// When this modifier_type value is TEXT, the `CatalogModifierList` represents a text-based
    /// modifier. When this modifier_type value is LIST, the `CatalogModifierList` contains a list
    /// of `CatalogModifier` objects.
    pub modifier_type: Option<CatalogModifierListModifierType>,
    /// The maximum length, in Unicode points, of the text string of the text-based modifier as
    /// represented by this CatalogModifierList object with the modifier_type set to TEXT.
    pub max_length: Option<i32>,
    /// Whether the text string must be a non-empty string (true) or not (false) for a text-based
    /// modifier as represented by this CatalogModifierList object with the modifier_type set to
    /// TEXT.
    pub text_required: Option<bool>,
    /// A note for internal use by the business.
    ///
    /// For example, for a text-based modifier applied to a T-shirt item, if the buyer-supplied
    /// text of "Hello, Kitty!" is to be printed on the T-shirt, this internal_name attribute can
    /// be "Use italic face" as an instruction for the business to follow.
    ///
    /// For non text-based modifiers, this internal_name attribute can be used to include SKUs,
    /// internal codes, or supplemental descriptions for internal use.
    /// Max Length 512
    pub internal_name: Option<String>,
    /// The minimum number of modifiers that must be selected from this list. The value can be
    /// overridden with `CatalogItemModifierListInfo`.
    ///
    /// Values:
    ///     0: No selection is required.
    ///     -1: Default value, the attribute was not set by the client. Treated as no selection required.
    ///     >0: The required minimum modifier selections. This can be larger than the total CatalogModifiers when allow_quantities is enabled.
    ///     < -1: Invalid. Treated as no selection required.
    pub min_selected_modifiers: Option<i64>,
    /// The maximum number of modifiers that must be selected from this list. The value can be
    /// overridden with `CatalogItemModifierListInfo`.
    ///
    /// Values:
    ///     0: No maximum limit.
    ///     -1: Default value, the attribute was not set by the client. Treated as no maximum limit.
    ///     >0: The maximum total modifier selections. This can be larger than the total CatalogModifiers when allow_quantities is enabled.
    ///     < -1: Invalid. Treated as no maximum limit.
    pub max_selected_modifiers: Option<i64>,
    /// If true, modifiers from this list are hidden from customer receipts. The default value is
    /// false. This setting can be overridden with CatalogItemModifierListInfo.hidden_from_customer_override.
    pub hidden_from_customer: Option<bool>,
}
