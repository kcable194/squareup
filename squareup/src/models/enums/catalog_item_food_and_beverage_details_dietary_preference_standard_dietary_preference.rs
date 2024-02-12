//! Model for CatalogItemFoodAndBeverageDetailsDietaryPreferenceStandardDietaryPreference enum.

use serde::{Deserialize, Serialize};

/// The name of the dietary preference from a standard pre-defined list. This should be null if
/// it's a custom dietary preference.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogItemFoodAndBeverageDetailsDietaryPreferenceStandardDietaryPreference {
    DairyFree,
    GlutenFree,
    Halal,
    Kosher,
    NutFree,
    Vegan,
    Vegetarian,
}
