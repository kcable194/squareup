//! Model struct for BulkUpdateCustomersRequest type

use crate::models::BulkUpdateCustomerData;
use serde::Serialize;
use std::collections::HashMap;

/// This is a model struct for BulkUpdateCustomersRequest type
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct BulkUpdateCustomersRequest {
    /// A map of 1 to 100 individual update requests, represented by customer ID: { customer data }
    /// key-value pairs.
    ///
    /// Each key is the ID of the customer profile to update. To update a customer profile that was
    /// created by merging existing profiles, provide the ID of the newly created profile.
    ///
    /// Each value contains the updated customer data. Only new or changed fields are required.
    /// To add or update a field, specify the new value. To remove a field, specify null.
    pub customers: HashMap<String, BulkUpdateCustomerData>,
}
