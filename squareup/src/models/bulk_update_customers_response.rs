//! Model struct for BulkUpdateCustomersResponse type

use serde::Deserialize;
use std::collections::HashMap;

use super::UpdateCustomerResponse;
use super::errors::Error;

/// This is a model struct for BulkUpdateCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkUpdateCustomersResponse {
    /// A map of responses that correspond to individual update requests, represented by key-value
    /// pairs.
    ///
    /// Each key is the customer ID that was specified for an update request and each value is the
    /// corresponding response. If the request succeeds, the value is the updated customer profile.
    /// If the request fails, the value contains any errors that occurred during the request.
    pub responses: Option<HashMap<String, UpdateCustomerResponse>>,
    /// Any top-level errors that prevented the bulk operation from running.
    pub errors: Option<Vec<Error>>,
}
