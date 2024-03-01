//! Request body struct for the Create webhook subscription API

use serde::Serialize;

use super::WebhookSubscription;

/// This is a model struct for CreateWebhookSubscriptionRequest type.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CreateWebhookSubscriptionRequest {
    /// A unique string that identifies the CreateWebhookSubscription request.
    ///
    /// Max Length: 45
    pub idempotency_key: Option<String>,
    /// The subscription object to create.
    pub subscription: WebhookSubscription,
}
