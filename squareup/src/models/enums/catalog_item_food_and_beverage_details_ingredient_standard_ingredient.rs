//! Model for CatalogItemFoodAndBeverageDetailsIngredientStandardIngredient enum.

use serde::{Deserialize, Serialize};

/// The name of the ingredient from a standard pre-defined list. This should be null if it's a
/// custom dietary preference.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogItemFoodAndBeverageDetailsIngredientStandardIngredient {
    Celery,
    Crustaceans,
    Eggs,
    Fish,
    Gluten,
    Lupin,
    Milk,
    Molluscs,
    Mustard,
    Peanuts,
    Sesame,
    Soy,
    Sulphites,
    TreeNuts,
}
