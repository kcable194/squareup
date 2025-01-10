//! Model struct for UpdateCustomerCustomAttributeDefinitionRequest type

use super::CustomAttributeDefinition;
use serde::Serialize;

/// This is a model struct for UpdateCustomerCustomAttributeDefinitionRequest type
#[derive(Clone, Debug, Serialize, Eq, PartialEq)]
pub struct UpdateCustomerCustomAttributeDefinitionRequest {
    /// The custom attribute definition that contains the fields to update.
    /// This endpoint supports sparse updates, so only new or changed fields need to be included in the request.
    /// Only the following fields can be updated:
    /// - `name`
    /// - `description`
    /// - `visibility`
    /// - `schema` for a Selection data type (only named options or maximum allowed selections).
    pub custom_attribute_definition: CustomAttributeDefinition,
    /// A unique identifier for this request, used to ensure idempotency.
    /// Max length: 45 characters.
    pub idempotency_key: String,
}
