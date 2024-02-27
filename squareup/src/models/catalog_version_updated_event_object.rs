//! Response body struct for the CatalogVersionUpdatedEventObject type

use serde::{Deserialize, Serialize};
use crate::models::CatalogVersionUpdatedEventCatalogVersion;

/// This is a model struct for CatalogVersionUpdatedEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CatalogVersionUpdatedEventObject {
    /// The version of the object.
    pub catalog_version: CatalogVersionUpdatedEventCatalogVersion,
}
