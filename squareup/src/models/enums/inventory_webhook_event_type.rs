//! Enum for InventoryWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of inventory event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InventoryWebhookEventType {
    #[serde(rename = "inventory.count.updated")]
    InventoryCountUpdated,
}

impl Display for InventoryWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryWebhookEventType::InventoryCountUpdated => {
                write!(f, "inventory.count.updated")
            }
        }
    }
}
