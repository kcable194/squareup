//! Model struct for BulkUpsertCustomerCustomAttributesResponse type

use std::collections::HashMap;

use super::BulkUpsertCustomerCustomAttributesResponseCustomerCustomAttributeUpsertResponse;
use crate::models::errors::Error;
use serde::{Deserialize, Serialize};

/// Represents a response for the BulkUpsertCustomerCustomAttributes endpoint.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct BulkUpsertCustomerCustomAttributesResponse {
    /// A list of responses that correspond to individual upsert requests.
    pub values:
        HashMap<String, BulkUpsertCustomerCustomAttributesResponseCustomerCustomAttributeUpsertResponse>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
