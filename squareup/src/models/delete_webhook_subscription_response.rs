//! Response struct for the Delete webhook subscription API

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for DeleteWebhookSubscriptionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct DeleteWebhookSubscriptionResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
}
