//! Model struct for TenderSquareAccountDetails type

use crate::models::enums::TenderSquareAccountDetailsStatus;
use serde::{Deserialize, Serialize};

/// Represents the details of a tender with type SQUARE_ACCOUNT.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct TenderSquareAccountDetails {
    /// The bank account payment's current state.
    pub status: Option<TenderSquareAccountDetailsStatus>,
}
