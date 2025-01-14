//! Model struct for ListCustomerCustomAttributesResponse type

use super::{errors::Error, CustomAttribute};
use serde::Deserialize;

/// This is a model struct for ListCustomerCustomAttributesResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListCustomerCustomAttributesResponse {
    /// The retrieved custom attributes.
    /// If `with_definitions` was set to `true` in the request, the custom attribute definition
    /// is returned in the `definition` field of each custom attribute.
    /// If no custom attributes are found, Square returns an empty object (`{}`).
    pub custom_attributes: Option<Vec<CustomAttribute>>,
    /// The cursor to use in your next call to this endpoint to retrieve the next page of results
    /// for your original request. This field is present only if additional results are available.
    pub cursor: Option<String>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
