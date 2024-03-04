//! Request body struct for the Update webhook subscription signature key API

use serde::Serialize;

/// This is a model struct for UpdateWebhookSubscriptionSignatureKeyRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateWebhookSubscriptionSignatureKeyRequest {
    /// A unique string that identifies the CreateWebhookSubscription request.
    ///
    /// Max Length: 45
    pub idempotency_key: Option<String>,
}
