//! Enum for CustomerWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of customer event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CustomerWebhookEventType {
    #[serde(rename = "customer.created")]
    CustomerCreated,
    #[serde(rename = "customer.deleted")]
    CustomerDeleted,
    #[serde(rename = "customer.updated")]
    CustomerUpdated,
}

impl Display for CustomerWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerWebhookEventType::CustomerCreated => {
                write!(f, "customer.created")
            }
            CustomerWebhookEventType::CustomerDeleted => {
                write!(f, "customer.deleted")
            }
            CustomerWebhookEventType::CustomerUpdated => {
                write!(f, "customer.updated")
            }
        }
    }
}
