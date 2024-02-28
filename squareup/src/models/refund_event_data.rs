//! Response body struct for the RefundEventData type

use crate::models::RefundEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for RefundEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct RefundEventData {
    /// The type of the event data object. The value is "refund". Max Length 50
    pub r#type: String,
    /// The ID of the event refund object. Max Length 255
    pub id: String,
    /// An object containing the refund associated with the event.
    pub object: RefundEventObject,
}
