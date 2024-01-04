//! Model struct for CatalogEcomSeoData type.

use serde::{Deserialize, Serialize};

/// The SEO data for a seller's Square Online store.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CatalogEcomSeoData {
    /// The SEO title used for the Square Online store.
    pub page_title: Option<String>,
    /// The SEO description used for the Square Online store.
    pub page_description: Option<String>,
    /// The SEO permalink used for the Square Online store.
    pub permalink: Option<String>,
}