//! Model struct for ListCustomerCustomAttributeDefinitionsResponse type

use super::{CustomAttributeDefinition, errors::Error};
use serde::Deserialize;

/// This is a model struct for ListCustomerCustomAttributeDefinitionsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListCustomerCustomAttributeDefinitionsResponse {
    /// The retrieved custom attribute definitions. If no custom attribute definitions are found,
    /// Square returns an empty object ({}).
    pub custom_attribute_definitions: Option<Vec<CustomAttributeDefinition>>,
    /// The cursor to provide in your next call to this endpoint to retrieve the next page of
    /// results for your original request. This field is present only if additional results are available.
    pub cursor: Option<String>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
