//! Response body struct for the CustomerEventObject type

use crate::models::Customer;
use serde::{Deserialize, Serialize};

/// This is a model struct for CustomerEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct CustomerEventObject {
    /// The customer associated with the event.
    pub customer: Customer,
}
