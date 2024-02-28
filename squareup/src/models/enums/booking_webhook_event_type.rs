//! Enum for BookingWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of booking event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum BookingWebhookEventType {
    #[serde(rename = "booking.created")]
    BookingCreated,
    #[serde(rename = "booking.updated")]
    BookingUpdated,
}

impl Display for BookingWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BookingWebhookEventType::BookingCreated => {
                write!(f, "booking.created")
            }
            BookingWebhookEventType::BookingUpdated => {
                write!(f, "booking.updated")
            }
        }
    }
}
