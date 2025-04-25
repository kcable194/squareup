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
    pub selection_type: Option<CatalogModifierListSelectionType>,
    /// The options included in the `CatalogModifierList`. You must include at least one
    /// `CatalogModifier`. Each CatalogObject must have type `MODIFIER` and contain
    /// `CatalogModifier` data.
    pub modifiers: Option<Vec<CatalogObject>>,
    /// The IDs of images associated with this `CatalogModifierList` instance. Currently these
    /// images are not displayed by Square, but are free to be displayed in 3rd party applications.
    pub image_ids: Option<Vec<String>>,
    /// The type of the modifier.
    ///
    /// When this modifier_type value is TEXT, the CatalogModifierList represents a text-based
    /// modifier. When this modifier_type value is LIST, the CatalogModifierList contains a list
    /// of CatalogModifier objects.
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
}
