//! Response struct for the Create webhook subscription API

use serde::Deserialize;

use super::{errors::Error, WebhookSubscription};

/// This is a model struct for CreateWebhookSubscriptionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CreateWebhookSubscriptionResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The new WebhookSubscription.
    pub subscription: Option<WebhookSubscription>,
}
