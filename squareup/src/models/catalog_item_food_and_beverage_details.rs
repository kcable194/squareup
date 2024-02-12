//! Model struct for CatalogItemFoodAndBeverageDetails type.

use serde::{Deserialize, Serialize};
use crate::models::catalog_item_food_and_beverage_details_ingredient::CatalogItemFoodAndBeverageDetailsIngredient;
use crate::models::CatalogItemFoodAndBeverageDetailsDietaryPreference;


/// The food and beverage-specific details of a FOOD_AND_BEV item.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogItemFoodAndBeverageDetails {
    /// The calorie count (in the unit of kcal) for the FOOD_AND_BEV type of items.
    pub calorie_count: Option<i32>,
    /// The text of the item's display label in the Square Point of Sale app. Only up to the first
    /// five characters of the string are used. This attribute is searchable, and its value length
    /// is of Unicode code points.
    pub dietary_preferences: Option<Vec<CatalogItemFoodAndBeverageDetailsDietaryPreference>>,
    /// The ingredients for the FOOD_AND_BEV type item.
    pub ingredients: Option<Vec<CatalogItemFoodAndBeverageDetailsIngredient>>
}
