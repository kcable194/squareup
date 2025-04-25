//! Model struct for BulkCreateCustomersResponse type

use serde::Deserialize;
use std::collections::HashMap;

use super::CreateCustomerResponse;
use super::errors::Error;

/// This is a model struct for BulkCreateCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkCreateCustomersResponse {
    /// A map of responses that correspond to individual create requests, represented by key-value
    /// pairs.
    ///
    /// Each key is the idempotency key that was provided for a create request and each value is
    /// the corresponding response. If the request succeeds, the value is the new customer profile.
    /// If the request fails, the value contains any errors that occurred during the request.
    pub responses: Option<HashMap<String, CreateCustomerResponse>>,
    /// Any top-level errors that prevented the bulk operation from running.
    pub errors: Option<Vec<Error>>,
}
