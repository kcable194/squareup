//! Model struct for DeleteCustomerCustomAttributeResponse type

use super::errors::Error;
use serde::Deserialize;

/// Represents the response for deleting a customer custom attribute.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DeleteCustomerCustomAttributeResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
