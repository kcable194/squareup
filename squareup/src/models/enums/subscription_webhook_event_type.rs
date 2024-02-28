//! Enum for SubscriptionWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of subscription event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum SubscriptionWebhookEventType {
    #[serde(rename = "subscription.created")]
    SubscriptionCreated,
    #[serde(rename = "subscription.updated")]
    SubscriptionUpdated,
}

impl Display for SubscriptionWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionWebhookEventType::SubscriptionCreated => {
                write!(f, "subscription.created")
            }
            SubscriptionWebhookEventType::SubscriptionUpdated => {
                write!(f, "subscription.updated")
            }
        }
    }
}
