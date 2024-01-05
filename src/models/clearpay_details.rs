//! Model struct for ClearpayDetails type.

use serde::{Deserialize, Serialize};

/// Details about a Clearpay payment. These details are only populated if the brand is CLEARPAY.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ClearpayDetails {
    /// Email address on the buyer's Clearpay account.
    /// Max Length 255
    pub email_address: Option<String>,
}