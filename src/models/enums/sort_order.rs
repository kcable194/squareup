//! Model for SortOrder enum

use std::fmt::Display;
use serde::{Deserialize, Serialize};

/// The order (e.g., chronological or alphabetical) in which results from a request are returned.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortOrder {
    /// The results are returned in descending (e.g., newest-first or Z-A) order.
    Desc,
    /// The results are returned in ascending (e.g., oldest-first or A-Z) order.
    Asc,
}

impl From<SortOrder> for String {
    fn from(sort_order: SortOrder) -> Self {
        sort_order.to_string()
    }
}

impl Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            SortOrder::Desc => String::from("DESC"),
            SortOrder::Asc => String::from("ASC"),
        };
        write!(f, "{}", str)
    }
}
