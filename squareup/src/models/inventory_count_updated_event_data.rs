//! Response body struct for the InventoryCountUpdatedEventData type

use crate::models::InventoryCountUpdatedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for InventoryCountUpdatedEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct InventoryCountUpdatedEventData {
    /// The type of the event data object. The value is "inventory_counts". Max Length 50
    pub r#type: String,
    /// The ID of the associated object.
    pub id: String,
    /// An object containing fields and values relevant to the event. Is absent if affected object
    /// was deleted.
    pub object: Option<InventoryCountUpdatedEventObject>,
}
