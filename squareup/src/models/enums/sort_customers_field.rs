//! Model for SortCustomersField enum

use serde::Serialize;
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]

/// Use one or more customer attributes as the sort key to sort searched customer profiles.
/// For example, use the creation date (created_at) of customers or default attributes as the sort key.
pub enum SortCustomersField {
    /// By default, customers are sorted alphanumerically by concatenating their given_name and family_name.
    #[default]
    Default,
    /// Use the creation date attribute (created_at) of customer profiles as the sort key.
    CreatedAt,
}

impl From<SortCustomersField> for String {
    fn from(sort_field: SortCustomersField) -> Self {
        sort_field.to_string()
    }
}

impl Display for SortCustomersField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SortCustomersField::Default => String::from("DEFAULT"),
            SortCustomersField::CreatedAt => String::from("CREATED_AT"),
        };
        write!(f, "{}", str)
    }
}
