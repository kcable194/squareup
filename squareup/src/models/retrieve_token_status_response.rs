//! Model struct for RetrieveTokenStatusResponse type

use super::errors::Error;
use crate::models::DateTime;
use crate::models::enums::OAuthPermission;
use serde::Deserialize;

/// This is a model struct for RetrieveTokenStatusResponse type
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct RetrieveTokenStatusResponse {
    /// The list of scopes associated with an access token.
    pub scopes: Option<Vec<OAuthPermission>>,
    /// The date and time when the access_token expires, in RFC 3339 format. Empty if the token
    /// never expires.
    pub expires_at: Option<DateTime>,
    /// The Square-issued application ID associated with the access token. This is the same
    /// application ID used to obtain the token.
    /// Max Length 191
    pub client_id: Option<String>,
    /// The ID of the authorizing merchant's business.
    /// Min Length 8, Max Length 191
    pub merchant_id: Option<String>,
    /// Any errors that occurred during the request.
    pub errors: Option<Vec<Error>>,
}
