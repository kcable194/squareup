//! Model struct for AddGroupToCustomerResponse type

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for AddGroupToCustomerResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct AddGroupToCustomerResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
