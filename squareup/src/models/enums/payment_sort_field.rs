//! Model for PaymentSortField enum

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Indicates a payment's current status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentSortField {
    #[default]
    CreatedAt,
    OfflineCreatedAt,
    UpdatedAt,
}

impl Display for PaymentSortField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PaymentSortField::CreatedAt => {
                write!(f, "CREATED_AT")
            }
            PaymentSortField::OfflineCreatedAt => {
                write!(f, "OFFLINE_CREATED_AT")
            }
            PaymentSortField::UpdatedAt => {
                write!(f, "UPDATED_AT")
            }
        }
    }
}
