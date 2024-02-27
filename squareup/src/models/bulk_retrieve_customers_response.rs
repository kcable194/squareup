//! Model struct for BulkRetrieveCustomersResponse type

use serde::Deserialize;
use std::collections::HashMap;

use super::errors::Error;
use super::RetrieveCustomerResponse;

/// This is a model struct for BulkRetrieveCustomersResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct BulkRetrieveCustomersResponse {
    /// A map of responses that correspond to individual retrieve requests, represented by
    /// key-value pairs.
    ///
    /// Each key is the customer ID that was specified for a retrieve request and each value is
    /// the corresponding response. If the request succeeds, the value is the requested customer
    /// profile. If the request fails, the value contains any errors that occurred during the request.
    pub responses: Option<HashMap<String, RetrieveCustomerResponse>>,
    /// Any top-level errors that prevented the bulk operation from running.
    pub errors: Option<Vec<Error>>,
}
