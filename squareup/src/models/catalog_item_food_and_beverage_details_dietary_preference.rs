//! Model struct for CatalogItemFoodAndBeverageDetailsDietaryPreference type.

use serde::{Deserialize, Serialize};
use crate::models::enums::{CatalogItemFoodAndBeverageDetailsDietaryPreferenceStandardDietaryPreference, CatalogItemFoodAndBeverageDetailsDietaryPreferenceType};


/// Dietary preferences that can be assigned to an FOOD_AND_BEV item and its ingredients.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemFoodAndBeverageDetailsDietaryPreference {
    /// The dietary preference type. Supported values include STANDARD and CUSTOM as specified in
    /// FoodAndBeverageDetails.DietaryPreferenceType.
    pub r#type: Option<CatalogItemFoodAndBeverageDetailsDietaryPreferenceType>,
    /// The name of the dietary preference from a standard pre-defined list. This should be null
    /// if it's a custom dietary preference.
    pub standard_name: Option<CatalogItemFoodAndBeverageDetailsDietaryPreferenceStandardDietaryPreference>,
    /// The name of a user-defined custom dietary preference. This should be null if it's a
    /// standard dietary preference.
    pub custom_name: Option<String>
}
