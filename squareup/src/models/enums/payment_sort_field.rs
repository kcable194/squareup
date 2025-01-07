//! Model for PaymentSortField enum

use std::fmt::{write, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::models::enums::PaymentEventPaymentEventType;

/// Indicates a payment's current status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentSortField {
    CreatedAt,
    OfflineCreatedAt,
    UpdatedAt,
}

impl Default for PaymentSortField {
    fn default() -> Self {
        PaymentSortField::CreatedAt
    }
}

impl Display for PaymentSortField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentSortField::CreatedAt => { write!(f, "CREATED_AT") }
            PaymentSortField::OfflineCreatedAt => { write!(f, "OFFLINE_CREATED_AT") }
            PaymentSortField::UpdatedAt => { write!(f, "UPDATED_AT") }
        }
    }
}