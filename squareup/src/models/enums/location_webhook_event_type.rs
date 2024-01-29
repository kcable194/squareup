//! Enum for LocationWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of location event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum LocationWebhookEventType {
    #[serde(rename = "location.created")]
    Created,
    #[serde(rename = "location.updated")]
    Updated,
}

impl Display for LocationWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationWebhookEventType::Created => {
                write!(f, "location.created")
            }
            LocationWebhookEventType::Updated => {
                write!(f, "location.updated")
            }
        }
    }
}
