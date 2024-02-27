//! Model struct for BulkDeleteCustomersRequest type

use serde::Serialize;

/// This is a model struct for BulkDeleteCustomersRequest type
#[derive(Clone, Debug, Default, Serialize, Eq, PartialEq)]
pub struct BulkDeleteCustomersRequest {
    /// The IDs of the customer profiles
    /// Min Length 1, Max Length 100
    pub customer_ids: Vec<String>,
}
