//! Model struct for DeleteCustomerCustomAttributeDefinitionResponse type

use super::errors::Error;
use serde::Deserialize;

/// This is a model struct for DeleteCustomerCustomAttributeDefinitionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DeleteCustomerCustomAttributeDefinitionResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
