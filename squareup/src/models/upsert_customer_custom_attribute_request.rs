//! Model struct for UpsertCustomerCustomAttributeRequest type

use super::CustomAttribute;
use serde::Serialize;

/// Represents the request body for the UpsertCustomerCustomAttribute endpoint.
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct UpsertCustomerCustomAttributeRequest {
    /// The custom attribute to create or update.
    /// - `value`: Must conform to the schema specified by the definition.
    /// - `version`: Optional, used for optimistic concurrency control.
    pub custom_attribute: CustomAttribute,
    /// A unique identifier for this request, used to ensure idempotency.
    /// Max length: 45 characters.
    pub idempotency_key: Option<String>,
}
