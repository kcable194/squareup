//! Model struct for BulkUpsertCustomerCustomAttributesResponse type

use super::BulkUpsertCustomerCustomAttributesResponseCustomerCustomAttributeUpsertResponse;
use crate::models::errors::Error;
use serde::{Deserialize, Serialize};

/// Represents a response for the BulkUpsertCustomerCustomAttributes endpoint.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct BulkUpsertCustomerCustomAttributesResponse {
    /// A list of responses that correspond to individual upsert requests.
    pub values:
        Vec<BulkUpsertCustomerCustomAttributesResponseCustomerCustomAttributeUpsertResponse>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
