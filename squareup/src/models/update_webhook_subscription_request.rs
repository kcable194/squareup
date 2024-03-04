//! Request body struct for the Update webhook subscription API

use serde::Serialize;

use super::WebhookSubscription;

/// This is a model struct for UpdateWebhookSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct UpdateWebhookSubscriptionRequest {
    /// The subscription object to update.
    pub subscription: Option<WebhookSubscription>,
}
