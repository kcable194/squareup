//! Response body struct for the CatalogEventData type

use serde::{Deserialize, Serialize};
use crate::models::CatalogVersionUpdatedEventObject;

/// This is a model struct for CatalogEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CatalogEventData {
    /// The type of the event data object.
    pub r#type: String,
    /// An object containing fields and values relevant to the event. Is absent if affected
    /// object was deleted.
    pub object: CatalogVersionUpdatedEventObject,
}
