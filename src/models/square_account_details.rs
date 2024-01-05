//! Model struct for SquareAccountDetails type.

use serde::{Deserialize, Serialize};
use crate::models::errors::Error;

use super::enums::Product;

/// Details about a Square Account payment. The details are only populated if the source_type is SQUARE_ACCOUNT.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SquareAccountDetails {
    /// Unique identifier for the payment source used for this payment.
    /// Max length 255
    pub payment_source_token: Option<String>,
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}