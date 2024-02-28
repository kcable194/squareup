//! Enum for InvoiceWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of invoice event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum InvoiceWebhookEventType {
    #[serde(rename = "invoice.canceled")]
    InvoiceCancelled,
    #[serde(rename = "invoice.created")]
    InvoiceCreated,
    #[serde(rename = "invoice.deleted")]
    InvoiceDeleted,
    #[serde(rename = "invoice.payment_made")]
    InvoicePaymentMade,
    #[serde(rename = "invoice.published")]
    InvoicePublished,
    #[serde(rename = "invoice.refunded")]
    InvoiceRefunded,
    #[serde(rename = "invoice.scheduled_charge_failed")]
    InvoiceScheduledChargeFailed,
    #[serde(rename = "invoice.updated")]
    InvoiceUpdated,
}

impl Display for InvoiceWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InvoiceWebhookEventType::InvoiceCancelled => {
                write!(f, "invoice.canceled")
            }
            InvoiceWebhookEventType::InvoiceCreated => {
                write!(f, "invoice.created")
            }
            InvoiceWebhookEventType::InvoiceDeleted => {
                write!(f, "invoice.deleted")
            }
            InvoiceWebhookEventType::InvoicePaymentMade => {
                write!(f, "invoice.payment_made")
            }
            InvoiceWebhookEventType::InvoicePublished => {
                write!(f, "invoice.published")
            }
            InvoiceWebhookEventType::InvoiceRefunded => {
                write!(f, "invoice.refunded")
            }
            InvoiceWebhookEventType::InvoiceScheduledChargeFailed => {
                write!(f, "invoice.scheduled_charge_failed")
            }
            InvoiceWebhookEventType::InvoiceUpdated => {
                write!(f, "invoice.updated")
            }
        }
    }
}
