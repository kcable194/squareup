//! Response struct for the Update webhook subscription signature key API

use serde::Deserialize;

use super::errors::Error;

/// This is a model struct for UpdateWebhookSubscriptionSignatureKeyResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateWebhookSubscriptionSignatureKeyResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// **Read only** The new Square-generated signature key used to validate the origin of the
    /// webhook event.
    pub signature_key: Option<String>,
}
