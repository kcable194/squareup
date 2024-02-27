//! Model struct for BulkDeleteCustomersResponse type

use serde::Deserialize;
use std::collections::HashMap;

use super::errors::Error;
use super::DeleteCustomerResponse;

/// This is a model struct for BulkDeleteCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkDeleteCustomersResponse {
    /// A map of responses that correspond to individual delete requests, represented by key-value
    /// pairs.
    ///
    /// Each key is the customer ID that was specified for a delete request and each value is the
    /// corresponding response. If the request succeeds, the value is an empty object ({ }). If
    /// the request fails, the value contains any errors that occurred during the request.
    pub responses: Option<HashMap<String, DeleteCustomerResponse>>,
    /// Any top-level errors that prevented the bulk operation from running.
    pub errors: Option<Vec<Error>>,
}
