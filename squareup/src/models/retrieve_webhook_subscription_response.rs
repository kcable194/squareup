//! Response struct for the Retrieve webhook subscription API

use serde::Deserialize;

use super::{WebhookSubscription, errors::Error};

/// This is a model struct for RetrieveWebhookSubscriptionResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RetrieveWebhookSubscriptionResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested Subscription.
    pub subscription: Option<WebhookSubscription>,
}
