//! Model struct for BulkUpsertCustomerCustomAttributesRequest type

use std::collections::HashMap;

use super::BulkUpsertCustomerCustomAttributesRequestCustomerCustomAttributeUpsertRequest;
use serde::{Deserialize, Serialize};

/// Represents a request for the BulkUpsertCustomerCustomAttributes endpoint.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct BulkUpsertCustomerCustomAttributesRequest {
    /// A list of 1 to 25 individual upsert requests.
    pub values: HashMap<String, BulkUpsertCustomerCustomAttributesRequestCustomerCustomAttributeUpsertRequest>,
}
