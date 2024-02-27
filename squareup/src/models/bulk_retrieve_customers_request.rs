//! Model struct for BulkRetrieveCustomersRequest type

use serde::Serialize;

/// This is a model struct for BulkRetrieveCustomersRequest type
#[derive(Clone, Debug, Default, Serialize, Eq, PartialEq)]
pub struct BulkRetrieveCustomersRequest {
    /// The IDs of the customer profiles to retrieve.
    /// Min Length 1, Max Length 100
    pub customer_ids: Vec<String>,
}
