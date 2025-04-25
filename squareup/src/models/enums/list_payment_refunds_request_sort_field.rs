//! Model for ListPaymentRefundsRequestSortField enum

use serde::Serialize;
use std::fmt::Display;

/// Use payment attribute as the sort key to sort.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ListPaymentRefundsRequestSortField {
    /// Use the creation date attribute (created_at) of payments as the sort key.
    #[default]
    CreatedAt,
    /// Use the updated at date attribute (updated_at) of payments as the sort key.
    UpdatedAt,
}

impl From<ListPaymentRefundsRequestSortField> for String {
    fn from(sort_field: ListPaymentRefundsRequestSortField) -> Self {
        sort_field.to_string()
    }
}

impl Display for ListPaymentRefundsRequestSortField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ListPaymentRefundsRequestSortField::CreatedAt => String::from("CreatedAt"),
            ListPaymentRefundsRequestSortField::UpdatedAt => String::from("UpdatedAt"),

        };
        write!(f, "{}", str)
    }
}
