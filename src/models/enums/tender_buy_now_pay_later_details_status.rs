//! Model for TenderBuyNowPayLaterDetailsStatus enum.

use serde::{Deserialize, Serialize};

/// Model for TenderBuyNowPayLaterDetailsStatus enum.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TenderBuyNowPayLaterDetailsStatus {
    /// The buy now pay later payment has been authorized but not yet captured.
    Authorized,
    /// The buy now pay later payment was authorized and subsequently captured (i.e., completed).
    Captured,
    /// The buy now pay later payment was authorized and subsequently voided (i.e., canceled).
    Voided,
    /// The buy now pay later payment failed.
    Failed,
}