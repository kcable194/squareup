//! Model struct for EventTypeMetadata type

use crate::models::enums::WebhookEventType;
use serde::{Deserialize, Serialize};

/// Contains the metadata of a webhook event type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EventTypeMetadata {
    /// **Read only** The event type.
    pub event_type: Option<WebhookEventType>,
    /// **Read only** The API version at which the event type was introduced.
    pub api_version_introduced: Option<String>,
    /// **Read only** The release status of the event type.
    pub release_status: Option<String>,
}
