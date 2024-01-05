//! Model for TenderSquareAccountDetailsStatus enum.

use serde::{Deserialize, Serialize};

/// Model for TenderSquareAccountDetailsStatus enum.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderSquareAccountDetailsStatus {
    /// The Square Account payment has been authorized but not yet captured.
    Authorized,
    /// The Square Account payment was authorized and subsequently captured (i.e., completed).
    Captured,
    /// The Square Account payment was authorized and subsequently voided (i.e., canceled).
    Voided,
    /// The Square Account payment failed.
    Failed,
}