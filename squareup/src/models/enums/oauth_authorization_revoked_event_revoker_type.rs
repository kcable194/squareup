//! Model for OauthAuthorizationRevokedEventRevokerType enum

use serde::{Deserialize, Serialize};

/// The type of revoked event type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OauthAuthorizationRevokedEventRevokerType {
    /// The application that requested access to a merchant's data.
    Application,
    /// The admin for the merchant.
    Merchant,
    /// An internal Square employee.
    Square,
}
