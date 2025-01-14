//! Model struct for BulkUpsertCustomerCustomAttributesRequestCustomerCustomAttributeUpsertRequest type

use super::CustomAttribute;
use serde::{Deserialize, Serialize};

/// Represents an individual upsert request in a BulkUpsertCustomerCustomAttributes request.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct BulkUpsertCustomerCustomAttributesRequestCustomerCustomAttributeUpsertRequest {
    /// The ID of the target customer profile.
    pub customer_id: String,
    /// The custom attribute to create or update.
    pub custom_attribute: CustomAttribute,
    /// A unique identifier for this individual upsert request, used to ensure idempotency.
    /// Max length: 45 characters.
    pub idempotency_key: Option<String>,
}
