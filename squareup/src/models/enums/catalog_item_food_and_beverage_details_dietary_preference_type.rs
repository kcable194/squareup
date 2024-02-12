//! Model for CatalogItemFoodAndBeverageDetailsDietaryPreferenceType enum.

use serde::{Deserialize, Serialize};

/// The dietary preference type. Supported values include STANDARD and CUSTOM as specified in
/// FoodAndBeverageDetails.DietaryPreferenceType.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogItemFoodAndBeverageDetailsDietaryPreferenceType {
    /// A standard value from a pre-determined list.
    Standard,
    /// A user-defined custom value.
    Custom,
}
