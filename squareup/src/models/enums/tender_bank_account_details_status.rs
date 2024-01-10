//! Model for TenderBankAccountDetailsStatus enum.

use serde::{Deserialize, Serialize};

/// The bank account payment's current state.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderBankAccountDetailsStatus {
    /// The bank account payment is in progress.
    Pending,
    /// The bank account payment has been completed.
    Completed,
    /// The bank account payment failed.
    Failed,
}
