//! Model struct for CreateCustomerCustomAttributeDefinitionRequest type

use super::CustomAttributeDefinition;
use serde::Serialize;

/// This is a model struct for CreateCustomerCustomAttributeDefinitionRequest type
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct CreateCustomerCustomAttributeDefinitionRequest {
    /// The custom attribute definition to create.
    /// Note: `name` must be unique (case-sensitive) across all visible customer-related custom attribute definitions for the seller.
    pub custom_attribute_definition: CustomAttributeDefinition,
    /// A unique identifier for this request, used to ensure idempotency.
    /// Max length: 45 characters.
    pub idempotency_key: Option<String>,
}
