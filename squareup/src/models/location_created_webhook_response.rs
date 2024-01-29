//! Response body struct for the location.created event webhook

use crate::models::enums::LocationWebhookEventType;
use serde::{Deserialize, Serialize};

use super::{DateTime, LocationCreatedEventData};

/// This is a model struct for LocationCreatedWebhookResponse type.
#[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct LocationCreatedWebhookResponse {
    /// The ID of the target seller associated with the event.
    pub merchant_id: String,
    /// The location id.
    pub location_id: String,
    /// The type of this event. The value is "location.created".
    pub r#type: LocationWebhookEventType,
    /// A unique ID for the event.
    pub event_id: String,
    /// Read only The timestamp of when the event was created, in RFC 3339 format.
    /// Examples for January 25th, 2020 6:25:34pm Pacific Standard Time:
    /// UTC: 2020-01-26T02:25:34Z
    /// Pacific Standard Time with UTC offset: 2020-01-25T18:25:34-08:00
    pub created_at: DateTime,
    /// The data associated with the event.
    pub data: LocationCreatedEventData,
}
