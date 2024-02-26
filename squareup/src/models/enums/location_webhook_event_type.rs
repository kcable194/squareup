//! Enum for LocationWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of location event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LocationWebhookEventType {
    #[serde(rename = "location.created")]
    LocationCreated,
    #[serde(rename = "location.updated")]
    LocationUpdated,
}

impl Display for LocationWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationWebhookEventType::LocationCreated => {
                write!(f, "location.created")
            }
            LocationWebhookEventType::LocationUpdated => {
                write!(f, "location.updated")
            }
        }
    }
}
