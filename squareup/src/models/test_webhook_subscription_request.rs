//! Request body struct for the Test webhook subscription API

use crate::models::enums::WebhookEventType;
use serde::Serialize;

/// This is a model struct for TestWebhookSubscriptionRequest type.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct TestWebhookSubscriptionRequest {
    /// The event type that will be used to test the WebhookSubscription. The event type must be
    /// contained in the list of event types in the WebhookSubscription.
    pub event_type: Option<WebhookEventType>,
}
