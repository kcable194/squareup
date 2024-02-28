//! Response body struct for the SubscriptionEventData type

use crate::models::SubscriptionEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for SubscriptionEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct SubscriptionEventData {
    /// The type of the event data object. The value is "subscription". Max Length 50
    pub r#type: String,
    /// The ID of the associated subscription. Max Length 255
    pub id: String,
    /// An object containing the subscription associated with the event.
    pub object: SubscriptionEventObject,
}
