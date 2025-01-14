//! Model struct for BulkUpsertCustomerCustomAttributesResponseCustomerCustomAttributeUpsertResponse type

use super::{errors::Error, CustomAttribute};
use serde::{Deserialize, Serialize};

/// Represents a response for an individual upsert request in a BulkUpsertCustomerCustomAttributes operation.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct BulkUpsertCustomerCustomAttributesResponseCustomerCustomAttributeUpsertResponse {
    /// The ID of the customer profile associated with the custom attribute.
    pub customer_id: Option<String>,
    /// The new or updated custom attribute.
    pub custom_attribute: Option<CustomAttribute>,
    /// Any errors that occurred while processing the individual request.
    pub errors: Option<Vec<Error>>,
}
