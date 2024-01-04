//! Model struct for ModifierLocationOverrides type.

use serde::{Deserialize, Serialize};
use super::Money;

/// Location-specific overrides for specified properties of a `CatalogModifier` object.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ModifierLocationOverrides {
    /// The ID of the Location object representing the location. This can include a
    /// deactivated location.
    pub location_id: Option<String>,
    /// The overridden price at the specified location. If this is unspecified, the modifier
    /// price is not overridden. The modifier becomes free of charge at the specified location,
    /// when this price_money field is set to 0.
    pub price_money: Option<Money>,
    /// **Read only** Indicates whether the modifier is sold out at the specified location
    /// or not. As an example, for cheese (modifier) burger (item), when the modifier is
    /// sold out, it is the cheese, but not the burger, that is sold out. The seller can
    /// manually set this sold out status. Attempts by an application to set this attribute
    /// are ignored.
    pub sold_out: Option<bool>,
}