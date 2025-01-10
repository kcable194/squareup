//! Model struct for UpsertCustomerCustomAttributeResponse type

use super::{errors::Error, CustomAttribute};
use serde::Deserialize;

/// Represents the response for the UpsertCustomerCustomAttribute endpoint.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpsertCustomerCustomAttributeResponse {
    /// The new or updated custom attribute.
    pub custom_attribute: Option<CustomAttribute>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
