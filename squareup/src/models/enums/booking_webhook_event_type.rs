//! Enum for BookingWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of booking event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BookingWebhookEventType {
    #[serde(rename = "booking.created")]
    Created,
    #[serde(rename = "booking.updated")]
    Updated,
}

impl Display for BookingWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingWebhookEventType::Created => {
                write!(f, "booking.created")
            }
            BookingWebhookEventType::Updated => {
                write!(f, "booking.updated")
            }
        }
    }
}
