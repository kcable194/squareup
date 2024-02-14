//! Response body struct for the OauthAuthorizationRevokedEventObject type

use crate::models::oauth_authorization_revoked_event_revocation_object::OauthAuthorizationRevokedEventRevocationObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for OauthAuthorizationRevokedEventObject type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct OauthAuthorizationRevokedEventObject {
    /// The revocation event.
    pub revocation: Option<OauthAuthorizationRevokedEventRevocationObject>,
}
