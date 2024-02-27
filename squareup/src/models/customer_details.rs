//! Model struct for CustomerDetails type.

use serde::{Deserialize, Serialize};

/// Details about a Square Account payment. The details are only populated if the source_type is SquareAccount.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomerDetails {
    /// Indicates whether the customer initiated the payment.
    pub customer_initiated: Option<bool>,
    /// Indicates that the seller keyed in payment details on behalf of the customer.
    /// This is used to flag a payment as Mail Order / Telephone Order (MOTO).
    pub seller_keyed_in: Option<bool>,
}
