//! Model for GrantType enum.

use serde::{Deserialize, Serialize};

/// An enum type of grant type.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "migration_token")]
    MigrationToken,
}
