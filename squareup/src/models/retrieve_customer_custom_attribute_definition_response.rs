//! Model struct for RetrieveCustomerCustomAttributeDefinitionResponse type

use super::{errors::Error, CustomAttributeDefinition};
use serde::Deserialize;

/// This is a model struct for RetrieveCustomerCustomAttributeDefinitionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveCustomerCustomAttributeDefinitionResponse {
    /// The retrieved custom attribute definition.
    pub custom_attribute_definition: Option<CustomAttributeDefinition>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
