//! Model struct for RetrieveCustomerCustomAttributeResponse type

use super::{errors::Error, CustomAttribute};
use serde::Deserialize;

/// This is a model struct for RetrieveCustomerCustomAttributeResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveCustomerCustomAttributeResponse {
    /// The retrieved custom attribute. If `with_definition` was set to `true` in the request,
    /// the custom attribute definition is returned in the `definition` field.
    pub custom_attribute: Option<CustomAttribute>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
