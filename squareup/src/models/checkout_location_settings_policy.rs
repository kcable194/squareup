//! Model struct for CheckoutLocationSettingsPolicy type

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CheckoutLocationSettingsPolicy {
    /// A unique ID to identify the policy when making changes. You must set the UID for policy
    /// updates, but it’s optional when setting new policies.
    pub uid: Option<String>,
    /// The title of the policy. This is required when setting the description, though you can
    /// update it in a different request.
    /// Max Length 50
    pub title: Option<String>,
    /// The description of the policy.
    /// Max Length 4096
    pub description: Option<String>,
}
