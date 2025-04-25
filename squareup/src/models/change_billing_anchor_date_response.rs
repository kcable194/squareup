//! Response body struct for the Change billing anchor date API

use serde::Deserialize;

use super::{Subscription, SubscriptionAction, errors::Error};

/// This is a model struct for the ChangeBillingAnchorDateResponse type.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct ChangeBillingAnchorDateResponse {
    /// Errors encountered during the request.
    pub errors: Option<Vec<Error>>,
    /// The specified subscription for updating billing anchor date.
    pub subscription: Option<Subscription>,
    /// A list of a single billing anchor date change for the subscription.
    pub actions: Option<Vec<SubscriptionAction>>,
}
