//! Model struct for CatalogItemFoodAndBeverageDetailsIngredient type.

use serde::{Deserialize, Serialize};
use crate::models::enums::{CatalogItemFoodAndBeverageDetailsDietaryPreferenceStandardDietaryPreference, CatalogItemFoodAndBeverageDetailsDietaryPreferenceType};


/// Describes the ingredient used in a FOOD_AND_BEV item.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemFoodAndBeverageDetailsIngredient {
    /// The dietary preference type of the ingredient. Supported values include STANDARD and
    /// CUSTOM as specified in FoodAndBeverageDetails.DietaryPreferenceType.
    pub r#type: Option<CatalogItemFoodAndBeverageDetailsDietaryPreferenceType>,
    /// The name of the ingredient from a standard pre-defined list. This should be null if it's
    /// a custom dietary preference.
    pub standard_name: Option<CatalogItemFoodAndBeverageDetailsIngredientStandardIngredient>,
    /// The name of a custom user-defined ingredient. This should be null if it's a standard
    /// dietary preference.
    pub custom_name: Option<String>
}
