//! Response struct for the Update webhook subscription API

use serde::Deserialize;

use super::{WebhookSubscription, errors::Error};

/// This is a model struct for UpdateWebhookSubscriptionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct UpdateWebhookSubscriptionResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The updated WebhookSubscription.
    pub subscription: Option<WebhookSubscription>,
}
