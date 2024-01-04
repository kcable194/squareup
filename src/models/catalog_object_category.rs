//! Model struct for CatalogObjectCategory type.

use serde::{Deserialize, Serialize};

/// A CatalogObjectCategory describing the category
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogObjectCategory {
    /// The ID of the object's category.
    pub id: Option<String>,
    /// The order of the object within the context of the category.
    pub ordinal: Option<i64>,
}