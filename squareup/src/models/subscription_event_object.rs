//! Response body struct for the SubscriptionEventObject type

use crate::models::Subscription;
use serde::{Deserialize, Serialize};

/// This is a model struct for SubscriptionEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct SubscriptionEventObject {
    /// The subscription associated with the event.
    pub subscription: Subscription,
}
