//! Model struct for CatalogCategory type.

use crate::models::enums::CatalogCategoryType;
use crate::models::{CatalogEcomSeoData, CatalogObjectCategory, CategoryPathToRootNode};
use serde::{Deserialize, Serialize};

/// A category to which a `CatalogItem` instance belongs.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogCategory {
    /// The category name. This is a searchable attribute for use in applicable query filters, and
    /// its value length is of Unicode code points.
    pub name: Option<String>,
    /// The IDs of images associated with this `CatalogCategory` instance. Currently these images
    /// are not displayed by Square, but are free to be displayed in 3rd party applications.
    pub image_ids: Option<Vec<String>>,
    /// The type of the category.
    pub category_type: Option<CatalogCategoryType>,
    /// The ID of the parent category of this category instance.
    pub parent_category: Option<CatalogObjectCategory>,
    /// Indicates whether a category is a top level category, which does not have any parent_category.
    pub is_top_level: Option<bool>,
    /// A list of IDs representing channels, such as a Square Online site, where the category can be
    /// made visible.
    pub channels: Option<Vec<String>>,
    /// The IDs of the CatalogAvailabilityPeriod objects associated with the category.
    pub availability_period_ids: Option<Vec<String>>,
    /// Indicates whether the category is visible (true) or hidden (false) on all of the seller's
    /// Square Online sites.
    pub online_visibility: Option<bool>,
    /// **Read only** The top-level category in a category hierarchy.
    pub root_category: Option<String>,
    /// The SEO data for a seller's Square Online store.
    pub ecom_seo_data: Option<CatalogEcomSeoData>,
    /// The path from the category to its root category. The first node of the path is the parent of the
    /// category and the last is the root category. The path is empty if the category is a root category.
    pub path_to_root: Option<Vec<CategoryPathToRootNode>>,
}
