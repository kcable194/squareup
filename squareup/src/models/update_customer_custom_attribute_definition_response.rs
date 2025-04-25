//! Model struct for UpdateCustomerCustomAttributeDefinitionResponse type

use super::{CustomAttributeDefinition, errors::Error};
use serde::Deserialize;

/// This is a model struct for UpdateCustomerCustomAttributeDefinitionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateCustomerCustomAttributeDefinitionResponse {
    /// The updated custom attribute definition.
    pub custom_attribute_definition: Option<CustomAttributeDefinition>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
