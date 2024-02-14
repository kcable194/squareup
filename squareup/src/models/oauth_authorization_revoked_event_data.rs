//! Response body struct for the OauthAuthorizationRevokedEventData type

use crate::models::oauth_authorization_revoked_event_object::OauthAuthorizationRevokedEventObject;
use serde::{Deserialize, Serialize};

/// This is a model struct for OauthAuthorizationRevokedEventData type.
#[derive(Clone, Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
pub struct OauthAuthorizationRevokedEventData {
    /// The type of the event data object. The value is "revocation". Max Length 50
    pub r#type: String,
    /// Not applicable, revocation is not an object
    pub id: Option<String>,
    /// An object containing information about revocation event.
    pub object: Option<OauthAuthorizationRevokedEventObject>,
}
