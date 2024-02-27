//! Model struct for BulkCreateCustomersRequest type

use crate::models::BulkCreateCustomerData;
use serde::Serialize;
use std::collections::HashMap;

/// This is a model struct for BulkCreateCustomersRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkCreateCustomersRequest {
    /// A map of 1 to 100 individual create requests, represented by idempotency key: { customer data } key-value pairs.
    ///
    /// Each key is an idempotency key that uniquely identifies the create request. Each value
    /// contains the customer data used to create the customer profile.
    pub customers: HashMap<String, BulkCreateCustomerData>,
}
