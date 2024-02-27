//! Response body struct for the catalog version updated event webhook

use crate::models::enums::CatalogWebhookEventType;
use serde::{Deserialize, Serialize};

use super::{CatalogEventData, DateTime};

/// This is a model struct for CatalogVersionUpdatedWebhookResponse type.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct CatalogVersionUpdatedWebhookResponse {
    /// The ID of the target seller associated with the event.
    pub merchant_id: String,
    /// The type of this event.
    pub r#type: CatalogWebhookEventType,
    /// A unique ID for the event.
    pub event_id: String,
    /// Read only The timestamp of when the event was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: DateTime,
    /// The data associated with the event.
    pub data: CatalogEventData,
}
