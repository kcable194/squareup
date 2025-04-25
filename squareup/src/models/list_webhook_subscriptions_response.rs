//! Response struct for the list webhook subscriptions API

use serde::Deserialize;

use super::{WebhookSubscription, errors::Error};

/// This is a model struct for ListWebhookSubscriptionsResponse type
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListWebhookSubscriptionsResponse {
    /// Information about errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The requested list of Subscriptions.
    pub subscriptions: Option<Vec<WebhookSubscription>>,
    /// The pagination cursor to be used in a subsequent request. If empty, this is the
    /// final response.
    ///
    /// For more information, see Pagination.
    pub cursor: Option<String>,
}
