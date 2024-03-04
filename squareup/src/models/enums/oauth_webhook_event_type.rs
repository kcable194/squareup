//! Enum for OauthWebhookEventType type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of oauth event coming from the webhook
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OauthWebhookEventType {
    #[serde(rename = "oauth.authorization.revoked")]
    OauthAuthorizationRevoked,
}

impl Display for OauthWebhookEventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OauthWebhookEventType::OauthAuthorizationRevoked => {
                write!(f, "oauth.authorization.revoked")
            }
        }
    }
}
