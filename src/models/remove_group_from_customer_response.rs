//! Model struct for RemoveGroupFromCustomerResponse type

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for RemoveGroupFromCustomerResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RemoveGroupFromCustomerResponse {
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
