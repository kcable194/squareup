//! Model for CatalogCategoryType enum

use serde::{Deserialize, Serialize};

/// Indicates the type of catalog category.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CatalogCategoryType {
    /// The regular category.
    RegularCategory,
    /// The menu category.
    MenuCategory,
    /// Kitchen categories are used by KDS (Kitchen Display System) to route items to specific clients
    KitchenCategory,
}
