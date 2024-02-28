//! Response body struct for the InventoryCountUpdatedEventObject type

use crate::models::InventoryCount;
use serde::{Deserialize, Serialize};

/// This is a model struct for InventoryCountUpdatedEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct InventoryCountUpdatedEventObject {
    /// The inventory counts.
    pub inventory_counts: Vec<InventoryCount>,
}
