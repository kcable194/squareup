//! Response body struct for the inventory event webhook

use crate::models::enums::InventoryWebhookEventType;
use serde::{Deserialize, Serialize};

use super::{DateTime, InventoryCountUpdatedEventData};

/// This is a model struct for InventoryEventWebhookResponse type.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct InventoryEventWebhookResponse {
    /// The ID of the target seller associated with the event.
    pub merchant_id: String,
    /// The type of this event.
    pub r#type: InventoryWebhookEventType,
    /// A unique ID for the event.
    pub event_id: String,
    /// Read only The timestamp of when the event was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: DateTime,
    /// The data associated with the event.
    pub data: InventoryCountUpdatedEventData,
}
